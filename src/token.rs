// Contract doesn't use standard library
#![no_std]
#![feature(wasm_import_memory)]
#![wasm_import_memory]

extern crate pwasm_std;

/// Will be described in the next step
#[no_mangle]
pub fn deploy(desc: *mut u8) {
}

/// The call function is the main function of the *deployed* contract
/// Function receives a pointer for the call descriptor.
#[no_mangle]
pub fn call(desc: *mut u8) {
    // pwasm_std::parse_args splits the call descriptor into arguments and result pointers
    let (_args, result) = unsafe { pwasm_std::parse_args(desc) };
    // result.done writes the result vector to the call descriptor.
    result.done(b"result".to_vec());
}
