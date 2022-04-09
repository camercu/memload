mod daemonize;
pub use daemonize::*;

use nix::sys::memfd::{self, MemFdCreateFlag};
use nix::unistd;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::AsRawFd;

pub fn run_from_mem(bin: &[u8], argv: &[CString], env: &[(&CStr, &CStr)]) {
    // create a new in-memory file to hold the binary data
    let name = CString::new("").unwrap();
    let fd = memfd::memfd_create(&name, MemFdCreateFlag::MFD_CLOEXEC).unwrap();

    // copy the binary data into the in-mem file
    let mut file = unsafe { File::from_raw_fd(fd) };
    file.write_all(bin).unwrap();

    // prepare env as C-style string array
    let env: Vec<CString> = env
        .iter()
        .map(|(k, v)| CString::new([k.to_bytes(), b"=", v.to_bytes()].concat()).unwrap())
        .collect();

    // execute the file
    unistd::fexecve(file.as_raw_fd(), argv, &env).unwrap();
}
