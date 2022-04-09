use nix::errno::Errno;
use nix::libc;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, SigmaskHow, Signal};
use nix::sys::stat::{self, Mode};
use nix::unistd::{self, ForkResult, SysconfVar};
use std::env;
use std::fs::File;
use std::os::unix::prelude::AsRawFd;
use std::path::Path;
use std::process;

const EXIT_SUCCESS: i32 = 0;

/// Daemonize a *nix process using "old-style" SysV process.
/// ref: https://www.freedesktop.org/software/systemd/man/daemon.html
pub fn daemonize() {
    // perform first fork
    do_fork(true);

    // detach child from terminal and create independent session
    unistd::setsid().expect("setsid failed");

    // perform second fork, preventing reacquisition of terminal
    // exiting parent here sends child's ppid to 1 (init)
    do_fork(true);

    // change working dir to root
    let root = Path::new("/");
    env::set_current_dir(&root).expect("chdir failed");

    // reset umask
    stat::umask(Mode::empty());

    // restore default signal handling
    reset_sighandlers();

    // close all open file descriptors except stdin, stdout, stderr
    close_fds();

    // redirect stdin, stdout, stderr to /dev/null
    let devnull = File::open("/dev/null").expect("failed to open /dev/null");
    unistd::dup2(devnull.as_raw_fd(), libc::STDIN_FILENO).expect("dup2 failed");
    unistd::dup2(devnull.as_raw_fd(), libc::STDOUT_FILENO).expect("dup2 failed");
    unistd::dup2(devnull.as_raw_fd(), libc::STDERR_FILENO).expect("dup2 failed");
}

/// Perform a fork, panicking on failure. If `kill_parent` is `true`, exits the
/// parent immediately after fork completes.
fn do_fork(kill_parent: bool) -> ForkResult {
    let result = unsafe { unistd::fork().expect("fork failed") };

    if let ForkResult::Parent { .. } = result {
        if kill_parent {
            process::exit(EXIT_SUCCESS);
        }
    }

    result
}

/// Close all file descriptors except stdin, stdout, stderr.
fn close_fds() {
    let max_fd = unistd::sysconf(SysconfVar::OPEN_MAX)
        .expect("sysconf failed")
        .expect("no limit to fds") as i32;

    for fd in 3..max_fd {
        unistd::close(fd).unwrap_or(()); // ignore close errors
    }

    Errno::clear(); // clear any errors from close failures
}

/// Set all signal handlers to default and restore signal mask. Ignores errors.
fn reset_sighandlers() {
    let default = SigAction::new(SigHandler::SigDfl, SaFlags::empty(), SigSet::empty());

    for sig in Signal::iterator() {
        // set signal handlers to default, ignoring errors
        unsafe { signal::sigaction(sig, &default).unwrap_or(default) };
    }

    Errno::clear(); // clear any errors from signaction failures

    // also reset the signal mask
    signal::sigprocmask(SigmaskHow::SIG_SETMASK, Some(&SigSet::empty()), None).unwrap_or(());
}
