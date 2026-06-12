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

use crate::test::framework::{TestCase, TestSuite};

pub mod apic_tests;
pub mod clock_tests;
pub mod gdt_tests;
pub mod idt_tests;
pub mod io_tests;
pub mod serial_tests;
pub mod timer_tests;

pub fn run_all() -> bool {
    let mut suite = TestSuite::new("sys");

    // apic_tests (34 tests)
    suite.add(TestCase::new("apic::timer_vector_value", apic_tests::test_timer_vector_value));
    suite.add(TestCase::new("apic::irq_timer_value", apic_tests::test_irq_timer_value));
    suite.add(TestCase::new("apic::irq_keyboard_value", apic_tests::test_irq_keyboard_value));
    suite.add(TestCase::new("apic::irq_cascade_value", apic_tests::test_irq_cascade_value));
    suite.add(TestCase::new("apic::irq_com2_value", apic_tests::test_irq_com2_value));
    suite.add(TestCase::new("apic::irq_com1_value", apic_tests::test_irq_com1_value));
    suite.add(TestCase::new("apic::irq_lpt2_value", apic_tests::test_irq_lpt2_value));
    suite.add(TestCase::new("apic::irq_floppy_value", apic_tests::test_irq_floppy_value));
    suite.add(TestCase::new("apic::irq_lpt1_value", apic_tests::test_irq_lpt1_value));
    suite.add(TestCase::new("apic::irq_rtc_value", apic_tests::test_irq_rtc_value));
    suite.add(TestCase::new("apic::irq_free1_value", apic_tests::test_irq_free1_value));
    suite.add(TestCase::new("apic::irq_free2_value", apic_tests::test_irq_free2_value));
    suite.add(TestCase::new("apic::irq_free3_value", apic_tests::test_irq_free3_value));
    suite.add(TestCase::new("apic::irq_mouse_value", apic_tests::test_irq_mouse_value));
    suite.add(TestCase::new("apic::irq_coprocessor_value", apic_tests::test_irq_coprocessor_value));
    suite.add(TestCase::new("apic::irq_primary_ata_value", apic_tests::test_irq_primary_ata_value));
    suite.add(TestCase::new(
        "apic::irq_secondary_ata_value",
        apic_tests::test_irq_secondary_ata_value,
    ));
    suite.add(TestCase::new(
        "apic::vector_timer_equals_timer_vector",
        apic_tests::test_vector_timer_equals_timer_vector,
    ));
    suite.add(TestCase::new("apic::vector_keyboard_value", apic_tests::test_vector_keyboard_value));
    suite.add(TestCase::new("apic::vector_mouse_value", apic_tests::test_vector_mouse_value));
    suite.add(TestCase::new("apic::vector_com1_value", apic_tests::test_vector_com1_value));
    suite.add(TestCase::new("apic::irq_to_vector_timer", apic_tests::test_irq_to_vector_timer));
    suite.add(TestCase::new(
        "apic::irq_to_vector_keyboard",
        apic_tests::test_irq_to_vector_keyboard,
    ));
    suite.add(TestCase::new("apic::irq_to_vector_mouse", apic_tests::test_irq_to_vector_mouse));
    suite.add(TestCase::new("apic::irq_to_vector_com1", apic_tests::test_irq_to_vector_com1));
    suite.add(TestCase::new("apic::irq_to_vector_com2", apic_tests::test_irq_to_vector_com2));
    suite.add(TestCase::new("apic::irq_to_vector_rtc", apic_tests::test_irq_to_vector_rtc));
    suite.add(TestCase::new("apic::irq_to_vector_floppy", apic_tests::test_irq_to_vector_floppy));
    suite.add(TestCase::new(
        "apic::irq_to_vector_primary_ata",
        apic_tests::test_irq_to_vector_primary_ata,
    ));
    suite.add(TestCase::new(
        "apic::irq_to_vector_secondary_ata",
        apic_tests::test_irq_to_vector_secondary_ata,
    ));
    suite.add(TestCase::new(
        "apic::irq_to_vector_base_offset",
        apic_tests::test_irq_to_vector_base_offset,
    ));
    suite.add(TestCase::new(
        "apic::vector_values_above_32",
        apic_tests::test_vector_values_above_32,
    ));
    suite.add(TestCase::new("apic::irq_values_below_16", apic_tests::test_irq_values_below_16));
    suite.add(TestCase::new("apic::all_irqs_unique", apic_tests::test_all_irqs_unique));
    suite.add(TestCase::new(
        "apic::irq_to_vector_is_const",
        apic_tests::test_irq_to_vector_is_const,
    ));

    // clock_tests (24 tests)
    suite.add(TestCase::new(
        "clock::time_struct_hour_range",
        clock_tests::test_time_struct_hour_range,
    ));
    suite.add(TestCase::new(
        "clock::time_struct_minute_range",
        clock_tests::test_time_struct_minute_range,
    ));
    suite.add(TestCase::new(
        "clock::time_struct_second_range",
        clock_tests::test_time_struct_second_range,
    ));
    suite.add(TestCase::new(
        "clock::format_time_buffer_size",
        clock_tests::test_format_time_buffer_size,
    ));
    suite.add(TestCase::new(
        "clock::format_time_colon_position",
        clock_tests::test_format_time_colon_position,
    ));
    suite.add(TestCase::new(
        "clock::format_time_valid_digits",
        clock_tests::test_format_time_valid_digits,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_buffer_size",
        clock_tests::test_format_time_full_buffer_size,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_colon_positions",
        clock_tests::test_format_time_full_colon_positions,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_valid_digits",
        clock_tests::test_format_time_full_valid_digits,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_hour_range",
        clock_tests::test_format_time_full_hour_range,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_minute_range",
        clock_tests::test_format_time_full_minute_range,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_second_range",
        clock_tests::test_format_time_full_second_range,
    ));
    suite.add(TestCase::new(
        "clock::format_date_short_buffer_size",
        clock_tests::test_format_date_short_buffer_size,
    ));
    suite.add(TestCase::new(
        "clock::format_date_short_contains_space",
        clock_tests::test_format_date_short_contains_space,
    ));
    suite.add(TestCase::new(
        "clock::format_date_short_contains_colon",
        clock_tests::test_format_date_short_contains_colon,
    ));
    suite.add(TestCase::new(
        "clock::format_date_short_ends_with_am_or_pm",
        clock_tests::test_format_date_short_ends_with_am_or_pm,
    ));
    suite.add(TestCase::new(
        "clock::format_date_only_buffer_size",
        clock_tests::test_format_date_only_buffer_size,
    ));
    suite.add(TestCase::new(
        "clock::format_date_only_no_time",
        clock_tests::test_format_date_only_no_time,
    ));
    suite.add(TestCase::new(
        "clock::unix_ms_returns_value",
        clock_tests::test_unix_ms_returns_value,
    ));
    suite.add(TestCase::new("clock::unix_ms_monotonic", clock_tests::test_unix_ms_monotonic));
    suite.add(TestCase::new(
        "clock::uptime_seconds_returns_value",
        clock_tests::test_uptime_seconds_returns_value,
    ));
    suite.add(TestCase::new("clock::get_time_consistency", clock_tests::test_get_time_consistency));
    suite.add(TestCase::new(
        "clock::format_time_produces_valid_output",
        clock_tests::test_format_time_produces_valid_output,
    ));
    suite.add(TestCase::new(
        "clock::format_time_full_produces_valid_output",
        clock_tests::test_format_time_full_produces_valid_output,
    ));

    // gdt_tests (3 tests)
    suite.add(TestCase::new("gdt::module_exists", gdt_tests::test_module_exists));
    suite.add(TestCase::new("gdt::basic_constants", gdt_tests::test_basic_constants));
    suite.add(TestCase::new("gdt::basic_operations", gdt_tests::test_basic_operations));

    // idt_tests (4 tests)
    suite.add(TestCase::new("idt::idt_module_exists", idt_tests::test_idt_module_exists));
    suite.add(TestCase::new("idt::idt_entry_count", idt_tests::test_idt_entry_count));
    suite.add(TestCase::new("idt::idt_entry_size", idt_tests::test_idt_entry_size));
    suite.add(TestCase::new("idt::interrupt_vectors", idt_tests::test_interrupt_vectors));

    // io_tests (30 tests)
    suite.add(TestCase::new("io::io_wait_exists", io_tests::test_io_wait_exists));
    suite.add(TestCase::new("io::io_wait_multiple_calls", io_tests::test_io_wait_multiple_calls));
    suite.add(TestCase::new("io::outb_function_signature", io_tests::test_outb_function_signature));
    suite.add(TestCase::new("io::inb_function_signature", io_tests::test_inb_function_signature));
    suite.add(TestCase::new("io::outw_function_signature", io_tests::test_outw_function_signature));
    suite.add(TestCase::new("io::inw_function_signature", io_tests::test_inw_function_signature));
    suite.add(TestCase::new("io::outl_function_signature", io_tests::test_outl_function_signature));
    suite.add(TestCase::new("io::inl_function_signature", io_tests::test_inl_function_signature));
    suite.add(TestCase::new("io::port_types_are_u16", io_tests::test_port_types_are_u16));
    suite.add(TestCase::new("io::byte_value_type", io_tests::test_byte_value_type));
    suite.add(TestCase::new("io::word_value_type", io_tests::test_word_value_type));
    suite.add(TestCase::new("io::dword_value_type", io_tests::test_dword_value_type));
    suite.add(TestCase::new("io::common_port_addresses", io_tests::test_common_port_addresses));
    suite.add(TestCase::new("io::pic_port_addresses", io_tests::test_pic_port_addresses));
    suite.add(TestCase::new("io::keyboard_port_address", io_tests::test_keyboard_port_address));
    suite.add(TestCase::new("io::rtc_port_addresses", io_tests::test_rtc_port_addresses));
    suite.add(TestCase::new("io::cmos_port_addresses", io_tests::test_cmos_port_addresses));
    suite.add(TestCase::new("io::pci_config_ports", io_tests::test_pci_config_ports));
    suite.add(TestCase::new("io::io_wait_port", io_tests::test_io_wait_port));
    suite.add(TestCase::new("io::port_max_value", io_tests::test_port_max_value));
    suite.add(TestCase::new("io::port_min_value", io_tests::test_port_min_value));
    suite.add(TestCase::new("io::byte_boundary_values", io_tests::test_byte_boundary_values));
    suite.add(TestCase::new("io::word_boundary_values", io_tests::test_word_boundary_values));
    suite.add(TestCase::new("io::dword_boundary_values", io_tests::test_dword_boundary_values));
    suite.add(TestCase::new("io::vga_port_addresses", io_tests::test_vga_port_addresses));
    suite.add(TestCase::new("io::ata_primary_ports", io_tests::test_ata_primary_ports));
    suite.add(TestCase::new("io::ata_secondary_ports", io_tests::test_ata_secondary_ports));

    // serial_tests (40 tests)
    suite.add(TestCase::new(
        "serial::serial_port_constant",
        serial_tests::test_serial_port_constant,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_empty_slice",
        serial_tests::test_serial_print_empty_slice,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_single_byte",
        serial_tests::test_serial_print_single_byte,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_multiple_bytes",
        serial_tests::test_serial_print_multiple_bytes,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_special_chars",
        serial_tests::test_serial_print_special_chars,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_str_empty",
        serial_tests::test_serial_print_str_empty,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_str_single",
        serial_tests::test_serial_print_str_single,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_str_multiple",
        serial_tests::test_serial_print_str_multiple,
    ));
    suite.add(TestCase::new(
        "serial::serial_println_empty",
        serial_tests::test_serial_println_empty,
    ));
    suite.add(TestCase::new(
        "serial::serial_println_message",
        serial_tests::test_serial_println_message,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_hex_zero",
        serial_tests::test_serial_print_hex_zero,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_hex_one",
        serial_tests::test_serial_print_hex_one,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_hex_max",
        serial_tests::test_serial_print_hex_max,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_hex_arbitrary",
        serial_tests::test_serial_print_hex_arbitrary,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_hex_powers_of_two",
        serial_tests::test_serial_print_hex_powers_of_two,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_zero",
        serial_tests::test_serial_print_dec_zero,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_one",
        serial_tests::test_serial_print_dec_one,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_max",
        serial_tests::test_serial_print_dec_max,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_arbitrary",
        serial_tests::test_serial_print_dec_arbitrary,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_powers_of_ten",
        serial_tests::test_serial_print_dec_powers_of_ten,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_sequential",
        serial_tests::test_serial_print_dec_sequential,
    ));
    suite.add(TestCase::new(
        "serial::serial_set_debug_enabled_true",
        serial_tests::test_serial_set_debug_enabled_true,
    ));
    suite.add(TestCase::new(
        "serial::serial_set_debug_enabled_false",
        serial_tests::test_serial_set_debug_enabled_false,
    ));
    suite.add(TestCase::new(
        "serial::serial_set_debug_enabled_toggle",
        serial_tests::test_serial_set_debug_enabled_toggle,
    ));
    suite.add(TestCase::new(
        "serial::serial_is_debug_enabled_returns_bool",
        serial_tests::test_serial_is_debug_enabled_returns_bool,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_binary_data",
        serial_tests::test_serial_print_binary_data,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_newline_variations",
        serial_tests::test_serial_print_newline_variations,
    ));
    suite.add(TestCase::new("serial::serial_print_tab", serial_tests::test_serial_print_tab));
    suite.add(TestCase::new(
        "serial::serial_print_all_printable_ascii",
        serial_tests::test_serial_print_all_printable_ascii,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_long_string",
        serial_tests::test_serial_print_long_string,
    ));
    suite.add(TestCase::new(
        "serial::serial_println_adds_newline",
        serial_tests::test_serial_println_adds_newline,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_hex_single_digit",
        serial_tests::test_serial_print_hex_single_digit,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_dec_single_digit",
        serial_tests::test_serial_print_dec_single_digit,
    ));
    suite.add(TestCase::new(
        "serial::serial_combined_output",
        serial_tests::test_serial_combined_output,
    ));
    suite.add(TestCase::new(
        "serial::serial_debug_flag_persistence",
        serial_tests::test_serial_debug_flag_persistence,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_str_utf8",
        serial_tests::test_serial_print_str_utf8,
    ));
    suite.add(TestCase::new(
        "serial::serial_print_u64_alias",
        serial_tests::test_serial_print_u64_alias,
    ));
    suite.add(TestCase::new("serial::serial_port_is_com1", serial_tests::test_serial_port_is_com1));
    suite.add(TestCase::new(
        "serial::serial_related_ports",
        serial_tests::test_serial_related_ports,
    ));

    // timer_tests (43 tests)
    suite.add(TestCase::new("timer::rdtsc_returns_value", timer_tests::test_rdtsc_returns_value));
    suite.add(TestCase::new("timer::rdtsc_increases", timer_tests::test_rdtsc_increases));
    suite.add(TestCase::new("timer::rdtsc_monotonic", timer_tests::test_rdtsc_monotonic));
    suite.add(TestCase::new(
        "timer::tsc_frequency_returns_value",
        timer_tests::test_tsc_frequency_returns_value,
    ));
    suite.add(TestCase::new("timer::ticks_to_ns_zero", timer_tests::test_ticks_to_ns_zero));
    suite.add(TestCase::new("timer::ticks_to_us_zero", timer_tests::test_ticks_to_us_zero));
    suite.add(TestCase::new("timer::ticks_to_ms_zero", timer_tests::test_ticks_to_ms_zero));
    suite.add(TestCase::new("timer::ticks_to_ns_positive", timer_tests::test_ticks_to_ns_positive));
    suite.add(TestCase::new("timer::ticks_to_us_positive", timer_tests::test_ticks_to_us_positive));
    suite.add(TestCase::new("timer::ticks_to_ms_positive", timer_tests::test_ticks_to_ms_positive));
    suite.add(TestCase::new("timer::us_to_ticks_zero", timer_tests::test_us_to_ticks_zero));
    suite.add(TestCase::new("timer::ms_to_ticks_zero", timer_tests::test_ms_to_ticks_zero));
    suite.add(TestCase::new("timer::us_to_ticks_positive", timer_tests::test_us_to_ticks_positive));
    suite.add(TestCase::new("timer::ms_to_ticks_positive", timer_tests::test_ms_to_ticks_positive));
    suite.add(TestCase::new(
        "timer::uptime_ms_returns_value",
        timer_tests::test_uptime_ms_returns_value,
    ));
    suite.add(TestCase::new(
        "timer::uptime_us_returns_value",
        timer_tests::test_uptime_us_returns_value,
    ));
    suite.add(TestCase::new(
        "timer::uptime_seconds_returns_value",
        timer_tests::test_uptime_seconds_returns_value,
    ));
    suite.add(TestCase::new("timer::uptime_ms_monotonic", timer_tests::test_uptime_ms_monotonic));
    suite.add(TestCase::new("timer::uptime_us_monotonic", timer_tests::test_uptime_us_monotonic));
    suite.add(TestCase::new(
        "timer::unix_timestamp_ms_returns_value",
        timer_tests::test_unix_timestamp_ms_returns_value,
    ));
    suite.add(TestCase::new(
        "timer::unix_timestamp_returns_value",
        timer_tests::test_unix_timestamp_returns_value,
    ));
    suite.add(TestCase::new(
        "timer::unix_timestamp_less_than_ms",
        timer_tests::test_unix_timestamp_less_than_ms,
    ));
    suite.add(TestCase::new("timer::stopwatch_start", timer_tests::test_stopwatch_start));
    suite.add(TestCase::new(
        "timer::stopwatch_elapsed_ticks",
        timer_tests::test_stopwatch_elapsed_ticks,
    ));
    suite.add(TestCase::new("timer::stopwatch_elapsed_us", timer_tests::test_stopwatch_elapsed_us));
    suite.add(TestCase::new("timer::stopwatch_elapsed_ms", timer_tests::test_stopwatch_elapsed_ms));
    suite.add(TestCase::new("timer::stopwatch_reset", timer_tests::test_stopwatch_reset));
    suite.add(TestCase::new(
        "timer::stopwatch_elapsed_increases",
        timer_tests::test_stopwatch_elapsed_increases,
    ));
    suite.add(TestCase::new("timer::is_init_returns_bool", timer_tests::test_is_init_returns_bool));
    suite.add(TestCase::new("timer::stats_returns_tuple", timer_tests::test_stats_returns_tuple));
    suite.add(TestCase::new(
        "timer::format_uptime_buffer_size",
        timer_tests::test_format_uptime_buffer_size,
    ));
    suite.add(TestCase::new(
        "timer::format_uptime_colon_positions",
        timer_tests::test_format_uptime_colon_positions,
    ));
    suite.add(TestCase::new(
        "timer::format_uptime_valid_digits",
        timer_tests::test_format_uptime_valid_digits,
    ));
    suite.add(TestCase::new(
        "timer::format_uptime_minute_range",
        timer_tests::test_format_uptime_minute_range,
    ));
    suite.add(TestCase::new(
        "timer::format_uptime_second_range",
        timer_tests::test_format_uptime_second_range,
    ));
    suite.add(TestCase::new("timer::short_delay_exists", timer_tests::test_short_delay_exists));
    suite.add(TestCase::new(
        "timer::short_delay_multiple_calls",
        timer_tests::test_short_delay_multiple_calls,
    ));
    suite.add(TestCase::new(
        "timer::ticks_conversion_roundtrip",
        timer_tests::test_ticks_conversion_roundtrip,
    ));
    suite.add(TestCase::new(
        "timer::ms_to_ticks_roundtrip",
        timer_tests::test_ms_to_ticks_roundtrip,
    ));
    suite.add(TestCase::new("timer::uptime_consistency", timer_tests::test_uptime_consistency));
    suite.add(TestCase::new(
        "timer::stats_freq_matches_tsc_frequency",
        timer_tests::test_stats_freq_matches_tsc_frequency,
    ));
    suite.add(TestCase::new("timer::timer_callback_type", timer_tests::test_timer_callback_type));
    suite.add(TestCase::new("timer::stopwatch_precision", timer_tests::test_stopwatch_precision));

    suite.run()
}
