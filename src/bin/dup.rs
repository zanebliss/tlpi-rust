use std::{
    env::args,
    fs::File,
    io::{self, Seek},
    os::fd::{AsRawFd, FromRawFd, RawFd},
};

use nix::{
    errno::Errno,
    fcntl::{
        fcntl, FcntlArg::{self},
    }, unistd::close,
};

fn dup(oldfd: RawFd) -> Result<i32, Errno> {
    fcntl(oldfd, FcntlArg::F_DUPFD(oldfd))
}

// I don't know of a way to guarantee atomicity outside of using mutexes. The exercise is
// specifically for implementing it without using the actual system call.
fn dup2(oldfd: i32, newfd: i32) -> Result<i32, Errno> {
    close(newfd)?;
    let newfd = fcntl(oldfd, FcntlArg::F_DUPFD(newfd))?;

    Ok(newfd)
}

// fn dup2(oldfd, newfd) -> i32 {
// }

/// 5.4 dup and dup2
///
/// implement dup() and dup2() using fcntl() and where necessary close()
fn main() -> io::Result<()> {
    if args().len() != 2 {
        println!("Usage: dup file");
        return Ok(());
    }

    let mut file = File::open(args().nth(1).unwrap())?;
    file.seek(io::SeekFrom::Start(300))?;

    let newfd = dup(file.as_raw_fd().into())?;
    let mut new_file = unsafe { File::from_raw_fd(newfd) };

    println!("fd: {:?}, newfd: {:?}", file.stream_position()?, new_file.stream_position()?);

    dup2(24, newfd)?;

    Ok(())
}
