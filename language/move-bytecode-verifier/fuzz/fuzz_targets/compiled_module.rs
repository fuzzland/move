// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

#![no_main]
use libfuzzer_sys::fuzz_target;
use move_binary_format::deserializer::deserialize_compiled_module;
use move_binary_format::file_format::CompiledModule;
//
// fn deserialize_module(data: &[u8]) -> Option<CompiledModule> {
//     let res = deserialize_compiled_module(data);
// //
// // }

fuzz_target!(|data: &[u8]| {
    let res = deserialize_compiled_module(data, 6);
    // match res {
    //     Ok(module) => data[10000000000000],
    //     Err(_) => 0,
    // };
});
