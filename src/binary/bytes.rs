use std::fs::File;
use std::io::{Read, Write};
pub struct Bin {
    pub b: std::io::Cursor<Vec<u8>>,
    pub le: bool,
}

impl Bin {
    pub fn new(param: (Vec<u8>, bool)) -> Bin {
        Bin {
            b: std::io::Cursor::new(param.0),
            le: param.1,
        }
    }
    pub fn read_file(filepath: &str) -> Bin {
        let mut file = File::open(filepath).unwrap();
        let l: u64 = file.metadata().unwrap().len();
        let mut buf = Vec::with_capacity(l as usize);
        match file.read_to_end(&mut buf) {
            _ => (),
        }
        Bin::new((buf, true))
    }
    pub fn write(&mut self, bys: &Vec<u8>) {
        self.b.write(bys).unwrap();
    }
    pub fn flush(&self, filepath: &str) -> Result<(), Box<std::error::Error>> {
        let mut file = File::create(filepath)?;
        let buf = self.b.get_ref();
        file.write_all(buf)?;
        file.flush()?;
        Ok(())
    }
    pub fn set(&mut self, pos: u64) {
        self.b.set_position(pos);
        assert_eq!(self.b.position(), pos);
    }
}
