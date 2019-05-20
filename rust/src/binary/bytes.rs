use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
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

    /*pub fn range(&mut self, src: u64, dst: u64) -> Result<Vec<u8>, Box<std::error::Error>> {
        if src > dst {
            println!("Error found: src is bigger than dst");
        }
        let orig = self.input.position();
        let length: u64 = dst - src;
        let mut v = Vec::with_capacity(length as usize);
        self.input.seek(SeekFrom::Start(src))?;
        for i in 0..dst {
            let i = i as usize;
            self.input.write(*v[i])?;
        }
        println!("{:?}", v);
        self.input.set_position(orig);
        Ok(v)
    }*/
}
//writer.seek(SeekFrom::End(-10))?;

//   for i in 0..10 {
//      writer.write(&[i])?;
// }
