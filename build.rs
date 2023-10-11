use std::{fs::{self, File}, io::{self, Write}, path::Path};
use io::ErrorKind::AlreadyExists;

fn main() -> io::Result<()> {
    match fs::create_dir("benchfiles") {
        Ok(_) => {},
        Err(e) => match e.kind() {
            AlreadyExists => {},
            _ => panic!("Can't create dir 'benchfiles': err {e}"),
        },
    };

    let short_file = Path::new("benchfiles/short_file");
    match File::create(short_file) {
        Err(err) => panic!("couldn't open {}: {}", short_file.display(), err),
        Ok(mut file) => file.write_all("0123456\n0123456789".repeat(2usize.pow(9)).as_bytes())?,
    };

    let short_lines = Path::new("benchfiles/short_lines");
    match File::create(short_lines) {
        Err(err) => panic!("couldn't open {}: {}", short_lines.display(), err),
        Ok(mut file) => file.write_all("01234\n0123456789ABCDEF\n0123456789ABCDEF0123456789".repeat(2usize.pow(20)).as_bytes())?,
    };

    let long_lines = Path::new("benchfiles/long_lines");
    match File::create(long_lines) {
        Err(err) => panic!("couldn't open {}: {}", long_lines.display(), err),
        Ok(mut file) => file.write_all(
            "0123456789ABCDEF0123456789ABCDEF0123456789\n0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF\n0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789"
            .repeat(2usize.pow(19)).as_bytes())?,
    };


    Ok(())
}

