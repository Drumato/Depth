pub fn build_interp() -> Vec<u8> {
    "\x00/lib64/ld-linux-x86-64.so.2\x00".as_bytes().to_vec()
}
pub fn build_comment() -> Vec<u8> {
    "Depth-v0.1.0:x86-64\x00".as_bytes().to_vec()
}

pub fn build_shstrndx(names: Vec<Vec<u8>>) -> Vec<u8> {
    let mut bts: Vec<u8> = Vec::new();
    bts.push(0x00);
    for n in names.iter() {
        for b in n.iter() {
            bts.push(*b);
        }
        bts.push(0x00);
    }
    bts
}
