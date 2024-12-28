use std::{env, iter};
use std::ffi::CString;
use std::ptr;

use libc::{c_char, execve, fork, wait};

pub fn exec_external(path: &str, args: Vec<CString>) -> i32 {
    let cmd = CString::new(path).unwrap();
    
    let argv: Vec<*const c_char> = iter::once(cmd.as_ptr())
        .chain(args.iter().map(|cs| cs.as_ptr()))
        .chain(iter::once(ptr::null()))
        .chain(iter::once(ptr::null()))
        .collect();
    // SAFETY: Create owned CStrings
    let env_strings: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();
    let env_vars: Vec<*const c_char> = env_strings.iter()
        .map(|cs| cs.as_ptr())
        .chain(iter::once(ptr::null()))
        .collect();
    
    match unsafe { fork() } {
        -1 => -1,
        0 => {
            unsafe {
                return execve(cmd.as_ptr(), argv.as_ptr(), env_vars.as_ptr());
            }
        },
        _ => {
            let mut status = 0;
            unsafe {
                wait(&mut status);
            }
            status
        },
    }
}
