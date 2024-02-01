use libc::{c_char, c_int, c_uint};
use std::ffi::CString;
use std::ffi::CStr;
//type aclrtContext = u64;
//type aclrtStream = u64;

pub mod acl_struct;
use crate::acl_struct::*;

pub fn copy_str(orig_str: *const c_char) -> Vec<u8>{
    let c_str: &CStr = unsafe { CStr::from_ptr(orig_str)}; 
    c_str.to_bytes().to_vec()
}

#[no_mangle]
pub extern "C" fn aclInit(path: *const c_char) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(path)};
    println!("Hijacked aclInit({})", c_str.to_str().unwrap());

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclInit").unwrap();
    let orig_func: extern "C" fn(*const c_char) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let str_vec: Vec<u8> = copy_str(path);
    //todo!();
    // copy Vector<u8> from user mode to kernel mode.
    // maybe check str_vec length and decide if pass null or actual string.
    //CString::new("").unwrap().as_ptr()
    let string = unsafe{ CString::from_vec_unchecked(str_vec.clone())};
    let ret = orig_func(string.as_ptr());
    unsafe {println!("val is {:?}", CString::from_vec_unchecked(str_vec))};
    println!("return val is {}", ret);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtSetDevice(deviceId: c_int) -> c_int {
    println!("Hijacked aclrtSetDevice({})", deviceId);

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtSetDevice").unwrap();
    let orig_func: extern "C" fn(c_int) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(deviceId);
    println!("return val is {}", ret);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtCreateContext(context: aclrtContext ,deviceId: c_int) -> c_int {
    println!("Hijacked aclrtCreateContext({:x})", context);

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtCreateContext").unwrap();
    let orig_func: extern "C" fn(u64, c_int) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(context, deviceId);
    println!("return val is {}", ret);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtCreateStream(stream: aclrtStream) -> c_int {
    println!("Hijacked aclrtStream({:x})", stream);

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtCreateStream").unwrap();
    let orig_func: extern "C" fn(u64) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(stream);
    println!("return val is {}", ret);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlLoadFromFile(modelPath: *const c_char, modelId: *mut c_uint) -> c_int {
    println!("Hijacked aclmdlLoadFromFile()");

    // let mut mem : Vec<u8> = Vec::with_capacity(1024);
    // mem.resize(1024, 0);
    // let addr = &mem[0] as * const_ u8 as u64;

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlLoadFromFile").unwrap();
    let orig_func: extern "C" fn(*const c_char, *mut c_uint) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let str_vec: Vec<u8> = copy_str(modelPath);
    //todo!();
    // copy Vector<u8> from user mode to kernel mode.
    let string = unsafe{ CString::from_vec_unchecked(str_vec)};

    let mut addr_holder: c_uint = 0 ;
    unsafe{ println!("addr is {:x}", addr_holder)};
    let ret = orig_func(string.as_ptr(), &mut addr_holder as *mut _);
    unsafe{ println!("addr is {:x}", addr_holder)};
    unsafe { *modelId = addr_holder as c_uint} ;
    println!("return val is {}", ret);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlCreateDesc() -> *mut aclmdlDesc {
    println!("Hijacked aclmdlCreateDesc()");

    // let mut mem : Vec<u8> = Vec::with_capacity(1024);
    // mem.resize(1024, 0);
    // let addr = &mem[0] as * const_ u8 as u64;

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlCreateDesc").unwrap();
    let orig_func: extern "C" fn() -> *mut aclmdlDesc = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func();
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlGetDesc(modelDesc: u64, modelId: c_uint) -> c_int {
    println!("Hijacked aclmdlGetDesc()");

    // let mut mem : Vec<u8> = Vec::with_capacity(1024);
    // mem.resize(1024, 0);
    // let addr = &mem[0] as * const_ u8 as u64;

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetDesc").unwrap();
    let orig_func: extern "C" fn(u64, c_uint) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(modelDesc, modelId);
    return ret;
}