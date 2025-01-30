extern crate anyhow;
extern crate thiserror;

fn _min(a: &[u8]) -> Option<u8> {
    if a.is_empty() {
        return None;
    }
    let mut m = a[0];
    for &x in &a[1..] {
        if x < m {
            m = x;
        }
    }
    Some(m)
}

fn _option() {
    extern crate fastrand;

    let m = _min(&[3, 1, 2]);
    println!("{}", m.unwrap());
    let _x = _min(&[]);
    assert!(_x.is_none());
    let mut a: Vec<u8> = (0..fastrand::u8(..3)).collect();
    let m = _min(&a);
    match m {
        Some(0) => println!("min is 0"),
        None => println!("vec was empty"),
        Some(v) => panic!("got weird min {}", v),
    }

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

// pub fn open<P: AsRef<Path>>(path: P) -> Result<File, std::io::Error>

#[derive(Debug, thiserror::Error)]
pub enum FirstError {
/*
    #[error("error opening file: {0}")]
    OpenError(std::io::Error),
    #[error("error reading file: {0}")]
    ReadError(std::io::Error),
*/
    #[error("non-ascii first character")]
    NonAscii,
    #[error("empty file")]
    EmptyFile,
}

fn first_char_of_file(path: &str) -> Result<char, anyhow::Error> {
    use std::io::Read;

    let mut f = std::fs::File::open(path)?;
    let mut c = [0u8];

    match f.read(&mut c)? {
        1 => match c[0] {
            v if v.is_ascii() => Ok(v as char),
            _ => Err(FirstError::NonAscii.into()),
        }
        0 => Err(FirstError::EmptyFile.into()),
        n => panic!("internal error: overread: {n}"),
    }
}

fn main() {
    let c = first_char_of_file("/tmp/stuff").unwrap();
    println!("{}", c);
}
