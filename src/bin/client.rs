use clap::{Parser, Subcommand};
use tokio::net::TcpStream;

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
            
            //S
        }
    }
    

    //Read data from server 
}
