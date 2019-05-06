use std::io::BufRead;

pub fn get_content(stream: &mut BufRead) -> String{
    let mut buffer = String::new();
    loop{
        match stream.read_line(&mut buffer){
            Ok(0) => break,
            Ok(_) => {
                continue;
            }
            Err(e) => {
                println!("Error found:{}",e);
                break;
            }
        }
    }
    buffer
}
