use std::{ffi::CStr, mem, ptr};

use libc::{getpwuid_r, getuid, passwd};

pub unsafe fn get_username() -> Option<String> {
    let mut buf = [0; 2048];
    let mut result = ptr::null_mut();
    let mut passwd: passwd = mem::zeroed();

    let getpwuid_r_code = getpwuid_r(
        getuid(),
        &mut passwd,
        buf.as_mut_ptr(),
        buf.len(),
        &mut result,
    );

    if getpwuid_r_code == 0 && !result.is_null() {
        let username = CStr::from_ptr(passwd.pw_name);
        let username = String::from_utf8_lossy(username.to_bytes());

        Some(username.into())
    } else {
        None
    }
}
