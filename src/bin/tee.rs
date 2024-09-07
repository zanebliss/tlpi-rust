use std::{
    env::args,
    fs::OpenOptions,
    io::{stdin, Write},
};

/// 4-1
/// tee
///
/// Reads stdin until EOF, writing a copy of the input to stdout and to the file passed as an arg.
/// By default it replaces existing files, unless -a is used. If -a is used, it appends stdin to
/// the existing file.
fn main() -> std::io::Result<()> {
    let mut input = String::new();
    let file_name = match args().nth(1) {
        Some(name) => name,
        None => {
            eprintln!("please provide a filename");
            return Ok(());
        }
    };

    let append_flag = if let Some(arg) = args().nth(2) {
        arg == "-a"
    } else {
        false
    };

    let mut buffer = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append_flag)
        .open(&file_name)?;

    stdin().read_line(&mut input)?;

    match buffer.write_all(input.as_bytes()) {
        Ok(..) => {
            print!("{input}")
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
