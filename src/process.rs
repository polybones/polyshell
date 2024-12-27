use std::ffi::CString;
use std::ptr;

use libc::{c_char, execve, fork, wait};

pub fn exec_external(path: &str, args: Vec<CString>) -> i32 {
    let cmd = CString::new(path).unwrap();
    let mut argv: Vec<*const c_char> = vec![cmd.as_ptr()];
    args.iter().for_each(|arg| {
        argv.push(arg.as_ptr());
    });
    argv.push(ptr::null());
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
