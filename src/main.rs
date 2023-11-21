use tokio::{io::{AsyncWriteExt, AsyncReadExt}, net::TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3444").await?;
    
    let mut buffer = vec![];
    buffer.write_u8(0).await?; // packet id
    
    // packets fields
    let version = String::from("1.0.0"); // client version (1.0.0)
    buffer.write_u8(version.len() as u8).await?;
    buffer.write_all(version.as_bytes()).await?;

    buffer.write_u8(0).await?; // enable compression (false)
    buffer.write_i64(-1).await?; // compression threshold (-1)
    
    stream.write_u8(buffer.len() as u8).await?; // packet length
    stream.write_all(&buffer).await?;

    loop {
        let v = stream.read_u8().await?;
        println!("{:#?}", v);
    }
}
