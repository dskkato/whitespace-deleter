use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter, Write};

const BUF_SIZE: usize = 8 * 1024 * 1024;

fn main() -> io::Result<()> {
    let fname: Vec<String> = env::args().skip(1).collect();
    if fname.len() != 2 {
        panic!("sample SRC OUT");
    };

    let (src, out) = (&fname[0], &fname[1]);
    if src == out {
        panic!("SRC and OUT need to be different names");
    }

    let src = File::open(src)?;
    let reader = BufReader::with_capacity(BUF_SIZE, src);

    let out = File::create(out)?;
    let mut writer = BufWriter::with_capacity(BUF_SIZE, out);

    for line in reader.lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.splitn(3, ' ').collect();
        write!(writer, "{},{}", &v[0], &v[0])?;
        for c in v[2].chars() {
            if c.is_whitespace() {
                continue;
            } else {
                write!(writer, "{}", c)?;
            }
        }
        writeln!(writer)?;
    }

    Ok(())
}
