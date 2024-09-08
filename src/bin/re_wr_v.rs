use core::str;
use std::{fs::File, io::{self, IoSlice, IoSliceMut, Read, Seek, Write}};

/// 5-7 
///
/// re_wr_v
/// Implement readv and writev
fn main() -> io::Result<()> {
    let mut file = File::options().write(true).read(true).open("file")?;

    let data1 = IoSlice::new("Hello, World!\n".as_bytes());
    let data2 = IoSlice::new("Fizz, buzz!\n".as_bytes());
    let data3 = IoSlice::new("Foo, bar!\n".as_bytes());
    let data4 = IoSlice::new("Zip, zap!".as_bytes());

    file.write_vectored(&[data1, data2, data3, data4])?;

    let mut buf1= [0; 32];
    let mut buf2= [0; 32];

    file.seek(io::SeekFrom::Start(0))?;

    file.read_vectored(&mut [IoSliceMut::new(&mut buf1), IoSliceMut::new(&mut buf2)])?;

    println!("{:?}", str::from_utf8(&buf1).unwrap());
    println!("{:?}", str::from_utf8(&buf2).unwrap());

    Ok(())
}
