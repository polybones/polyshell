use std::env;
use std::ffi::CString;
use std::iter;
use std::ptr;

use libc::{c_char, execve, fork, wait};

#[inline]
pub fn exec_external(path: CString, args: Vec<CString>) -> i32 {
    let argv: Vec<*const c_char> = iter::once(path.as_ptr())
        .chain(args.iter().map(|cs| cs.as_ptr()))
        .chain(iter::once(ptr::null()))
        .collect();
    // TODO: Will probably fix this to be better eventually.
    let env_strings: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();
    let env_vars: Vec<*const c_char> = env_strings.iter()
        .map(|cs| cs.as_ptr())
        .chain(iter::once(ptr::null()))
        .collect();

    match unsafe { fork() } {
        0 => {
            unsafe {
                execve(
                    path.as_ptr(),
                    argv.as_ptr(),
                    env_vars.as_ptr(),
                )
            }
        },
        _ => {
            let mut status = 0;
            unsafe { wait(&mut status); }
            status
        },
    }
}
