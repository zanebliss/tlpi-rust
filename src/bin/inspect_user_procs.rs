use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::{env, process};

use nix::unistd::User;

#[derive(Debug)]
struct Proc {
    pid: String,
    name: String,
}

/// 12-1
///
/// Write a program that lists the process ID and command name for all the processes being run by
/// the user named in the program's command-line argument.
fn main() -> Result<(), Box<dyn Error>> {
    if env::args().len() != 2 {
        println!("Usage: inspect_user_procs username");
        return Ok(());
    }

    let uid;
    if let Some(user) = User::from_name(&env::args().nth(1).unwrap())? {
        uid = user.uid;
    } else {
        eprintln!("Error: user not found");
        process::exit(1);
    }

    let mut procs: Vec<Proc> = vec![];

    let read_dir = fs::read_dir("/proc")?;
    for dir_entry in read_dir {
        let entry = dir_entry?;
        let status = match File::open(format!("{}/{}", entry.path().to_string_lossy(), "status")) {
            Ok(file) => file,
            Err(_) => continue,
        };
        let pid = entry.file_name().into_string().unwrap();
        let mut name = String::from("");

        let reader = BufReader::new(status);
        for result in reader.lines() {
            let line = result?;

            if name.is_empty() {
                name = line.clone()
            }

            if line.contains(&uid.to_string()) {
                procs.push(
                    Proc {
                        pid,
                        name: String::from(name.split_once(':').unwrap().1.trim())
                    }
                );

                break
            }
        }
    }

    for proc in procs {
        println!("pid: {:?}, command name: {:?}", proc.pid, proc.name);
    }

    Ok(())
}
