use std::{env::args, fs::File, io::{self, Seek, Write}};

/// 5.5
///
/// validate_dup
/// Write a program to verify that duplicated file descriptors share a file offset value and open
/// file status flags
fn main() -> io::Result<()> {
    if args().len() != 2 {
        println!("Usage: validate_dup file");
        return Ok(())
    }

    let path = args().last().unwrap();

    let mut file = File::options().write(true).read(true).open(path)?;

    for _ in 0..=500 {
        file.write(b"x")?;
    }

    file.seek(io::SeekFrom::Start(250))?;

    let mut newfile = file.try_clone()?;

    println!("File permissions: {:?}, offset: {}", file.metadata()?.permissions(), file.stream_position()?);
    println!("Newfile permissions: {:?}, offset: {}", newfile.metadata()?.permissions(), newfile.stream_position()?);

    Ok(())
}
