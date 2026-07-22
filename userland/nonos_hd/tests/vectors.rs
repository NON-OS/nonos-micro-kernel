// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Every derivation primitive proven against published vectors: the complete
//! official BIP39 set (trezor/python-mnemonic vectors.json, embedded), the
//! three BIP32 test vectors from the BIP text, FIPS/RFC hash and HMAC known
//! answers, and the standard Ethereum path end to end. The secp256k1 public
//! keys the non-hardened steps need come from the audited k256 crate here;
//! inside the capsule the same role is played by the kernel syscall.

use nonos_hd::bip32::{
    add_mod_n, child_hardened, child_normal, compress_pubkey, is_valid_scalar, master_from_seed,
    Xprv, ORDER,
};
use nonos_hd::bip39::{entropy_to_words, seed_from_words, word_index, words_to_entropy, MAX_WORDS};
use nonos_hd::{derive_eth_key, hmac_sha512, sha256, sha512, ENGLISH_WORDLIST};

use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::elliptic_curve::PrimeField;
use k256::{ProjectivePoint, Scalar};
use sha3::{Digest, Keccak256};

fn pubkey65(sk: &[u8; 32]) -> Option<[u8; 65]> {
    let scalar = Option::<Scalar>::from(Scalar::from_repr((*sk).into()))?;
    let point = (ProjectivePoint::GENERATOR * scalar).to_affine();
    let encoded = point.to_encoded_point(false);
    if encoded.as_bytes().len() != 65 {
        return None; // the identity encodes as a single byte
    }
    let mut out = [0u8; 65];
    out.copy_from_slice(encoded.as_bytes());
    Some(out)
}

/// Decode a Base58Check xprv into (chain_code, private_key), verifying the
/// double-SHA256 checksum via bs58. Layout: version 4, depth 1, parent
/// fingerprint 4, child number 4, chain code 32, 0x00, key 32.
fn decode_xprv(xprv: &str) -> ([u8; 32], [u8; 32]) {
    let payload = bs58::decode(xprv).with_check(None).into_vec().expect("xprv checksum");
    assert_eq!(payload.len(), 78, "xprv payload length");
    assert_eq!(payload[45], 0x00, "private key marker");
    let mut chain = [0u8; 32];
    let mut key = [0u8; 32];
    chain.copy_from_slice(&payload[13..45]);
    key.copy_from_slice(&payload[46..78]);
    (chain, key)
}

fn phrase_to_indices(phrase: &str) -> Vec<u16> {
    phrase
        .split_whitespace()
        .map(|w| word_index(w.as_bytes()).unwrap_or_else(|| panic!("word {w} not in list")))
        .collect()
}

#[test]
fn wordlist_shape() {
    assert_eq!(ENGLISH_WORDLIST.len(), 2048);
    assert_eq!(ENGLISH_WORDLIST[0], "abandon");
    assert_eq!(ENGLISH_WORDLIST[2047], "zoo");
    // Sorted order is what the binary-search lookup depends on.
    for pair in ENGLISH_WORDLIST.windows(2) {
        assert!(pair[0] < pair[1], "wordlist must be strictly sorted: {} {}", pair[0], pair[1]);
    }
}

#[test]
fn hash_known_answers() {
    assert_eq!(
        hex::encode(sha256(b"")),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        hex::encode(sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        hex::encode(sha512(b"abc")),
        "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
         2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
    );
    // RFC 4231 test case 1.
    assert_eq!(
        hex::encode(hmac_sha512(&[0x0b; 20], b"Hi There")),
        "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
         daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
    );
    // RFC 4231 test case 2 exercises a key shorter than the block.
    assert_eq!(
        hex::encode(hmac_sha512(b"Jefe", b"what do ya want for nothing?")),
        "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea250554\
         9758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"
    );
}

#[test]
fn bip39_official_vectors() {
    let raw = include_str!("data/bip39_vectors.json");
    let json: serde_json::Value = serde_json::from_str(raw).expect("vectors.json parses");
    let english = json["english"].as_array().expect("english array");
    assert!(english.len() >= 24, "expected the full published set");

    for vector in english {
        let entropy = hex::decode(vector[0].as_str().unwrap()).unwrap();
        let phrase = vector[1].as_str().unwrap();
        let expected_seed = hex::decode(vector[2].as_str().unwrap()).unwrap();
        let expected_xprv = vector[3].as_str().unwrap();

        // entropy -> words matches the published phrase exactly.
        let mut words = [0u16; MAX_WORDS];
        let count = entropy_to_words(&entropy, &mut words).expect("valid entropy length");
        let expected_indices = phrase_to_indices(phrase);
        assert_eq!(&words[..count], expected_indices.as_slice(), "phrase for {phrase}");

        // words -> entropy round-trips with a valid checksum.
        let (back, len) = words_to_entropy(&words[..count]).expect("checksum accepts");
        assert_eq!(&back[..len], entropy.as_slice());

        // phrase + TREZOR passphrase -> the published PBKDF2 seed.
        let mut seed = [0u8; 64];
        assert!(seed_from_words(&words[..count], b"TREZOR", &mut seed));
        assert_eq!(seed.as_slice(), expected_seed.as_slice(), "seed for {phrase}");

        // seed -> the published BIP32 root key.
        let (chain, key) = decode_xprv(expected_xprv);
        let master = master_from_seed(&seed).expect("master derives");
        assert_eq!(master.chain, chain, "chain code for {phrase}");
        assert_eq!(master.key, key, "master key for {phrase}");
    }
}

enum Step {
    H(u32),
    N(u32),
}

fn check_bip32_chain(seed_hex: &str, steps: &[(Step, &str)], root: &str) {
    let seed = hex::decode(seed_hex).unwrap();
    let mut node = master_from_seed(&seed).expect("master");
    let (chain, key) = decode_xprv(root);
    assert_eq!(node.chain, chain);
    assert_eq!(node.key, key);

    for (step, expected) in steps {
        node = match step {
            Step::H(i) => child_hardened(&node, *i).expect("hardened child"),
            Step::N(i) => {
                let pubkey = pubkey65(&node.key).expect("pubkey");
                let compressed = compress_pubkey(&pubkey).expect("compress");
                child_normal(&node, &compressed, *i).expect("normal child")
            }
        };
        let (chain, key) = decode_xprv(expected);
        assert_eq!(node.chain, chain);
        assert_eq!(node.key, key);
    }
}

#[test]
fn bip32_vector_1() {
    check_bip32_chain(
        "000102030405060708090a0b0c0d0e0f",
        &[
            (Step::H(0), "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7"),
            (Step::N(1), "xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs"),
            (Step::H(2), "xprv9z4pot5VBttmtdRTWfWQmoH1taj2axGVzFqSb8C9xaxKymcFzXBDptWmT7FwuEzG3ryjH4ktypQSAewRiNMjANTtpgP4mLTj34bhnZX7UiM"),
            (Step::N(2), "xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334"),
            (Step::N(1000000000), "xprvA41z7zogVVwxVSgdKUHDy1SKmdb533PjDz7J6N6mV6uS3ze1ai8FHa8kmHScGpWmj4WggLyQjgPie1rFSruoUihUZREPSL39UNdE3BBDu76"),
        ],
        "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi",
    );
}

#[test]
fn bip32_vector_2() {
    check_bip32_chain(
        "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a2\
         9f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542",
        &[
            (Step::N(0), "xprv9vHkqa6EV4sPZHYqZznhT2NPtPCjKuDKGY38FBWLvgaDx45zo9WQRUT3dKYnjwih2yJD9mkrocEZXo1ex8G81dwSM1fwqWpWkeS3v86pgKt"),
            (Step::H(2147483647), "xprv9wSp6B7kry3Vj9m1zSnLvN3xH8RdsPP1Mh7fAaR7aRLcQMKTR2vidYEeEg2mUCTAwCd6vnxVrcjfy2kRgVsFawNzmjuHc2YmYRmagcEPdU9"),
            (Step::N(1), "xprv9zFnWC6h2cLgpmSA46vutJzBcfJ8yaJGg8cX1e5StJh45BBciYTRXSd25UEPVuesF9yog62tGAQtHjXajPPdbRCHuWS6T8XA2ECKADdw4Ef"),
            (Step::H(2147483646), "xprvA1RpRA33e1JQ7ifknakTFpgNXPmW2YvmhqLQYMmrj4xJXXWYpDPS3xz7iAxn8L39njGVyuoseXzU6rcxFLJ8HFsTjSyQbLYnMpCqE2VbFWc"),
            (Step::N(2), "xprvA2nrNbFZABcdryreWet9Ea4LvTJcGsqrMzxHx98MMrotbir7yrKCEXw7nadnHM8Dq38EGfSh6dqA9QWTyefMLEcBYJUuekgW4BYPJcr9E7j"),
        ],
        "xprv9s21ZrQH143K31xYSDQpPDxsXRTUcvj2iNHm5NUtrGiGG5e2DtALGdso3pGz6ssrdK4PFmM8NSpSBHNqPqm55Qn3LqFtT2emdEXVYsCzC2U",
    );
}

#[test]
fn bip32_vector_3_leading_zeros() {
    check_bip32_chain(
        "4b381541583be4423346c643850da4b320e46a87ae3d2a4e6da11eba819cd4ac\
         ba45d239319ac14f863b8d5ab5a0d0c64d2e8a1e7d1457df2e5a3c51c73235be",
        &[(Step::H(0), "xprv9uPDJpEQgRQfDcW7BkF7eTya6RPxXeJCqCJGHuCJ4GiRVLzkTXBAJMu2qaMWPrS7AANYqdq6vcBcBUdJCVVFceUvJFjaPdGZ2y9WACViL4L")],
        "xprv9s21ZrQH143K25QhxbucbDDuQ4naNntJRi4KUfWT7xo4EKsHt2QJDu7KXp1A3u7Bi1j8ph3EGsZ9Xvz9dGuVrtHHs7pXeTzjuxBrCmmhgC6",
    );
}

#[test]
fn eth_account_path_end_to_end() {
    // The canonical development mnemonic. Its m/44'/60'/0'/0/0 key and
    // address are published in countless wallet test suites; private key and
    // address are asserted independently, tied together through k256+keccak.
    let phrase =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let indices = phrase_to_indices(phrase);
    let mut seed = [0u8; 64];
    assert!(seed_from_words(&indices, b"", &mut seed));

    let mut key = [0u8; 32];
    assert!(derive_eth_key(&seed, pubkey65, &mut key));
    assert_eq!(
        hex::encode(key),
        "1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727"
    );

    let pubkey = pubkey65(&key).expect("pubkey");
    let digest = Keccak256::digest(&pubkey[1..]);
    let address = &digest[12..];
    assert_eq!(hex::encode(address), "9858effd232b4033e47d90003d41ec34ecaeda94");
}

#[test]
fn recovery_is_deterministic_and_checksum_gated() {
    let phrase =
        "legal winner thank year wave sausage worth useful legal winner thank yellow";
    let indices = phrase_to_indices(phrase);

    // Same phrase twice -> byte-identical seeds.
    let mut seed_a = [0u8; 64];
    let mut seed_b = [0u8; 64];
    assert!(seed_from_words(&indices, b"", &mut seed_a));
    assert!(seed_from_words(&indices, b"", &mut seed_b));
    assert_eq!(seed_a, seed_b);

    // Any single-word substitution that still parses fails the checksum.
    let mut wrong = indices.clone();
    wrong[11] = word_index(b"yard").expect("valid word");
    assert!(words_to_entropy(&wrong).is_none(), "wrong last word must be rejected");

    // A word outside the list never resolves at all.
    assert!(word_index(b"metamask").is_none());

    // A different valid phrase yields a different seed.
    let other = phrase_to_indices(
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
    );
    let mut seed_c = [0u8; 64];
    assert!(seed_from_words(&other, b"", &mut seed_c));
    assert_ne!(seed_a, seed_c);
}

#[test]
fn scalar_edges() {
    assert!(!is_valid_scalar(&[0u8; 32]), "zero is not a key");
    assert!(!is_valid_scalar(&ORDER), "the group order is not a key");
    let mut below = ORDER;
    below[31] -= 1;
    assert!(is_valid_scalar(&below), "n-1 is the largest valid scalar");

    // a + (n - a) reduces to exactly zero, which the validity gate rejects.
    let mut a = [0u8; 32];
    a[31] = 5;
    let mut n_minus_a = ORDER;
    n_minus_a[31] -= 5;
    let sum = add_mod_n(&a, &n_minus_a);
    assert_eq!(sum, [0u8; 32]);
    assert!(!is_valid_scalar(&sum));

    // One over the group order wraps to one.
    let one = {
        let mut v = [0u8; 32];
        v[31] = 1;
        v
    };
    let sum = add_mod_n(&ORDER, &one);
    assert_eq!(sum, one);
}

#[test]
fn hardened_index_bounds() {
    let parent = Xprv { key: [7u8; 32], chain: [9u8; 32] };
    // Hardened indices at or past 2^31 would collide with the offset space.
    assert!(child_hardened(&parent, 0x8000_0000).is_none());
    // Normal derivation refuses indices in the hardened range.
    let pubkey = pubkey65(&parent.key).expect("pubkey");
    let compressed = compress_pubkey(&pubkey).expect("compress");
    assert!(child_normal(&parent, &compressed, 0x8000_0000).is_none());
    assert!(child_normal(&parent, &compressed, 1).is_some());
}
