use std::io;

/// 4-2
/// cp_null
///
/// Like `cp` but if the file to be copied contains holes (sequences of null
/// bytes), those are also copied to the target file.
fn main() -> io::Result<()> {
    let (from_f, to_f) = match (std::env::args().nth(1), std::env::args().nth(2)) {
        (Some(from), Some(to)) => (std::fs::File::open(from), std::fs::File::create(to)),
        _ => {
            println!("Please provide two file arguments");
            return Ok(());
        }
    };

    std::io::copy(&mut from_f?, &mut to_f?)?;

    Ok(())
}
