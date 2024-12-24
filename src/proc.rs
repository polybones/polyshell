use std::ffi::CString;
use std::ptr;

use libc::{c_char, execve, fork, wait};

pub fn exec_external(path: &str) -> i32 {
    let cmd = CString::new(path).unwrap();
    let argv: Vec<*const c_char> = vec![cmd.as_ptr(), ptr::null()];
    let colorterm = CString::new("COLORTERM=truecolor").unwrap();
    let env_vars = vec![colorterm.as_ptr(), ptr::null()];
    match unsafe { fork() } {
        0 => {
            unsafe {
                return execve(cmd.as_ptr(), argv.as_ptr(), env_vars.as_ptr());
            }
        },
        _ => unsafe {
            let mut status = 0;
            wait(&mut status);
            status
        },
    }
}
