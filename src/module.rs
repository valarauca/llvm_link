use std::mem;
use std::path::Path;
use std::ffi::{
    CString,
    CStr
};

use super::llvm_sys::lto::*;
use super::get_error_msg;
#[inline(always)]
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

/// Unifying properties that all linker objects posses.
///
/// Object files are known by the type `ObjFile`. This lets the developer inspect them
/// easierly
///
/// ```rust,no_run
/// use llvm_link::{ObjFile,LinkerObject};
///
/// let obj = match ObjFile::new("my/file.o") {
///     Ok(x) => x,
///     Err(e) => panic!("Could not load {}",e)
/// };
/// match obj.get_target_triple() {
///     Ok(x) => println!("m/file.o's target triple is {}",x),
///     Err(e) => panic!("Could not load triple {}",e)
/// };
/// ```
pub trait LinkerObject {

    fn as_object(&self) -> lto_module_t;
    
    /// Get the number of symbols in an object file
    fn get_num_symbols(&self) -> u32 {
        unsafe {lto_module_get_num_symbols(self.as_object())}
    }
    
    /// Get the name of a symbol at a certain index
    fn get_symbol_name(&self, index: u32) -> Result<String,String> {
        let ptr = unsafe {lto_module_get_symbol_name(self.as_object(), index)};
        if ptr.is_null() {
            return Err(get_error_msg());
        }
        Ok(c_str(ptr))
    }
    
    /// Get the target triple an object file was compiled for
    fn get_target_triple(&self) -> Result<String,String> {
        let ptr = unsafe {lto_module_get_target_triple(self.as_object())};
        if ptr.is_null() {
            return Err(get_error_msg());
        }
        Ok(c_str(ptr))
    }
}

/// Object Files loaded from the File System into LLVM directly
///
/// These files are not part of Rust's memory. They remain within
/// the LLVM. The `ObjectFile` type is really only 1 ptr wide
pub struct ObjFile {
    ptr: lto_module_t
}
impl Drop for ObjFile {
    fn drop(&mut self) { unsafe {lto_module_dispose(self.ptr)}; }
}
impl LinkerObject for ObjFile {

    /// Exposes internal C-binding poitner
    #[inline(always)]
    fn as_object(&self) -> lto_module_t { self.ptr }
}
impl ObjFile {

    /// Load an Object File from the file system
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ObjFile,String> {
        let path = CString::new(path.as_ref().to_string_lossy().to_string()).unwrap();
        ObjFile::from_cstr(&path) 
    }

    /// Load an Object File asserting its target triple
    pub fn assert_target_triple<P: AsRef<Path>, S: AsRef<str>>(path: P, target_triple_prefix: S) -> Result<ObjFile,String> {
        let path = CString::new(path.as_ref().to_string_lossy().to_string()).unwrap();
        let tt = CString::new(target_triple_prefix.as_ref()).unwrap(); 
        let flag = ObjFile::is_obj_file_for_target_ffi(&path,&tt);
        if flag {
            ObjFile::from_cstr(&path)
        } else {
            Err(format!("Input file {:?} isn't compatible with triple {:?}",path,tt))
        }
    }
    
    /// Check if path leads to an object file
    pub fn is_object_file<P: AsRef<Path>>(path: P) -> bool {
        let path = CString::new(path.as_ref().to_string_lossy().to_string()).unwrap();
        ObjFile::is_object_file_ffi(&path)   
    }
    
    /// Check if path is an object file for a specific target triple
    pub fn is_object_file_for_target<P: AsRef<Path>, S: AsRef<str>>(path: P, target_triple_prefix: S) -> bool {
        let path = CString::new(path.as_ref().to_string_lossy().to_string()).unwrap();
        let tt = CString::new(target_triple_prefix.as_ref()).unwrap(); 
        ObjFile::is_obj_file_for_target_ffi(&path,&tt)
    }

    /// Functionally this is an identical operation to `ObjFile::new`
    ///
    /// The only difference is the internal path is not re-allocated
    /// as `CStr` implies there is _always_ a null terminator. While
    /// `AsRef<path>` is a very nice Rusty way to handle path variables.
    pub fn from_cstr(path: &CStr) -> Result<ObjFile, String> {
        let p = path.as_ptr();
        let ptr = unsafe {lto_module_create(p as *const _)};
        if ptr.is_null() {
            return Err(get_error_msg());
        }
        Ok(ObjFile{ ptr: ptr })
    }

    /// Functionally this is identical to the operation `ObjFile::is_object_file`
    ///
    /// Just the internal path is not re-allocated
    #[inline]
    pub fn is_object_file_ffi(path: &CStr) -> bool {
        let c = path.as_ptr();
        let flag = unsafe {lto_module_is_object_file(c as *const _)};
        flag == 1
    }

    /// Functionally this is identical to the operation `ObjFile::is_object_file_for_target`
    ///
    /// Just the internal path is not re-allocated
    #[inline]
    pub fn is_obj_file_for_target_ffi(path: &CStr, tt: &CStr) -> bool {
        let p = path.as_ptr();
        let t = tt.as_ptr();
        let flag = unsafe {lto_module_is_object_file_for_target(p as *const _, t as *const _)};
        flag == 1
    }
}


/// Link Time Optimization Object
///
/// Object that is fully loaded into memory. This contains its own buffer
/// as the LLVM is quickly free its memory after generating an Object File.
#[allow(dead_code)]
pub struct Object {
    ptr: lto_module_t,
    buffer: Vec<u8>
}
impl Drop for Object {
    fn drop(&mut self) { unsafe {lto_module_dispose(self.ptr)}; }
}
impl LinkerObject for Object {

    /// Exposes internal C-binding poitner
    #[inline(always)]
    fn as_object(&self) -> lto_module_t { self.ptr }
}
impl Object {

    /// Builds a new object file.
    #[inline]
    pub fn from_vec(obj: Vec<u8>) -> Result<Object,String> {
        let buffer = obj;
        let ptr = unsafe {
            let ptr = buffer.as_ptr();
            let len = buffer.len();
            lto_module_create_from_memory(ptr as *const _, len)
        };
        if ptr.is_null() {
            return Err(get_error_msg());
        } 
        
        else {
            Ok(Object {
                ptr: ptr,
                buffer: buffer
            })
        }
    }

    /// Builds a new object file
    ///
    /// But clones the underlying buffer to ensure ownership
    pub fn from_slice(obj: &[u8]) -> Result<Object,String> {
        Object::from_vec(obj.to_vec())  
    }

    /// Internal method to steal inner buffer.
    ///
    /// Swaps `v` with own internal buffer. Be warned **THE LLVM HAS POINTERS TO THIS**
    /// if `v` is free'd before `self` this can cause memory errors
    pub fn get_buffer(&mut self, v: &mut Vec<u8>) {
        mem::swap(&mut self.buffer, v);
    }
}
