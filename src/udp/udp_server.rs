use mini_redis::{client, Result};
use std::fmt::Write;
use gethostname::gethostname;

#[tokio::main]
pub async fn server_start(host: &str, port:i16) -> Result<()> {
    // Open a connection to the mini-redis address.
    let host_str = host.to_owned()+":"+&String::from(port.to_string());
    
  println!("Host to connect: {}", host_str);
    let mut client = client::connect(host_str).await?;
    let test = client.get("hello").await?;
    if test != Some("picakunda".into()) {
      // Set the key "hello" with value "world"
      let hn = String::from("");
      let _ = gethostname().write_str(&hn);
      client.set("hello", hn.into()).await?;
    }else{
	    client.set("hello","picakunda".into()).await?;
    }
    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())

  }