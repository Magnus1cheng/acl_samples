use libc::times;
use libc::{c_char, c_int, c_uint, c_void};
use std::ffi::CString;
use std::ffi::CStr;
use std::collections::BTreeMap;
use chrono::Utc;
//type aclrtContext = u64;
//type aclrtStream = u64;

pub mod acl_struct;
use crate::acl_struct::*;

pub static mut HANDLE_MAP: BTreeMap<u64, u64> = BTreeMap::new();

pub fn get_id() -> u64 {
    let dt = Utc::now();
    let timestamp: u64 = dt.timestamp() as u64;
    println!("generated timestamp {}", timestamp);
    return timestamp;
}

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
pub extern "C" fn aclmdlCreateDesc() -> u64 {
    println!("Hijacked aclmdlCreateDesc()");


    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlCreateDesc").unwrap();
    let orig_func: extern "C" fn() -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func();
    let ret_handle = get_id();
    unsafe { HANDLE_MAP.insert(ret_handle, ret); }
    return ret_handle;
}


    // let mut mem : Vec<u8> = Vec::with_capacity(1024);
    // mem.resize(1024, 0);
    // let addr = &mem[0] as * const_ u8 as u64;

#[no_mangle]
pub extern "C" fn aclmdlGetDesc(modelDesc: u64 /* aclmdlDesc* */, modelId: c_uint) -> c_int {
    println!("Hijacked aclmdlGetDesc()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetDesc").unwrap();
    let orig_func: extern "C" fn(u64, c_uint) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let real_desc = unsafe { HANDLE_MAP.get(&modelDesc).unwrap().clone() };
    let ret = orig_func(real_desc, modelId);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlGetInputSizeByIndex(modelDesc: u64 /* aclmdlDesc* */, index: usize) -> usize {
    println!("Hijacked aclmdlGetInputSizeByIndex()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetInputSizeByIndex").unwrap();
    let orig_func: extern "C" fn(u64, usize) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let real_desc = unsafe { HANDLE_MAP.get(&modelDesc).unwrap().clone() };
    let ret = orig_func(real_desc, index);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtMalloc(devPtr: *mut *mut c_void /* aclmdlDesc* */, size: usize, policy: aclrtMemMallocPolicy) -> c_int {
    println!("Hijacked aclrtMalloc()");

    // let mut mem : Vec<u8> = Vec::with_capacity(1024);
    // mem.resize(1024, 0);
    // let addr = &mem[0] as * const_ u8 as u64;

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtMalloc").unwrap();
    let orig_func: extern "C" fn(*mut *mut c_void, usize, aclrtMemMallocPolicy) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let mut addr_holder = 0 ;
    let mut addr_holder_ptr: u64 = &mut addr_holder as *mut _ as u64;
    //unsafe{ println!("addr 1 is {:x}", addr_holder_ptr)};
    let ret = orig_func(&mut addr_holder_ptr as *mut _ as  *mut *mut c_void, size, policy);
    unsafe { (*(devPtr as *mut u64)) = addr_holder_ptr} ;
    //unsafe{ println!("addr is {:x}", addr_holder_ptr)};

    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlCreateDataset() -> u64 {
    println!("Hijacked aclmdlCreateDataset()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlCreateDataset").unwrap();
    let orig_func: extern "C" fn() -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let ret = orig_func(); // btree map here

    return ret;
}


#[no_mangle]
pub extern "C" fn aclCreateDataBuffer(data: *mut c_void /* aclmdlDesc* */, size: usize) -> u64 {
    println!("Hijacked aclCreateDataBuffer()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclCreateDataBuffer").unwrap();
    let orig_func: extern "C" fn(*mut c_void, usize) -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let ret = orig_func(data, size); // btree map here

    return ret;
}


#[no_mangle]
pub extern "C" fn aclmdlAddDatasetBuffer(dataset: u64 /* aclmdlDataset* */, dataBuffer: u64) -> c_int {
    println!("Hijacked aclmdlAddDatasetBuffer()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlAddDatasetBuffer").unwrap();
    let orig_func: extern "C" fn(u64, u64) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let ret = orig_func(dataset, dataBuffer); // btree map here

    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlGetNumOutputs(modelDesc: u64 /* aclmdlDesc* */) -> usize {
    println!("Hijacked aclmdlGetNumOutputs()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetNumOutputs").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let real_desc = unsafe { HANDLE_MAP.get(&modelDesc).unwrap().clone() };
    let ret = orig_func(real_desc); // btree map here
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlGetOutputSizeByIndex(modelDesc: u64 /* aclmdlDesc* */, index: usize) -> usize {
    println!("Hijacked aclmdlGetOutputSizeByIndex()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetOutputSizeByIndex").unwrap();
    let orig_func: extern "C" fn(u64, usize) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&modelDesc).unwrap().clone() };
    let ret = orig_func(real_desc, index); // btree map here
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtMemcpy(dst: *mut c_void, destMax: usize, src: *const c_void, count: usize, kind: aclrtMemcpyKind) -> c_int {
    println!("Hijacked aclrtMemcpy()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtMemcpy").unwrap();
    let orig_func: extern "C" fn(*mut c_void, usize, *const c_void, usize, aclrtMemcpyKind) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let mut ret: c_int = 500000;
    // if destMax < count, maybe return error? no need to do copy
    match kind {
        aclrtMemcpyKind::ACL_MEMCPY_HOST_TO_HOST => {
            unsafe { std::ptr::copy_nonoverlapping(src as * const u8, dst as * mut u8, count); }
            ret = 0;
        }
        aclrtMemcpyKind::ACL_MEMCPY_HOST_TO_DEVICE => {
            let mut mem : Vec<u8> = Vec::with_capacity(count);
            mem.resize(count, 0);
            let kernel_addr = &mut mem[0] as * mut _ as u64;
            unsafe { core::intrinsics::copy_nonoverlapping(src, kernel_addr as * mut c_void, count); }
            ret = orig_func(dst, destMax, kernel_addr as *const c_void, count, kind);
        }
        aclrtMemcpyKind::ACL_MEMCPY_DEVICE_TO_HOST => {
            let mut mem : Vec<u8> = Vec::with_capacity(count);
            mem.resize(count, 0);
            let kernel_addr = &mut mem[0] as * mut _ as u64;
            ret = orig_func(kernel_addr as * mut _, count, src, count, kind);
            unsafe { core::intrinsics::copy_nonoverlapping(kernel_addr as *const _, dst, count); }
        }
        aclrtMemcpyKind::ACL_MEMCPY_DEVICE_TO_DEVICE => {
            ret = orig_func(dst, destMax, src, count, kind);
        }
    }
    //let ret = orig_func(modelDesc, index); // btree map here
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlExecute(modelId: c_uint, input: u64, output: u64) -> c_int {
    println!("Hijacked aclmdlExecute()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlExecute").unwrap();
    let orig_func: extern "C" fn(c_uint, u64, u64) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(modelId, input, output);
   
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlGetDatasetBuffer(dataset: u64, index: usize) -> u64 {
    println!("Hijacked aclmdlGetDatasetBuffer()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetDatasetBuffer").unwrap();
    let orig_func: extern "C" fn(u64, usize) -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(dataset, index);
   
    return ret;
}

#[no_mangle]
pub extern "C" fn aclGetDataBufferAddr(dataBuffer: u64) -> u64 {
    println!("Hijacked aclGetDataBufferAddr()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetDataBufferAddr").unwrap();
    let orig_func: extern "C" fn(u64) -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(dataBuffer);
   
    return ret;
}

#[no_mangle]
pub extern "C" fn aclGetDataBufferSizeV2(dataBuffer: u64 /*const aclDataBuffer * */) -> usize {
    println!("Hijacked aclGetDataBufferSizeV2()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetDataBufferSizeV2").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let ret = orig_func(dataBuffer);
   
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtMallocHost(hostPtr: *mut *mut c_void, size: usize) -> c_int {
    println!("Hijacked aclrtMallocHost()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtMallocHost").unwrap();
    let orig_func: extern "C" fn(*mut *mut c_void, usize) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    // todo!();
    let ret = orig_func(hostPtr, size);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtFreeHost(hostPtr: *const c_void) -> c_int {
    println!("Hijacked aclrtFreeHost()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtFreeHost").unwrap();
    let orig_func: extern "C" fn(*const c_void) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    // todo!();
    let ret = orig_func(hostPtr);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlGetDatasetNumBuffers(dataset: u64) -> usize {
    println!("Hijacked aclmdlGetDatasetNumBuffers()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlGetDatasetNumBuffers").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let ret = orig_func(dataset);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclmdlDestroyDesc(modelDesc: u64) -> usize {
    println!("Hijacked aclmdlDestroyDesc()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclmdlDestroyDesc").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&modelDesc).unwrap().clone() };
    let ret = orig_func(real_desc);
    return ret;
}
