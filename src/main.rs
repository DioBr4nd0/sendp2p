use std::{ path::PathBuf, str::FromStr };

use anyhow::Result;
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{
    net_protocol::Blobs,
    rpc::client::blobs::{ ReadAtLen, WrapOption },
    ticket::BlobTicket,
    util::SetTagOption,
};

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint: Endpoint = Endpoint::builder().discovery_n0().bind().await?;
    let blobs = Blobs::memory().build(&endpoint);
    let node = Router::builder(endpoint).accept(iroh_blobs::ALPN, blobs.clone()).spawn().await?;

    let client = blobs.client();

    let args = std::env::args().collect::<Vec<_>>();
    match &args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [_cmd, "send", path] => {
            let abs_path = PathBuf::from_str(path)?.canonicalize()?;

            println!("Analyzing file...");

            let blob = client
                .add_from_path(abs_path, true, SetTagOption::Auto, WrapOption::NoWrap).await?
                .finish().await?;

            let node_id = node.endpoint().node_id();
            let ticket = BlobTicket::new(node_id.into(), blob.hash, blob.format)?;

            println!("File Analyzed. Fetch the file by running:");
            println!("cargo run  receive {ticket} {path}");

            tokio::signal::ctrl_c().await?;
        }
        [_cmd, "receive", ticket, path] => {
            let path_buf = PathBuf::from_str(path)?;
            let ticket = BlobTicket::from_str(ticket)?;

            println!("Starting Download...");

            client.download(ticket.hash(), ticket.node_addr().clone()).await?.finish().await?;

            println!("Finished Download.");
            println!("Copying to desination");

            let mut file = tokio::fs::File::create(path_buf).await?;
            let mut reader = client.read_at(ticket.hash(), 0, ReadAtLen::All).await?;
            tokio::io::copy(&mut reader, &mut file).await?;

            println!("Finished copying...");
        }
        _ => {
            println!("Couldn't parse command line arguments: {args:?}");
            println!("Usage:");
            println!("    # to send:");
            println!("    cargo run  send [FILE]");
            println!("    # this will print a ticket.");
            println!();
            println!("    # to receive:");
            println!("    cargo run  receive [TICKET] [FILE]");
        }
    }
    println!("shutting down...");
    node.shutdown().await?;
    Ok(())
}
