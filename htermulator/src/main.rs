use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn handle_client(mut stream : TcpStream) -> std::io::Result<()>{
    let mut buf = [0;128];
    let _n = stream.read(&mut buf[..]);
    println!("{:?}",&buf);
    Ok(())
    }

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088")?;

    for stream  in listener.incoming(){
        match stream{
            Ok(stream) => {
                return handle_client(stream);
        }
            Err(e) => {
                println!("{}",e);
        }
    }
    }
    Ok(())
}
