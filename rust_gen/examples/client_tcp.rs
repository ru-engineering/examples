use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let string = match get_line(){
        CrashCases::Ok(s) => s,
        CrashCases::ConnectErr => String::from("CANT CONNECT"),
        CrashCases::WriteErr => String::from("CANT WRITE"),
        CrashCases::ReadErr => String::from("CANT READ"),
    };
    println!("got {:?}",string);

    Ok(())
} // the stream is closed here


fn get_line()-> CrashCases{
    let mut stream = match TcpStream::connect("127.0.0.1:8080"){
    Ok(x) => x,
    Err(_) => return CrashCases::ConnectErr,
    };

    match stream.write(b"Hello there!") {
        Ok(_)=>{},
        Err(_)=> return CrashCases::WriteErr,
    }
    let mut buffer = [0;1024];
    match stream.read(&mut buffer){
        Ok(_) => {},
        Err(_) => return CrashCases::ReadErr,
    }

    let mut string : &str = std::str::from_utf8(&buffer).unwrap();
    string = string.trim_end_matches('\0');
    CrashCases::Ok(String::from(string))
}


enum CrashCases{
    Ok(String),
    ConnectErr,
    WriteErr,
    ReadErr,
}
