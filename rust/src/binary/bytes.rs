use std::fs::File;
use std::io::{self, BufReader, Read, Write};
pub struct Binaryen {
    pub input: std::io::Cursor<Vec<u8>>,
    pub le: bool,
}

impl Binaryen {
    pub fn new(param: (Vec<u8>, bool)) -> Binaryen {
        Binaryen {
            input: std::io::Cursor::new(param.0),
            le: param.1,
        }
    }
    pub fn flush(&self, filepath: &str) -> Result<(), Box<std::error::Error>> {
        let mut file = File::create(filepath)?;
        let buf = self.input.get_ref();
        file.write_all(buf)?;
        file.flush()?;
        Ok(())
    }
}
