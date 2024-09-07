use std::{env::args, error::Error, fs::File, io::{Seek, Write}};

/// 5-3
/// atomic_append
fn main() -> Result<(), Box<dyn Error>> {
    if args().len() < 3 {
        println!("Usage: atomic_append filename num-bytes [x]");
        return Ok(());
    }

    let fname = args().nth(1).unwrap();
    let num_bytes: u32 = args().nth(2).unwrap().parse()?;
    let append_flag = if let Some(_) = args().nth(3) {
        false
    } else {
        true
    };

    let mut fd = File::options().append(append_flag).write(true).open(fname)?;

    for _ in 1..=num_bytes {
        if append_flag {
            fd.write_all(b"x")?;
        } else {
            fd.seek(std::io::SeekFrom::End(0))?;
            fd.write(b"x")?;
        }
    }

    Ok(())
}

