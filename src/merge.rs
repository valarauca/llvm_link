use std::slice;
use std::mem;
use std::path::Path;
use std::ffi::CString;
use std::default::Default;
use super::llvm_sys::lto::*;
use super::get_error_msg;
use super::module::{
    ObjFile,
    Object,
    LinkerObject
};

/// CodeGeneration Model
///
/// Determine **P**osition **I**ndependent **C**ode Model.
/// for more information please see
/// [LLVM Reference docs](http://llvm.org/test-doxygen/api/lto_8h.html#a2bce26a37f3a58f5966c327e984e13c2)
#[derive(Copy,Clone,Debug)]
pub enum PIC {
    Static,
    Dynamic,
    DynamicNoPIC,
    Default
}
impl Default for PIC {
    /// Return's `PIC::Default`
    #[inline(always)]
    fn default() -> PIC {
        PIC::Default
    }
}
impl Into<lto_codegen_model> for PIC {
    #[inline(always)]
    fn into(self) -> lto_codegen_model {
        match self.clone() {
            PIC::Static => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_STATIC,
            PIC::Dynamic => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_DYNAMIC,
            PIC::DynamicNoPIC => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_DYNAMIC_NO_PIC,
            PIC::Default => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_DEFAULT
        }
    }
}

/// Used to construct the Linker
///
/// This builds an instance of, and can execute the LLVM libLTO linker
///
/// ```rust,no_run
///
/// use llvm_link::{Linker,PIC,ObjFile,Object};
///
/// // build the linker(s)
/// let mut linker_stage1 = match Linker::new(true,true,false,PIC::default(),&["foo", "bar"]) {
///     Ok(x) => x,
///     Err(e) => panic!("Could not construct linker. Here's why: {}",e)
/// };
/// let mut linker_stage2 = match Linker::new(false,false,true,PIC::default(),&[]) {
///     Ok(x) => x,
///     Err(e) => panic!("Could not construct linker. Here's why: {}",e)
/// };
///
/// //load files
/// let obj1 = match ObjFile::new("/home/me/my_project/my_file.o") {
///     Ok(x) => x,
///     Err(e) => panic!("The LLVM can't load my_file.o Here's why: {}",e)
/// };
/// let obj2 = match ObjFile::new("/home/me/my_project/my_other_file.o") {
///     Ok(x) => x,
///     Err(e) => panic!("The LLVM can't load my_other_file.o Here's why: {}",e)
/// };
/// let obj3 = match ObjFile::new("/home/me/my_project/my_final_file.o") {
///     Ok(x) => x,
///     Err(e) => panic!("The LLVM can't load my_file_file.o Here's why: {}",e)
/// };
///
/// // add the files
/// linker_stage1.add_file(obj1);
/// linker_stage2.add_file(obj2);
///
/// // compile in memory
/// let stage1_obj: Object = match linker_stage1.link_to_mem() {
///     Ok(x) => x,
///     Err(e) => panic!("Link error in stage 1: {}",e)
/// };
///
/// // set up stage 2
/// linker_stage2.add_buffer(stage1_obj);
/// linker_stage2.add_file(obj3);
///
/// // release output
/// match linker_stage2.link_to_file("/home/me/my_project/my_project.a") {
///     Ok(_) => { },
///     Err(e) => panic!("Link error in stage 2: {}", e)
/// };
/// ```
///
#[allow(dead_code)]
pub struct Linker {
    ptr: lto_code_gen_t,
    symbols: Vec<CString>,
    data: Vec<Vec<u8>>
}
impl Drop for Linker {
    fn drop(&mut self) {
        unsafe {lto_codegen_dispose(self.ptr)};
    }
}
impl Linker {

    /// Construct a new Linker
    ///
    /// Contains _all_ the options that are useful for this
    ///
    /// * `keep_dwarf`: DebugModel. If `true` DWARF symbols are preserved
    /// * `embed_use_list`: Should the use list symbols be embedded within the output. This should
    /// be true for all but the last binary
    /// * `should_internalize`: I have zero clue what this does. It was removed in LLVMv4.0 here is
    /// a discussion why [linky](https://groups.google.com/forum/#!topic/llvm-dev/EMKIiIqxOo4) 
    /// * `pic mode`: I think this has to with Position Independent Code (not 100% sure)
    /// * `keep_symbols`: Force the linker to preserve these symbols
    pub fn new(keep_dwarf: bool, embed_use_list: bool, should_internalize: bool, pic_mode: PIC, keep_symbols: &[&str]) -> Result<Linker,String> {
        let linker = unsafe {lto_codegen_create()};
        if linker.is_null() {
            return Err(get_error_msg());
        }
        let mut keep = Vec::with_capacity(0);
        //dwarf
        let debug = if keep_dwarf.clone() {
                lto_debug_model::LTO_DEBUG_MODEL_DWARF
            } else {
                lto_debug_model::LTO_DEBUG_MODEL_NONE
        };
        let flag = unsafe {lto_codegen_set_debug_model(linker, debug)};
        if flag != 0 {
            return Err(get_error_msg());
        }
        //embed use list
        let flag = if embed_use_list.clone() { 1 } else { 0 };
        unsafe {lto_codegen_set_should_embed_uselists(linker, flag)};
        //internalizing
        let flag = if should_internalize.clone() { 1 } else { 0 };
		unsafe {lto_codegen_set_should_internalize(linker, flag)};
        // PIC
        let flag = unsafe {lto_codegen_set_pic_model(linker, pic_mode.clone().into())};
        if flag != 0 {
            return Err(get_error_msg());
        }
        //debug symbols
        for s in keep_symbols {
            let symbol = CString::new(s.to_string()).unwrap();
		    let p = symbol.as_ptr();	
		    unsafe {lto_codegen_add_must_preserve_symbol(linker, p as *const _)};
            keep.push(symbol);
        }
        Ok(Linker {
            ptr: linker,
            symbols: keep,
            data: Vec::with_capacity(0)
        })
    }

    /// Link an Object File
    pub fn add_file(&mut self, object_file: ObjFile) {
        let ptr = object_file.as_object();
        unsafe {lto_codegen_set_module(self.ptr, ptr)};
        mem::forget(object_file);
    }

    /// Link to a Object File (in a memory buffer)
    pub fn add_buffer(&mut self, object_buffer: Object) {
        let ptr = object_buffer.as_object();
        let mut vv = Vec::with_capacity(0);
        let mut obj = object_buffer;
        obj.get_buffer(&mut vv);
        self.data.push(vv);
        unsafe {lto_codegen_set_module(self.ptr, ptr)};
        mem::forget(obj);
    }

    /// Complete linking, return a memory buffer
    pub fn link_to_mem(self) -> Result<Object,String> {
        let mut len = 0usize;
        let ptr = unsafe {lto_codegen_compile_optimized(self.ptr, &mut len)};
        if ptr.is_null() {
            return Err(get_error_msg());
        }
        let buffer = unsafe{ slice::from_raw_parts(ptr as *const u8, len)};
        Object::from_slice(buffer)
    }

    /// Complete linking, write to a file
    pub fn link_to_file<P: AsRef<Path>>(self, path: P) -> Result<(),String> {
        let path = CString::new(path.as_ref().to_string_lossy().to_string()).unwrap();
        let flag = unsafe {lto_codegen_compile_to_file(self.ptr, path.as_ptr() as *mut _)};
        if flag != 0 {
            return Err(get_error_msg());
        }
        Ok(())
    }
}
