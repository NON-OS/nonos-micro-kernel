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

use crate::process::tests::scheduler_types_tests as t;
use crate::test::framework::{TestCase, TestSuite};

pub fn register(s: &mut TestSuite) {
    s.add(TestCase::new("sched_policy_constants", t::sched_policy_constants));
    s.add(TestCase::new("sched_priority_range", t::sched_priority_range));
    s.add(TestCase::new("nice_value_range", t::nice_value_range));
    s.add(TestCase::new("sched_flag_constants", t::sched_flag_constants));
    s.add(TestCase::new("ioprio_class_constants", t::ioprio_class_constants));
    s.add(TestCase::new("ioprio_who_constants", t::ioprio_who_constants));
    s.add(TestCase::new("timeslice_constants", t::timeslice_constants));
    s.add(TestCase::new("sched_attr_default", t::sched_attr_default));
    s.add(TestCase::new("sched_attr_is_realtime_fifo", t::sched_attr_is_realtime_fifo));
    s.add(TestCase::new("sched_attr_is_realtime_rr", t::sched_attr_is_realtime_rr));
    s.add(TestCase::new("sched_attr_is_not_realtime", t::sched_attr_is_not_realtime));
    s.add(TestCase::new(
        "sched_attr_effective_priority_normal",
        t::sched_attr_effective_priority_normal,
    ));
    s.add(TestCase::new(
        "sched_attr_effective_priority_normal_with_nice",
        t::sched_attr_effective_priority_normal_with_nice,
    ));
    s.add(TestCase::new(
        "sched_attr_effective_priority_fifo",
        t::sched_attr_effective_priority_fifo,
    ));
    s.add(TestCase::new("sched_attr_effective_priority_rr", t::sched_attr_effective_priority_rr));
    s.add(TestCase::new(
        "sched_attr_effective_priority_deadline",
        t::sched_attr_effective_priority_deadline,
    ));
    s.add(TestCase::new(
        "sched_attr_effective_priority_idle",
        t::sched_attr_effective_priority_idle,
    ));
    s.add(TestCase::new(
        "sched_attr_effective_priority_batch",
        t::sched_attr_effective_priority_batch,
    ));
    s.add(TestCase::new("sched_attr_can_run_on_cpu", t::sched_attr_can_run_on_cpu));
    s.add(TestCase::new(
        "sched_attr_can_run_on_cpu_high_cpu",
        t::sched_attr_can_run_on_cpu_high_cpu,
    ));
    s.add(TestCase::new("sched_attr_get_timeslice_fifo", t::sched_attr_get_timeslice_fifo));
    s.add(TestCase::new("sched_attr_get_timeslice_rr", t::sched_attr_get_timeslice_rr));
    s.add(TestCase::new("sched_attr_get_timeslice_normal", t::sched_attr_get_timeslice_normal));
    s.add(TestCase::new("sched_attr_clone", t::sched_attr_clone));
    s.add(TestCase::new("encode_decode_ioprio", t::encode_decode_ioprio));
    s.add(TestCase::new("encode_decode_ioprio_rt", t::encode_decode_ioprio_rt));
    s.add(TestCase::new("encode_decode_ioprio_idle", t::encode_decode_ioprio_idle));
    s.add(TestCase::new("encode_ioprio_max_level", t::encode_ioprio_max_level));
    s.add(TestCase::new("sched_param_default", t::sched_param_default));
    s.add(TestCase::new("sched_param_with_priority", t::sched_param_with_priority));
    s.add(TestCase::new("linux_sched_attr_default", t::linux_sched_attr_default));
    s.add(TestCase::new("linux_sched_attr_clone", t::linux_sched_attr_clone));
    s.add(TestCase::new("sched_policy_stats_default", t::sched_policy_stats_default));
    s.add(TestCase::new("sched_policy_stats_clone", t::sched_policy_stats_clone));
}
