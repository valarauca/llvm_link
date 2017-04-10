//!llvm_link
//!
//!These are bindings to the LLVM's `libLTO` **L**ink **T**ime **O**ptimization library.
//!
//!This crate currently assumes you are locally using LLVMv3.9.1 It was originally
//!compiled on Fedora25 AMD64, so you may have trouble getting it to work
//!on other platforms. Windows and OSX is not yet supported.
//!To build on Fedora25 you will need to install.
//!
//!```bash
//!sudo dnf install gcc gcc-c++ llvm-devel redhat-rpm-config ncurses-devel
//!```
//!
//!To use in a Rust Project
//!
//!```ignore
//![dependencies]
//!llvm_link = "0.0.1"
//!```
//!
//! # Examples:
//!
//! To get the local libLTO version:
//!
//! ```rust,no_run
//! use llvm_link::get_lto_version;
//!
//! println!("{}",get_lto_version());
//! ```
//!
extern crate llvm_sys;
use llvm_sys::lto::*;
use std::ffi::CStr;
mod module;
mod merge;


/// Get the LTO version
pub fn get_lto_version() -> String {
    fn c_str(x: *const i8) -> String {
        if x.is_null() {
            return String::with_capacity(0);
        }
        unsafe {
            CStr::from_ptr(x as *const _)
                .to_string_lossy()
                .trim()
                .to_string()
        }
    }
    let ptr = unsafe {lto_get_version()};
    c_str(ptr)
}

/// Get libLTO API version
pub fn get_lto_api_version() -> u32 {
    unsafe {lto_api_version()}
}

/// Get the LTO error message
///
/// This is set when ever an interface returns null
pub fn get_error_msg() -> String {
    fn c_str(x: *const i8) -> String {
        if x.is_null() {
            return String::with_capacity(0);
        }
        unsafe {
            CStr::from_ptr(x as *const _)
                .to_string_lossy()
                .trim()
                .to_string()
        }
    }
    let ptr = unsafe{ lto_get_error_message() };
    c_str(ptr)
}

pub use self::module::{
    LinkerObject,
    ObjFile,
    Object
};
pub use self::merge::{
    Linker,
    PIC
};
