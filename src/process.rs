use std::env;
use std::ffi::CString;
use std::iter;
use std::mem;
use std::ptr;

use anyhow::{anyhow, Result};
use libc::{c_char, execve, fork, sigaction, sigemptyset, sigset_t, wait, SIG_DFL, SIGINT, SIGQUIT};

pub fn exec_external(path: &str, args: Vec<CString>) -> Result<()> {
    let cmd = CString::new(path).unwrap();
    
    let argv: Vec<*const c_char> = iter::once(cmd.as_ptr())
        .chain(args.iter().map(|cs| cs.as_ptr()))
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
        0 => {
            let mut set: sigset_t = unsafe { mem::zeroed() };
            unsafe { sigemptyset(&mut set) };

            let mut action: sigaction = unsafe { mem::zeroed() };
            action.sa_sigaction = SIG_DFL;
            action.sa_mask = set;       
            action.sa_flags = 0;        
            let signals = [
                SIGINT,
                SIGQUIT,
            ];

            for &sig in signals.iter() {
                if unsafe { sigaction(sig, &action, std::ptr::null_mut()) } != 0 {
                    return Err(anyhow!("failed to set up signal handler for signal {}", sig).into());
                }
            }
            let status_code = unsafe { execve(cmd.as_ptr(), argv.as_ptr(), env_vars.as_ptr()) };
            if status_code == 0 {
                return Ok(());
            }
            else {
                return Err(anyhow!("status code {status_code}"));
            }
        },
        _ => {
            let mut status_code = 0;
            unsafe {
                wait(&mut status_code);
            }
            if status_code == 0 {
                return Ok(());
            }
            else {
                return Err(anyhow!("status code {status_code}"));
            }
        },
    }
}
