use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:3444").await?;
    stream.write_u8(1).await?;
    stream.write_u8(0).await?;

    Ok(())
}
