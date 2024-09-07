use std::{
    env::args,
    fs::File,
    io::{self, Seek, SeekFrom, Write},
};

const START_OFFSET: u64 = 0;

/// 5-2
/// open_seek
///
/// opens an existing file for writing and seeks to the beginning before
/// writing some data
fn main() -> io::Result<()> {
    if args().len() == 1 {
        println!("Must provide a file");
        return Ok(());
    }

    let path = args().last().unwrap();

    let mut fd = File::options().append(true).open(path)?;

    fd.seek(SeekFrom::Start(START_OFFSET))?;

    fd.write(b"Hello, World!")?;

    Ok(())
}
