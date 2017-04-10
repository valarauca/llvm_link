var searchIndex = {};
searchIndex["llvm_link"] = {"doc":"llvm_link","items":[[3,"ObjFile","llvm_link","Object Files loaded from the File System into LLVM directly",null,null],[3,"Object","","Link Time Optimization Object",null,null],[3,"Linker","","Used to construct the Linker",null,null],[4,"PIC","","CodeGeneration Model",null,null],[13,"Static","","",0,null],[13,"Dynamic","","",0,null],[13,"DynamicNoPIC","","",0,null],[13,"Default","","",0,null],[5,"get_lto_version","","Get the LTO version",null,{"inputs":[],"output":{"name":"string"}}],[5,"get_lto_api_version","","Get libLTO API version",null,{"inputs":[],"output":{"name":"u32"}}],[5,"get_error_msg","","Get the LTO error message",null,{"inputs":[],"output":{"name":"string"}}],[11,"drop","","",1,{"inputs":[{"name":"self"}],"output":null}],[11,"as_object","","Exposes internal C-binding poitner",1,{"inputs":[{"name":"self"}],"output":{"name":"lto_module_t"}}],[11,"new","","Load an Object File from the file system",1,{"inputs":[{"name":"p"}],"output":{"name":"result"}}],[11,"assert_target_triple","","Load an Object File asserting its target triple",1,{"inputs":[{"name":"p"},{"name":"s"}],"output":{"name":"result"}}],[11,"is_object_file","","Check if path leads to an object file",1,{"inputs":[{"name":"p"}],"output":{"name":"bool"}}],[11,"is_object_file_for_target","","Check if path is an object file for a specific target triple",1,{"inputs":[{"name":"p"},{"name":"s"}],"output":{"name":"bool"}}],[11,"from_cstr","","Functionally this is an identical operation to `ObjFile::new`",1,{"inputs":[{"name":"cstr"}],"output":{"name":"result"}}],[11,"is_object_file_ffi","","Functionally this is identical to the operation `ObjFile::is_object_file`",1,{"inputs":[{"name":"cstr"}],"output":{"name":"bool"}}],[11,"is_obj_file_for_target_ffi","","Functionally this is identical to the operation `ObjFile::is_object_file_for_target`",1,{"inputs":[{"name":"cstr"},{"name":"cstr"}],"output":{"name":"bool"}}],[11,"drop","","",2,{"inputs":[{"name":"self"}],"output":null}],[11,"as_object","","Exposes internal C-binding poitner",2,{"inputs":[{"name":"self"}],"output":{"name":"lto_module_t"}}],[11,"from_vec","","Builds a new object file.",2,{"inputs":[{"name":"vec"}],"output":{"name":"result"}}],[11,"from_slice","","Builds a new object file",2,null],[11,"get_buffer","","Internal method to steal inner buffer.",2,{"inputs":[{"name":"self"},{"name":"vec"}],"output":null}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"pic"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"default","","Return's `PIC::Default`",0,{"inputs":[],"output":{"name":"pic"}}],[11,"into","","",0,{"inputs":[{"name":"self"}],"output":{"name":"lto_codegen_model"}}],[11,"drop","","",3,{"inputs":[{"name":"self"}],"output":null}],[11,"new","","Construct a new Linker",3,null],[11,"add_file","","Link an Object File",3,{"inputs":[{"name":"self"},{"name":"objfile"}],"output":null}],[11,"add_buffer","","Link to a Object File (in a memory buffer)",3,{"inputs":[{"name":"self"},{"name":"object"}],"output":null}],[11,"link_to_mem","","Complete linking, return a memory buffer",3,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"link_to_file","","Complete linking, write to a file",3,{"inputs":[{"name":"self"},{"name":"p"}],"output":{"name":"result"}}],[8,"LinkerObject","","Unifying properties that all linker objects posses.",null,null],[10,"as_object","","",4,{"inputs":[{"name":"self"}],"output":{"name":"lto_module_t"}}],[11,"get_num_symbols","","Get the number of symbols in an object file",4,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"get_symbol_name","","Get the name of a symbol at a certain index",4,{"inputs":[{"name":"self"},{"name":"u32"}],"output":{"name":"result"}}],[11,"get_target_triple","","Get the target triple an object file was compiled for",4,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"get_num_symbols","","Get the number of symbols in an object file",4,{"inputs":[{"name":"self"}],"output":{"name":"u32"}}],[11,"get_symbol_name","","Get the name of a symbol at a certain index",4,{"inputs":[{"name":"self"},{"name":"u32"}],"output":{"name":"result"}}],[11,"get_target_triple","","Get the target triple an object file was compiled for",4,{"inputs":[{"name":"self"}],"output":{"name":"result"}}]],"paths":[[4,"PIC"],[3,"ObjFile"],[3,"Object"],[3,"Linker"],[8,"LinkerObject"]]};
initSearch(searchIndex);