#![cfg_attr(feature="gen_ir", feature(plugin))]
#![cfg_attr(feature="gen_ir", plugin(interpolate_idents))]

extern crate libc;
extern crate llvm_sys;

use std::ffi::{CString, CStr};

use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

#[macro_use]
mod macros;

#[cfg(feature="gen_ir")]
#[macro_use]
mod gen_ir;

mod context;
mod types;
mod builder;
mod module;
mod function;
mod pass_manager;
mod target;
mod execution_engine;
mod value;

// TODO: This was to maintain compatiblity, we should remove this
pub use context::*;
pub use types::*;
pub use builder::*;
pub use module::*;
pub use function::*;
pub use pass_manager::*;
pub use target::*;
pub use execution_engine::*;
pub use value::*;

pub fn set_value_name(val: LLVMValueRef, name: &str) {
    let c_name = CString::new(name).unwrap();
    unsafe {
        llvm::LLVMSetValueName(val, c_name.as_ptr());
    }
}
