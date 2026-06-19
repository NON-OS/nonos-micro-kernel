# Import map

Every donor file present under `third_party/redox/src` has exactly one row here.
Driver files come from `https://github.com/redox-os/drivers` at commit
`20ffe4d7f4a85b7cc1f59495d7e6e355fed4cb06`. RedoxFS files come from
`https://gitlab.redox-os.org/redox-os/redoxfs` at commit
`af493b9f4e1ee7086bc6e44a43f096d981936cfe`. Both are MIT. SHA256 is of the copied
file as vendored. Status: `vendored`, `translating`, `translated`, or `deleted`.

| donor path | sha256 | nonos target | status |
|-----------|--------|--------------|--------|
| drivers/LICENSE | cb46b697c3fd9d27d7bfe1b1ad48f8a58a284984504c6eb215ae2164538df7cb | third_party/redox driver license provenance | vendored |
| drivers/storage/ahcid/src/ahci/hba.rs | e9cf6d4ec08617ccec3a88f6256d5fe8cc1c7df6f1b57f17e383133c45a7e61c | userland/capsule_driver_ahci/src/engine (cmd_header, cmd_table, prdt, port, init, stop, start, program, issue, recover) | translated |
| drivers/storage/ahcid/src/ahci/fis.rs | 43c03da46071087507d9f17f265c6855d3735e2de21621b3e9cabb958c8eaceb | userland/capsule_driver_ahci/src/engine/fis.rs | translated |
| drivers/storage/ahcid/src/ahci/disk_ata.rs | c851ef14d306ea37459cd7cf89ad487ee0f080d1962031c112807a16222101ec | userland/capsule_driver_ahci/src/engine (identify, transfer, flush, build, prdt_write) | translated |
| drivers/storage/ahcid/src/ahci/mod.rs | 6f35dc0b0b83f0de413075c0094953aec5bd86ac7ae11e6c32aa5fa37f843b81 | userland/capsule_driver_ahci/src/setup/block_port.rs (port probe and bring-up) | translated |
| drivers/graphics/bgad/src/bga.rs | 840e5330e69597e317240128d810a10d61b10fd4e5979b6ac411395bcb940304 | userland/capsule_driver_bga/src/dispi (VBE DISPI modeset) | translating |
| drivers/graphics/bgad/src/main.rs | 93c7d98b8fd8dfb26ac213a9b440588d5acabeca0f86aec9d779921d36961639 | userland/capsule_driver_bga/src/setup (PCI bring-up + BAR map) | translating |
| redoxfs/LICENSE | 93feb465f692bf21fe310c2d7d2e906982950222897304ddddbcdab873cd11b8 | third_party/redox license provenance | vendored |
| redoxfs/Cargo.toml | 6c6b998c79d39d728e1522d6e0c41f563787249a64cd7af5864d19dbd7228924 | third_party/redox crate provenance | vendored |
| redoxfs/README.md | 5da7a9eef5739fa8a9a001de510c4bfa9dfdbacb4e9664fbb0b993d47bffa23a | third_party/redox filesystem provenance | vendored |
| redoxfs/src/block.rs | e63b7fd6dfa883e62787974378492d408340f8e82d44a3fb9af165e07680641e | src/fs/blockfs block/header-ring translation | translating |
| redoxfs/src/filesystem.rs | 586b443e8e38181c6ef49fa21eee64baae02b29f5b581eb5f0faefafef174373 | src/fs/blockfs mount/format translation | translating |
| redoxfs/src/header.rs | 39c3cd0a953ab59fce7b1dfdd2b2eeb87d533ac33d7978b3c4f7138fec1920ad | src/fs/blockfs superblock translation | translating |
| redoxfs/src/htree.rs | 43b997c6a55fb45178a875bb02f638ea445076c888b3a1b408003f06d42b466f | src/fs/blockfs directory index translation | vendored |
| redoxfs/src/key.rs | d079f2421fbf9336e66037a7896dc10f099ccb14f65713eab881df8de30feac9 | src/fs/blockfs keyslot policy translation | vendored |
| redoxfs/src/node.rs | 99961381e9b5709fb6f247dc685a4e08853871272555e78e90098ab388c67c82 | src/fs/blockfs inode translation | vendored |
| redoxfs/src/record.rs | c1db3df5e1b52e3fdc5f25b52620978025b6f0c2069dc476d1e94069b13b88c3 | src/fs/blockfs file record translation | vendored |
| redoxfs/src/transaction.rs | 229b64804937a976ea1b4bf2780b6155b81f50a024a6de597c173f3fadd7c4d6 | src/fs/blockfs transaction translation | vendored |
| redoxfs/src/tree.rs | e586b095f6e9d526d9e12201b4ce14071030c60a1c3c058a91525f0e600222ea | src/fs/blockfs allocator/tree translation | vendored |
