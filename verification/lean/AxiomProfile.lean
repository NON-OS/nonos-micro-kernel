/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The axiom profile of the specification layer, checked by Lean itself rather
than asserted. `#print axioms` prints the exact axiom closure of a theorem:
for every flagship theorem below the expected output names at most Lean's
three standard axioms (propext, Classical.choice, Quot.sound) and never
`sorryAx`, which is what a `sorry` would introduce. The CI lean job runs this
file and publishes the output as evidence, so a green run leaves a record of
what was proven and on what foundations, not just an exit code.
-/

import Nonos

#print axioms Nonos.AntiRollback.no_rollback_after_boot
#print axioms Nonos.AntiRollbackState.refines_abstract
#print axioms Nonos.AntiRollbackState.no_rollback_after_update
#print axioms Nonos.AntiRollbackState.update_floor_monotone
#print axioms Nonos.Authorization.empty_token_denied
#print axioms Nonos.BlockIO.accepted_request_stays_on_disk
#print axioms Nonos.BootImage.accepted_region_stays_in_bounds
#print axioms Nonos.Capability.attenuate_confines
#print axioms Nonos.CapabilityBits.word_chain_never_widens
#print axioms Nonos.Crypto.wrong_tag_rejected

-- Wallet userland and signing crypto: live NOX/staking reads, EIP-1559 fee
-- sizing, EIP-2 low-s canonicalisation, address derivation, hex parsing, and
-- the private-key import wipe, each proven on Lean's standard axioms alone.
#print axioms Nonos.WalletNoxApr.empty_pool_no_rate
#print axioms Nonos.WalletNoxApr.apr_monotone_in_emission
#print axioms Nonos.WalletNoxCalldata.padding_is_zero
#print axioms Nonos.WalletNoxCalldata.address_placed
#print axioms Nonos.WalletQuantity.high_bytes_refused
#print axioms Nonos.WalletQuantity.decoded_lt_two_pow_128
#print axioms Nonos.WalletEip1559.tip_at_least_one_gwei
#print axioms Nonos.WalletEip1559.cap_covers_tip
#print axioms Nonos.WalletHex.nibble_lt_16
#print axioms Nonos.WalletFormatNox.whole_reconstructs
#print axioms Nonos.WalletFormatApr.reconstructs
#print axioms Nonos.WalletParseWord.words_disjoint
#print axioms Nonos.WalletImportWipe.wiped_all_zero
#print axioms Nonos.CryptoLowS.normalized_is_low
#print axioms Nonos.CryptoLowS.normalize_idempotent
#print axioms Nonos.CryptoKeccakAddr.in_hash_range
#print axioms Nonos.CryptoSecretValid.zero_rejected
#print axioms Nonos.CryptoSecretValid.overflow_rejected
#print axioms Nonos.CryptoSecretValid.valid_iff
#print axioms Nonos.CryptoRfc6979.distinct_nonce_distinct_msg
#print axioms Nonos.WalletRlp.single_low_byte_bare
#print axioms Nonos.WalletRlp.short_prefix_in_range
#print axioms Nonos.WalletGwei.never_overstates
#print axioms Nonos.WalletGwei.monotone
#print axioms Nonos.WalletShortAddr.shown_in_range
#print axioms Nonos.KeyringCustody.access_implies_owner
#print axioms Nonos.KeyringCustody.non_owner_denied
#print axioms Nonos.KeyringCustody.non_owner_indistinguishable
#print axioms Nonos.WalletTxEnvelope.type_prefix_unambiguous
#print axioms Nonos.WalletTxEnvelope.signing_adds_signature
#print axioms Nonos.WalletNonceReplay.replay_refused
#print axioms Nonos.WalletNonceReplay.no_two_nonces
#print axioms Nonos.CryptoGf256.add_self
#print axioms Nonos.CryptoGf256.add_cancel
#print axioms Nonos.CryptoGf256.add_assoc
#print axioms Nonos.CryptoKeccakPad.multiple_of_rate
#print axioms Nonos.CryptoKeccakPad.always_pads

#print axioms Nonos.Ipc.zero_length_rejected
#print axioms Nonos.Isolation.no_wx_page
#print axioms Nonos.Loader.accepted_entry_inside_file
#print axioms Nonos.NetParse.a_pointer_ends_the_walk
#print axioms Nonos.Paging.confined_preserves_no_wx
#print axioms Nonos.Path.leading_dotdot_neutralized
#print axioms Nonos.Secure.every_trace_is_secure
#print axioms Nonos.Secure.dma_owned_by_caller
#print axioms Nonos.Secure.dma_within_class_limit
#print axioms Nonos.Secure.elf_table_in_bounds
#print axioms Nonos.Secure.irq_vector_count_bounded
#print axioms Nonos.Secure.irq_not_already_bound
#print axioms Nonos.Secure.quota_within_cap
#print axioms Nonos.NonInterference.step_preserves_token
#print axioms Nonos.NonInterference.token_locality
#print axioms Nonos.NonInterference.token_noninterference
#print axioms Nonos.NonInterference.no_authority_leak
#print axioms Nonos.NonInterference.step_no_gain
#print axioms Nonos.NonInterference.no_authority_amplification
#print axioms Nonos.NonInterference.step_preserves_mapping
#print axioms Nonos.NonInterference.mapping_locality
#print axioms Nonos.NonInterference.mapping_noninterference
#print axioms Nonos.NonInterference.Unwinding.locality
#print axioms Nonos.NonInterference.Unwinding.noninterference
#print axioms Nonos.NonInterference.admitted_noninterference
#print axioms Nonos.NonInterference.dma_noninterference
#print axioms Nonos.NonInterference.copies_noninterference
#print axioms Nonos.NonInterference.floor_noninterference
#print axioms Nonos.NonInterference.elf_noninterference
#print axioms Nonos.NonInterference.irq_noninterference
#print axioms Nonos.NonInterference.step_preserves_domain_view
#print axioms Nonos.NonInterference.domain_noninterference
#print axioms Nonos.NonInterference.touches_disjoint
#print axioms Nonos.NonInterference.domain_isolation
#print axioms Nonos.NonInterference.disjoint_domains_noninterfere
#print axioms Nonos.AttestBinding.starkAttest_true_iff
#print axioms Nonos.AttestBinding.admitted_is_enrolled
#print axioms Nonos.AttestBinding.admitted_enrolled_after_trace
#print axioms Nonos.AttestBinding.admitted_accepted_after_trace
#print axioms Nonos.AttestBinding.no_cross_policy_replay
#print axioms Nonos.Spawn.only_attested_capsules_run
#print axioms Nonos.Spawn.enforcing_run_admits_only_verified
#print axioms Nonos.Spawn.enforcing_refuses_a_missing_trailer
#print axioms Nonos.Spawn.production_is_always_enforcing
#print axioms Nonos.SpawnCaps.installed_within_ceiling
#print axioms Nonos.SpawnCaps.installed_within_manifest
#print axioms Nonos.SpawnCaps.authority_only_narrows
#print axioms Nonos.Delegation.never_outlives_parent
#print axioms Nonos.Delegation.never_outlasts_request
#print axioms Nonos.Delegation.live_child_implies_live_parent
#print axioms Nonos.Stark.AssociationSet.an_excluded_deposit_cannot_pass
#print axioms Nonos.Stark.AssociationSet.the_registry_only_grows
#print axioms Nonos.Stark.Attest.a_proof_for_one_capsule_is_rejected_for_another
#print axioms Nonos.Stark.Constraint.constraint_holds_iff_quotient_exists
#print axioms Nonos.Stark.Constraint.the_quotient_is_pinned
#print axioms Nonos.Stark.CopyConstraint.wiring_forces_equality
#print axioms Nonos.Stark.Field.mul_distributes_over_add
#print axioms Nonos.Stark.Extension.low_degree_extension_is_unique
#print axioms Nonos.Stark.Commitment.commitment_binds_value
#print axioms Nonos.Stark.FeeRouter.route_conserves
#print axioms Nonos.Stark.FeeRouter.an_accepted_fee_is_within_cap
#print axioms Nonos.Stark.Fold.eval_split
#print axioms Nonos.Stark.Instance.honest_run_has_a_quotient
#print axioms Nonos.Stark.Instance.tampered_run_has_no_quotient
#print axioms Nonos.Stark.Fold.fold_length
#print axioms Nonos.Stark.Fold.the_honest_fold_reaches_a_constant
#print axioms Nonos.Stark.Fri.final_layer_accepts_iff_matches
#print axioms Nonos.Stark.Lookup.non_table_value_has_zero_multiplicity
#print axioms Nonos.Stark.Merkle.distinct_leaves_give_distinct_roots
#print axioms Nonos.Stark.NullifierSet.a_recorded_nullifier_cannot_be_respent
#print axioms Nonos.Stark.NullifierSet.spend_only_grows
#print axioms Nonos.Stark.Permutation.perm_preserves_count
#print axioms Nonos.Stark.Pool.every_reachable_pool_is_solvent
#print axioms Nonos.Stark.RootWindow.a_pushed_root_is_accepted
#print axioms Nonos.Stark.RootWindow.the_window_never_exceeds_the_cap
#print axioms Nonos.Stark.Staking.rewards_never_exceed_fees
#print axioms Nonos.Stark.RunningSum.final_is_total
#print axioms Nonos.Stark.RunningSum.conservation
#print axioms Nonos.Stark.RunningSum.follows_pins_the_trace
#print axioms Nonos.Stark.Polynomial.eval_mul
#print axioms Nonos.Stark.Polynomial.zerofier_nonzero_off_the_points
#print axioms Nonos.Stark.Polynomial.factor
#print axioms Nonos.Stark.Polynomial.roots_divide
#print axioms Nonos.Stark.Polynomial.agreement_divides_by_zerofier
#print axioms Nonos.Stark.Polynomial.root_bound
#print axioms Nonos.Stark.Transcript.order_changes_the_state
#print axioms Nonos.Syscall.decode_agrees_with_the_registry
#print axioms Nonos.UsbHid.bindings_never_exceed_the_cap

-- Assurance capstone: the guarantees composed, post-quantum hybrid authority,
-- and the admission theorem that holds over the exact gated function the loader
-- runs (attested AND rollback-fresh AND post-quantum-authorized).
#print axioms Nonos.Assurance.authority
#print axioms Nonos.Assurance.freshness
#print axioms Nonos.Assurance.integrity
#print axioms Nonos.Assurance.classical_break_insufficient
#print axioms Nonos.Assurance.no_pq_no_authority
#print axioms Nonos.Assurance.only_ok_capsules_run
#print axioms Nonos.Assurance.run_capsule_pq_authorized
#print axioms Nonos.Assurance.unattested_never_runs
#print axioms Nonos.Assurance.stale_never_runs
#print axioms Nonos.Assurance.unsigned_pq_never_runs

-- Kernel-mechanism modules added alongside the capstone. One flagship theorem
-- from each is profiled so the CI evidence records its axiom closure and the
-- sorryAx gate covers it.
#print axioms Nonos.MemGrant.run_conserves
#print axioms Nonos.MemGrant.run_granted_le_capacity
#print axioms Nonos.Scheduler.rotate_mem
#print axioms Nonos.Scheduler.at_most_one_cpu_wins
#print axioms Nonos.Scheduler.a_ready_process_is_claimed_once
#print axioms Nonos.Scheduler.only_ready_is_claimed
#print axioms Nonos.PageDescriptor.leaf_present_iff_requested
#print axioms Nonos.PageDescriptor.kernel_leaf_never_reaches_el0
#print axioms Nonos.PageDescriptor.user_leaf_reaches_el0
#print axioms Nonos.PageDescriptor.read_only_leaf_never_writable
#print axioms Nonos.PageDescriptor.user_leaf_never_executes_at_el1
#print axioms Nonos.PageDescriptor.kernel_leaf_never_executes_at_el0
#print axioms Nonos.PageDescriptor.table_is_present_and_not_a_block
#print axioms Nonos.PageTable.mapAllChecked_safe
#print axioms Nonos.Iommu.empty_grant_denies
#print axioms Nonos.Dispatch.serviced_requires_cap
#print axioms Nonos.Dispatch.denied_below
#print axioms Nonos.Quota.acquireAll_used_le_cap
#print axioms Nonos.Interval.disjoint_not_mem
#print axioms Nonos.Interval.mem_of_subset
#print axioms Nonos.Refcount.dec_from_one_dead
#print axioms Nonos.Rflags.mask_is_exactly_the_privileged_bits
#print axioms Nonos.Rflags.iopl_is_masked
#print axioms Nonos.Rflags.interrupt_flag_is_not_masked
#print axioms Nonos.Timer.tickAll_monotone
#print axioms Nonos.Endpoint.recv_was_sent
#print axioms Nonos.Heap.double_free_safe
#print axioms Nonos.Heap.free_not_allocated
#print axioms Nonos.Fd.close_not_open
#print axioms Nonos.Ring.run_count_le_cap
#print axioms Nonos.Bounds.index_in_buffer
#print axioms Nonos.Nonce.issue2_distinct
#print axioms Nonos.Priority.preempts_total
#print axioms Nonos.Zeroize.wiped_is_zero
#print axioms Nonos.Mmio.empty_grant_denies
#print axioms Nonos.CapTable.revoke_not_holds
#print axioms Nonos.CapTable.grant_then_revoke

-- Capability tokens: a valid token clears all three gates, and the boolean and
-- result-path entry points agree.
#print axioms Nonos.CapToken.valid_not_revoked
#print axioms Nonos.CapToken.revoked_invalid
#print axioms Nonos.CapToken.full_ok_iff_valid

-- Capability masks: a subset (delegated) mask never carries a capability its
-- parent lacks, and granting is monotone and non-aliasing.
#print axioms Nonos.CapMask.subset_no_extra
#print axioms Nonos.CapMask.has_add_other
#print axioms Nonos.CapMask.subset_trans
#print axioms Nonos.Vfs.resolve_dotdots_root
#print axioms Nonos.Rng.drawN_advances
#print axioms Nonos.Tlb.invalidate_evicts

-- Concurrency, reclamation and rate-limiting mechanisms. One flagship theorem
-- from each is profiled so its axiom closure is recorded and the sorryAx gate
-- covers it.
#print axioms Nonos.Semaphore.acquire_valid
#print axioms Nonos.Semaphore.acquire_release_roundtrip
#print axioms Nonos.Mutex.owner_unique
#print axioms Nonos.Ticket.serving_unique
#print axioms Nonos.Ticket.take_monotone
#print axioms Nonos.Seqlock.changed_rejected
#print axioms Nonos.TokenBucket.refill_never_exceeds_burst
#print axioms Nonos.Signal.blocked_signal_still_pending
#print axioms Nonos.Signal.unblock_delivers
#print axioms Nonos.Reaper.reaped_not_zombie
#print axioms Nonos.Epoch.drainN_old_le
#print axioms Nonos.Barrier.not_released_before_all
#print axioms Nonos.Buddy.alloc_conserves
#print axioms Nonos.Buddy.split_conserves
#print axioms Nonos.Cow.write_drops_original
#print axioms Nonos.Bitmap.set_then_clear_frees

-- Locking and address-space mechanisms, each backed by a real kernel primitive.
#print axioms Nonos.Spinlock.try_fails_when_held
#print axioms Nonos.Rwlock.writer_excludes_readers
#print axioms Nonos.Futex.fifo_first_out
#print axioms Nonos.Futex.waiter_enqueued
#print axioms Nonos.Vma.disjoint_no_shared_addr

-- WiFi trusted path: the WPA2 supplicant's handshake discipline and the CCMP
-- packet-number replay window the data plane enforces under each key.
#print axioms Nonos.Wpa2Handshake.install_requires_valid_msg3
#print axioms Nonos.Wpa2Handshake.replay_never_advances
#print axioms Nonos.Wpa2Handshake.mic_input_within_eapol
#print axioms Nonos.Wpa2Handshake.connected_ptk_from_fixed_nonces
#print axioms Nonos.CcmpReplay.no_nonce_reuse
#print axioms Nonos.CcmpReplay.replay_dropped
#print axioms Nonos.CcmpReplay.accepted_pn_dead_forever

-- User/kernel boundary: the range policy every usercopy clears before a byte
-- moves keeps an accepted range wholly inside user space.
#print axioms Nonos.UserCopy.accepted_within_user
#print axioms Nonos.UserCopy.accepted_nonzero_addr

-- The page walk behind that policy: every table above a returned leaf granted
-- user, and a leaf a transfer accepts is user and, for writes, writable.
#print axioms Nonos.UserWalk.tables_above_grant_user
#print axioms Nonos.UserWalk.read_path_is_user_accessible
#print axioms Nonos.UserWalk.write_path_is_user_writable
#print axioms Nonos.UserWalk.write_implies_read

-- Demand paging: the fault router and the per-process page budget, and the
-- fact that a served page is never executable.
#print axioms Nonos.DemandPaging.kernel_half_never_mapped
#print axioms Nonos.DemandPaging.refused_forever
#print axioms Nonos.DemandPaging.saturated_refused
#print axioms Nonos.DemandPaging.saturated_admission_bounded
#print axioms Nonos.DemandPaging.demand_not_wx

-- ELF load protection: no writable-and-executable segment is admitted, and the
-- RELRO span ends the load read-only.
#print axioms Nonos.LoadProtect.accepted_wx_safe
#print axioms Nonos.LoadProtect.sealed_not_writable

-- Service registry: registration preserves name and port uniqueness and never
-- grows the table past its cap.
#print axioms Nonos.ServiceRegistry.register_preserves_names
#print axioms Nonos.ServiceRegistry.register_preserves_ports
#print axioms Nonos.ServiceRegistry.register_within_cap

-- PCI command-write allowlist: an admitted write changes only writable bits,
-- the merge branch is confined, and a raw protected-bit write is refused.
#print axioms Nonos.PciCmdWrite.admitted_changes_only_writable
#print axioms Nonos.PciCmdWrite.merge_branch_confined
#print axioms Nonos.PciCmdWrite.raw_protected_write_refused

-- PID allocation: an allocated PID is never the reserved 0, is not already
-- live, and the stored counter never wraps to 0.
#print axioms Nonos.PidAlloc.chosen_pid_ne_zero
#print axioms Nonos.PidAlloc.chosen_pid_inactive
#print axioms Nonos.PidAlloc.chosen_next_ne_zero

-- Network state machines: DHCP binds a lease only on a matching ACK, and TCP
-- reaches Established only through the handshake.
#print axioms Nonos.Dhcp.bound_only_via_matching_ack
#print axioms Nonos.Tcp.established_only_via_handshake

-- Syscall routing: an unclaimed number is refused, and the first handler that
-- claims a number decides it and shadows all later handlers.
#print axioms Nonos.SyscallRoute.route_unclaimed_is_enosys
#print axioms Nonos.SyscallRoute.route_earlier_shadows
#print axioms Nonos.SyscallRoute.route_append_stable

-- File-descriptor allocation: a returned descriptor is the lowest free one at
-- or above the floor, and the allocator declines only when the window is full.
#print axioms Nonos.FdAlloc.alloc_free
#print axioms Nonos.FdAlloc.alloc_lowest
#print axioms Nonos.FdAlloc.alloc_none_full

-- Multisig k-of-n: an accepted config is a well-formed threshold, a valid add
-- keeps signers distinct and authorized, and a met threshold is backed by them.
#print axioms Nonos.MultiSig.valid_config
#print axioms Nonos.MultiSig.add_preserves_nodup
#print axioms Nonos.MultiSig.threshold_backed_by_distinct_authorized

-- MSI-X exclusion: no address a clamped BAR mapping covers falls inside the
-- protected MSI-X table or PBA region.
#print axioms Nonos.MsixExclusion.no_protected_byte_mapped
#print axioms Nonos.MsixExclusion.safeLen_le_length

-- MSI-X interrupt bind: an admitted bind is bounded to the pool and device
-- table, addressable, a device IRQ, and not a double-bind.
#print axioms Nonos.IrqBind.accepted_vector_count_bounded
#print axioms Nonos.IrqBind.accepted_msix_addressable
#print axioms Nonos.IrqBind.accepted_not_already_bound

-- DMA map admission: an accepted mapping is owned by the caller, on a fresh
-- claim epoch, and bounded to the device class page limit.
#print axioms Nonos.DmaMap.accepted_owned_by_caller
#print axioms Nonos.DmaMap.accepted_fresh_epoch
#print axioms Nonos.DmaMap.accepted_within_class_limit

-- ELF relocation write size: any supported relocation writes at most 8 bytes,
-- an unknown type is refused, and an admitted relocation writes only in-segment.
#print axioms Nonos.ElfReloc.writeSize_le_8
#print axioms Nonos.ElfReloc.other_unsupported
#print axioms Nonos.ElfReloc.admitted_no_oob

-- ELF program-header bounds: an accepted table lies wholly inside the image, so
-- every header the loader reads is in bounds.
#print axioms Nonos.ElfPhdr.accepted_table_in_bounds
#print axioms Nonos.ElfPhdr.accepted_no_overflow
#print axioms Nonos.ElfPhdr.wrong_size_rejected


-- Scheduling fairness / liveness: the ready set is a scheduling invariant, no
-- admitted task is ever dropped or starved out of the queue.
#print axioms Nonos.Fairness.rotateN_mem
#print axioms Nonos.Fairness.rotateN_length
#print axioms Nonos.Fairness.no_starvation_by_loss
#print axioms Nonos.Fairness.never_stalls
#print axioms Nonos.Fairness.rotateN_to_head
#print axioms Nonos.Fairness.reaches_head

-- Trusted-path audit fixes (PR #405). Each theorem pins the property the code
-- change establishes, on Lean's standard axioms alone.
#print axioms Nonos.FrameNoAlias.alloc_no_alias
#print axioms Nonos.FrameNoAlias.bump_aliases
#print axioms Nonos.FrameNoAlias.fixed_none_on_empty
#print axioms Nonos.InputConsumer.drain_implies_post
#print axioms Nonos.InputConsumer.irq_only_cannot_drain
-- And who can hold a token to stamp with in the first place: only the process
-- the call was addressed to, spent once.
#print axioms Nonos.ReplyAuthorization.a_reply_token_comes_from_a_call_to_the_replier
#print axioms Nonos.ReplyAuthorization.a_redirect_token_comes_from_a_call_to_the_sender
#print axioms Nonos.ReplyAuthorization.remove_consumes_one
#print axioms Nonos.ReplyAuthorization.pop_consumes_one

#print axioms Nonos.ReplyCorrelation.forged_never_delivered
#print axioms Nonos.ReplyCorrelation.firstMatch_matches
#print axioms Nonos.ReplyCorrelation.all_forged_none
#print axioms Nonos.ServiceRegisterAuth.new_no_bypass
#print axioms Nonos.ServiceRegisterAuth.old_low_pid_bypass
#print axioms Nonos.FramebufferSwap.swap_involutive
#print axioms Nonos.FramebufferSwap.present_rgb_converts
#print axioms Nonos.ZeroState.off_was_wiped
#print axioms Nonos.ZeroState.wipe_covers_every_region
#print axioms Nonos.ZeroState.wiped_residue_empty
#print axioms Nonos.ZeroState.off_holds_nothing
#print axioms Nonos.ZeroState.into_off_only_from_wiped
#print axioms Nonos.ZeroState.no_shortcut_to_off
#print axioms Nonos.ZeroState.quiescing_is_not_wiping
#print axioms Nonos.ZeroState.secrets_imply_still_here
#print axioms Nonos.ZeroState.off_is_terminal
#print axioms Nonos.StationAddress.never_group
#print axioms Nonos.StationAddress.always_local
#print axioms Nonos.StationAddress.never_a_factory_address
#print axioms Nonos.StationAddress.never_broadcast
#print axioms Nonos.StationAddress.never_zero
