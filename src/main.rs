use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            // 声明一个bytes变量，值为“Hello!”
            let msg = b"Hello!";

            // 向server传输msg
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            // using 6 byte buffer
            let mut data = [0 as u8; 6];
            // 读取返回值
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
