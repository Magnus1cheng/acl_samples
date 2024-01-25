use libc::{c_char, c_int};
use std::ffi::CString;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn aclInit(path: *const c_char) -> c_int {
    let c_str = unsafe { CStr::from_ptr(path)};
    println!("Hijacked aclInit({})", c_str.to_str().unwrap());

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclInit").unwrap();
    let orig_func: extern "C" fn(*const c_char) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(path);
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
pub extern "C" fn aclrtCreateContext(aclrtContext: u64 ,deviceId: c_int) -> c_int {
    println!("Hijacked aclrtCreateContext({:x})", aclrtContext);

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtCreateContext").unwrap();
    let orig_func: extern "C" fn(u64, c_int) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(aclrtContext, deviceId);
    println!("return val is {}", ret);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtCreateStream(aclrtStream: u64) -> c_int {
    println!("Hijacked aclrtStream({:x})", aclrtStream);

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtCreateStream").unwrap();
    let orig_func: extern "C" fn(u64) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(aclrtStream);
    println!("return val is {}", ret);
    return ret;
}