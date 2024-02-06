//Maybe use u64 to indicate all type of pointers to avoid error

use libc::{int32_t, times};
use libc::{c_char, c_int, c_uint, c_void};
use std::ffi::CString;
use std::ffi::CStr;
use std::collections::BTreeMap;
use chrono::Utc;
use std::ptr;
use spin::Mutex;
//type aclrtContext = u64;
//type aclrtStream = u64;

pub mod acl_struct;
use crate::acl_struct::*;

  pub static mut HANDLE_MAP: BTreeMap<u64, u64> = BTreeMap::new();
  pub static mut ID: i32 = 1;
  pub static M: Mutex<i32> = Mutex::new(0);


pub fn get_id() -> u64 {
    // let dt = Utc::now();
    // let timestamp: u64 = dt.timestamp() as u64;
    M.lock();
    let mut ret = 0;
    unsafe {
      ret = ID;
      ID = ID+1;
    }
    println!("generated return value {}", ret);
    return ret as u64;
}

pub fn copy_str(orig_str: *const c_char) -> Vec<u8>{
    let c_str: &CStr = unsafe { CStr::from_ptr(orig_str)}; 
    c_str.to_bytes().to_vec()
}

#[no_mangle]
pub extern "C" fn aclInit(path: *const c_char) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(path)};
    println!("Hijacked aclInit({})", c_str.to_str().unwrap());

    let lib = CString::new("/home/HwHiAiUser/Ascend/ascend-toolkit/latest/lib64/libascendcl.so").unwrap();
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
    println!("orig func is {:?}", orig_func);
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

#[no_mangle]
pub extern "C" fn aclrtGetRunMode(runMode: *mut aclrtRunMode) -> c_int {
    println!("Hijacked aclmdlDestroyDesc()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtGetRunMode").unwrap();
    let orig_func: extern "C" fn(*mut aclrtRunMode) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let mut aclrtRunMode = aclrtRunMode::ACL_HOST;
    let ret = orig_func(&mut aclrtRunMode as *mut aclrtRunMode);
    println!("run mode is {:?}", aclrtRunMode);
    unsafe { *runMode = aclrtRunMode} ;
    return ret;
}

#[no_mangle]
pub extern "C" fn aclopSetModelDir(path: *const c_char) -> c_int {
    let c_str: &CStr = unsafe { CStr::from_ptr(path)};
    println!("Hijacked aclopSetModelDir({})", c_str.to_str().unwrap());

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclopSetModelDir").unwrap();
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
    return ret;
}

#[no_mangle]
pub extern "C" fn aclopCreateAttr() -> u64 {
    println!("Hijacked aclmdlDestroyDesc()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclopCreateAttr").unwrap();
    let orig_func: extern "C" fn() -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let ret = orig_func();
    return ret;
}

#[no_mangle]
pub extern "C" fn aclCreateTensorDesc(dataType: aclDataType, numDims: i32, dims: *const i64, format: aclFormat) -> u64 {
    println!("Hijacked aclCreateTensorDesc()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclCreateTensorDesc").unwrap();
    let orig_func: extern "C" fn(aclDataType, i32, *const i64, aclFormat) -> u64 = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };
    let mut ret: u64 = 0;
    if numDims != 0 {
    let dims_total_bytes: usize = (numDims * 8) as usize; // i64 has 8 bytes
    let mut mem : Vec<u8> = Vec::with_capacity(dims_total_bytes);
    mem.resize(dims_total_bytes, 0);
    let addr = &mut mem[0] as *mut _ as u64;
    unsafe { core::intrinsics::copy_nonoverlapping(dims, addr as *mut i64, numDims as usize); }

        ret = orig_func(dataType, numDims, addr as *const i64, format);
    } else {
        ret = orig_func(dataType, 0, ptr::null(), format);
    }
    let ret_handle = get_id();
    unsafe { HANDLE_MAP.insert(ret_handle, ret); }
    println!("add tensor desc {:x} -> {:x}", ret_handle, ret);
    return ret_handle;
}

#[no_mangle]
pub extern "C" fn aclGetTensorDescSize(desc: u64) -> usize {
    println!("Hijacked aclGetTensorDescSize()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetTensorDescSize").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&desc).unwrap().clone() };
    let ret = orig_func(real_desc);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclGetTensorDescNumDims(desc: u64) -> usize {
    println!("Hijacked aclGetTensorDescNumDims()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetTensorDescNumDims").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&desc).unwrap().clone() };
    let ret = orig_func(real_desc);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclGetTensorDescElementCount(desc: u64) -> usize {
    println!("Hijacked aclGetTensorDescElementCount()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetTensorDescElementCount").unwrap();
    let orig_func: extern "C" fn(u64) -> usize = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&desc).unwrap().clone() };
    let ret = orig_func(real_desc);
    return ret;
}

#[no_mangle]
pub extern "C" fn aclGetTensorDescDimV2(desc: u64, index: usize, dimSize: *mut i64) -> c_int {
    println!("Hijacked aclGetTensorDescDimV2()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetTensorDescDimV2").unwrap();
    let orig_func: extern "C" fn(u64, usize, *mut i64) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&desc).unwrap().clone() };
    let mut dim_holder:i64 = 0;
    let ret = orig_func(real_desc, index, &mut dim_holder as *mut i64);
    unsafe {*dimSize = dim_holder};

    return ret;
}


#[no_mangle]
pub extern "C" fn aclGetTensorDescType(desc: u64) -> aclDataType {
    println!("Hijacked aclGetTensorDescType()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclGetTensorDescType").unwrap();
    let orig_func: extern "C" fn(u64) -> aclDataType = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let real_desc = unsafe { HANDLE_MAP.get(&desc).unwrap().clone() };
    let ret = orig_func(real_desc);
    return ret;
}


#[no_mangle]
pub extern "C" fn aclopCompileAndExecute(opType: u64, numInputs: i32, inputDesc: u64, inputs: u64,
                                        numOutputs: i32, outputDesc: u64, outputs: u64, attr: u64,
                                        engineType: i32, compileFlag: i32, opPath:u64 , stream: u64) -> c_int {
    println!("Hijacked aclopCompileAndExecute()");

    let lib = CString::new("libacl_op_compiler.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclopCompileAndExecute").unwrap();
    let orig_func: extern "C" fn(u64, i32, u64 , u64, i32, u64, u64, 
        u64, i32, i32, u64, u64) -> c_int = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    //opType
    let type_str_vec: Vec<u8> = copy_str(opType as *const c_char);
    let type_string = unsafe{ CString::from_vec_unchecked(type_str_vec.clone())};

    // const aclTensorDesc *const inputDesc[]
    let inputDesc_total_bytes: usize = (numInputs * 8) as usize; // u64 has 8 bytes
    let mut inputDesc_mem : Vec<u8> = Vec::with_capacity(inputDesc_total_bytes);
    inputDesc_mem.resize(inputDesc_total_bytes, 0);
    let inputDesc_addr = &mut inputDesc_mem[0] as *mut _ as u64;

    //unsafe{ println!("inputDesc {:x}", *(inputDesc as *const u64)) };
    for i in 0..numInputs {
        let real_desc = unsafe { HANDLE_MAP.get(&(*(inputDesc as *const u64).add(i as usize))).unwrap().clone() };
        unsafe { core::intrinsics::copy_nonoverlapping(&real_desc, (inputDesc_addr as *mut u64).add(i as usize) as *mut u64, 1); }
    }

    // const aclDataBuffer *const inputs[]
    let inputs_buffer_ptr_total_bytes: usize = (numInputs * 8) as usize;
    let mut inputs_buffer_ptr_mem : Vec<u8> = Vec::with_capacity(inputs_buffer_ptr_total_bytes);
    inputs_buffer_ptr_mem.resize(inputs_buffer_ptr_total_bytes, 0);
    let inputs_buffer_ptr_addr = &mut inputs_buffer_ptr_mem[0] as *mut _ as u64;
    unsafe { core::intrinsics::copy_nonoverlapping(inputs as *const u64, inputs_buffer_ptr_addr as *mut u64, numInputs as usize); }

    // const aclTensorDesc *const outputDesc[]
    let outputDesc_total_bytes: usize = (numOutputs * 8) as usize;
    let mut outputDesc_mem : Vec<u8> = Vec::with_capacity(outputDesc_total_bytes);
    outputDesc_mem.resize(outputDesc_total_bytes, 0);
    let outputDesc_addr = &mut outputDesc_mem[0] as *mut _ as u64;
    for i in 0..numOutputs {
        let real_desc = unsafe { HANDLE_MAP.get(&(*(outputDesc as *const u64).add(i as usize))).unwrap().clone() };
        unsafe { core::intrinsics::copy_nonoverlapping(&real_desc, (outputDesc_addr as *mut u64).add(i as usize) as *mut u64, 1); }
    }

    //aclDataBuffer *const outputs[],
    let outputs_buffer_ptr_total_bytes: usize = (numOutputs * 8) as usize;
    let mut outputs_buffer_ptr_mem : Vec<u8> = Vec::with_capacity(outputs_buffer_ptr_total_bytes);
    outputs_buffer_ptr_mem.resize(outputs_buffer_ptr_total_bytes, 0);
    let outputs_buffer_ptr_addr = &mut outputs_buffer_ptr_mem[0] as *mut _ as u64;
    unsafe { core::intrinsics::copy_nonoverlapping(outputs as *const u64, outputs_buffer_ptr_addr as *mut u64, numOutputs as usize); }

    let mut opPath_ptr:*const c_char= ptr::null();
    //opPath
    if !(opPath as *const c_void).is_null() {
        let opPath_str_vec: Vec<u8> = copy_str(opPath as *const c_char);
        let opPath_string = unsafe{ CString::from_vec_unchecked(opPath_str_vec.clone())};
        opPath_ptr = opPath_string.as_ptr();
    }
    let ret = orig_func(type_string.as_ptr() as u64, numInputs, inputDesc_addr as u64, inputs_buffer_ptr_addr as u64, numOutputs,
    outputDesc_addr as u64, outputs_buffer_ptr_addr as u64, attr, engineType, compileFlag, opPath_ptr as u64, stream);
    
    return ret;
}

#[no_mangle]
pub extern "C" fn aclrtSynchronizeStream(stream: aclrtStream) -> aclDataType {
    println!("Hijacked aclrtSynchronizeStream()");

    let lib = CString::new("libascendcl.so").unwrap();
    let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
    let func_name = CString::new("aclrtSynchronizeStream").unwrap();
    let orig_func: extern "C" fn(u64) -> aclDataType = unsafe {
        std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
    };

    let ret = orig_func(stream);
    return ret;
}


// #[no_mangle]
// pub extern "C" fn aclopCompileAndExecute(opType: u64, numInputs: i32, inputDesc: u64, inputs: u64,
//                                         numOutputs: i32, outputDesc: u64, outputs: u64, attr: u64,
//                                         engineType: i32, compileFlag: i32, opPath:u64 , stream: u64) -> c_int {
//     println!("Hijacked aclopCompileAndExecute()");
//     let lib = CString::new("/home/HwHiAiUser/Ascend/ascend-toolkit/7.0.RC1/aarch64-linux/lib64/libacl_op_compiler.so").unwrap();
//     let handle = unsafe { libc::dlopen(lib.as_ptr(), libc::RTLD_LAZY) };    
//     let func_name = CString::new("aclopCompileAndExecute").unwrap();
//     let orig_func: extern "C" fn(u64, i32, u64 , u64, i32, u64, u64, 
//         u64, i32, i32, u64, u64) -> c_int = unsafe {
//         std::mem::transmute(libc::dlsym(handle, func_name.as_ptr()))
//     };
//     println!("orig func {:?}", orig_func);
//     let ret = orig_func(opType, numInputs, inputDesc, inputs, numOutputs,
//         outputDesc, outputs, attr, engineType, compileFlag, opPath, stream);
//     println!(" ret val is {}", ret);
//     return ret;
// }

// aclopCreateKernel
// aclopCompile
// aclopCompileAndExecute