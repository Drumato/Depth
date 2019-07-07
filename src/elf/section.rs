pub fn build_interp() -> Vec<u8> {
    "\x00/lib64/ld-linux-x86-64.so.2\x00".as_bytes().to_vec()
}
pub fn build_comment() -> Vec<u8> {
    "\x00Depth-v0.1.0:x86-64\x00".as_bytes().to_vec()
}
