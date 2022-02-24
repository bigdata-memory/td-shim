// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
#![no_main]
use libfuzzer_sys::fuzz_target;

use uefi_pi::hob;

const HOB_ACPI_TABLE_GUID: [u8; 16] = [
    0x70, 0x58, 0x0c, 0x6a, 0xed, 0xd4, 0xf4, 0x44, 0xa1, 0x35, 0xdd, 0x23, 0x8b, 0x6f, 0xc, 0x8d,
];

const HOB_KERNEL_INFO_GUID: [u8; 16] = [
    0x12, 0xa4, 0x6f, 0xb9, 0x1f, 0x46, 0xe3, 0x4b, 0x8c, 0xd, 0xad, 0x80, 0x5a, 0x49, 0x7a, 0xc0,
];

fn fuzz_hob_parser(buffer: &[u8]) {
    if hob::get_hob_total_size(buffer).is_some() {
        hob::dump_hob(buffer);
        hob::get_system_memory_size_below_4gb(buffer);
        hob::get_total_memory_top(buffer);
        hob::get_fv(buffer);
        hob::get_next_extension_guid_hob(buffer, &HOB_ACPI_TABLE_GUID);
        hob::get_next_extension_guid_hob(buffer, &HOB_KERNEL_INFO_GUID);
        hob::get_guid_data(buffer);
        hob::seek_to_next_hob(buffer);
    }
}
fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    fuzz_hob_parser(data);
});