use std::fmt::Display;
use std::str::Utf8Error;
use std::{env::args, ffi::CStr, io};

use nix::libc;
use nix::libc::{endpwent, passwd};

struct Password<'a> {
    pw_name: &'a str,
    pw_uid: u32,
    pw_gid: u32,
    pw_dir: &'a str,
    pw_gecos: &'a str,
    pw_passwd: &'a str,
    pw_shell: &'a str,
}

impl Display for Password<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(pw_name={} pw_passwd={} pw_uid={} pw_gid={} pw_dir={} pw_gecos={} pw_shell{})",
            self.pw_name,
            self.pw_passwd,
            self.pw_uid,
            self.pw_gid,
            self.pw_dir,
            self.pw_gecos,
            self.pw_shell,
        )
    }
}

fn build_passwd<'a>(passwd: *const passwd) -> Result<Password<'a>, Utf8Error> {
    unsafe {
        Ok(Password {
            pw_name: CStr::from_ptr((*passwd).pw_name).to_str()?,
            pw_passwd: CStr::from_ptr((*passwd).pw_passwd).to_str()?,
            pw_gecos: CStr::from_ptr((*passwd).pw_gecos).to_str()?,
            pw_uid: (*passwd).pw_uid,
            pw_gid: (*passwd).pw_gid,
            pw_dir: CStr::from_ptr((*passwd).pw_dir).to_str()?,
            pw_shell: CStr::from_ptr((*passwd).pw_shell).to_str()?,
        })
    }
}

fn getpwent(name: &str) -> Result<Option<Password>, Utf8Error> {
    let mut current_pw: *mut passwd;
    let mut passwd: Option<Password> = None;

    unsafe {
        loop {
            current_pw = libc::getpwent();

            if name == CStr::from_ptr((*current_pw).pw_name).to_str()? {
                passwd = Some(build_passwd(current_pw)?);

                break;
            }

            if libc::getpwent().is_null() {
                break;
            }
        }
        endpwent();
    }

    return Ok(passwd);
}

/// 8-1
/// getpwname
///
/// Implement getpwname() using setpwent(), getpwent(), and endpwent()
fn main() -> io::Result<()> {
    if let Some(pw_name) = args().nth(1) {
        match getpwent(&pw_name).unwrap() {
            Some(passwd) => println!("{}", passwd),
            None => println!("No match!")
        }
    } else {
        println!("Usage: getpwname name");
    }

    Ok(())
}
