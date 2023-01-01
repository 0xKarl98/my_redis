use clap::{Parser, Subcommand};
use tokio::net::TcpStream;
use bytes::{BytesMut, BufMut};
//This for ?
use tokio::io::{AsyncReadExt, AsyncWriteExt};

//Define the supported command 
#[derive(Subcommand,Debug)]
enum Command {
    //[clap(about = "Get the value of a key")]
    Get { key: String },
    //[clap(about = "Set the value of a key")]
    Set { key: String, value: String },
}
#[derive(Parser,Debug)]
struct Cli {
    command : Command , 
}

//with such macro , we can make the main function async
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {

    let args = Cli::parse();
    
    //await keyword should be called inside async code block
    TcpStream::connect("127.0.0.1:8000").await.unwrap();
    match args.command {
        Command::Set {key, value} => {
            stream.write_all(b"set").await?;
            //the meaning of this line ?
            stream.write_all(b"").await?;

            //write the key 
            stream.write_all(&key.as_bytes()).await?;
            stream.write_all(b"").await?;
            //write the value 
            stream.write_all(&value.as_bytes()).await?;
            
            let mut buf = BytesMut::with_capacity(1024);
            let length = stream.read_buf(&mut buf).await?;
            match std::str::from_utf8(&mut buf) {
                Ok(uKey) => {
                    if uKey == "key updated " {
                        println!("key updated" , uKey);
                    } else if uKey == "key set" {
                        println!("key set" , uKey);
                    }
                }
                Err(err) => {
                    // failed to convert bytes into string slice
                    println!("error: {}", err);
                }

                return Ok(());
            }
 
        }
        Ok(()) 
    }
    


}
