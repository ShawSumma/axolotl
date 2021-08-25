use libc::c_char;
#[cfg(target_family = "unix")]
use nix::{
    fcntl::OFlag,
    sys::mman::{mmap, shm_open, MapFlags, ProtFlags},
    sys::stat::Mode,
    unistd::{execv, ftruncate},
};
use serde::{de::Deserialize, ser::Serialize};

use std::collections::HashMap;
use std::ffi::CStr;
use std::path::Path;
use std::str;

#[cfg(target_family = "windows")]
pub extern "C" fn axolotl_init() -> CVec {
    unimplemented!()
}

/// returns the data contained in the shared memory object
/// which will contain meta information, and the data of the
/// super global variables
#[cfg(target_family = "unix")]
pub unsafe fn axolotl_init<'a, T: Deserialize<'a>>() -> T {
    let filename = format!("{}-axolotlenv", std::env::current_exe().unwrap().display());
    let descriptor = shm_open(filename.as_str(), OFlag::O_RDONLY, Mode::empty()).unwrap();
    let bufptr = mmap(
        std::ptr::null_mut(),
        1000,
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        MapFlags::MAP_SHARED,
        descriptor,
        0,
    )
    .unwrap();
    let c_str: &CStr = unsafe { CStr::from_ptr(bufptr as *mut i8) };
    let str_slice: &str = c_str.to_str().unwrap();
    ron::from_str(str_slice).unwrap()
}

#[cfg(target_family="unix")]
pub unsafe fn axolotl_exec<T: Serialize, P: AsRef<Path>, S: AsRef<str>>(value: &T, executable: P, args: &[S]) {
    let descriptor = shm_open(format!("{}-axolotlenv", executable.as_ref().display()).as_str(), OFlag::O_CREAT | OFlag::O_RDWR, Mode::S_IRWXU).unwrap();
    //let mut data = Vec::with_capacity(1000);
    let serialized = ron::to_string(value).unwrap().into_bytes();
    if serialized.len() > 999 {
        panic!("data too large");
    }
    //data[0..serialized.len()] = serialized[..];
    ftruncate(descriptor, 1000);
    let buffer: *mut u8 = mmap(std::ptr::null_mut(), 1000, ProtFlags::PROT_READ | ProtFlags::PROT_WRITE, MapFlags::MAP_SHARED, descriptor, 0).unwrap() as *mut u8;
    let mut data = Vec::from_raw_parts(buffer, 1000, 1000);
    for (num, d) in serialized.iter().enumerate() {
        data[num] = *d;
    }
    data[serialized.len()] = b'\0';
    //execv(path, args.iter().map(|v| {CStr::from_bytes_with_nul_unchecked(CString::new(v).expect("failed to convert argument to CString"))}).collect::<CStr>());
}