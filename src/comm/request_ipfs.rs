// use ipfs::{Ipfs, IpfsOptions, IpfsPath, TestTypes, UninitializedIpfs};
use std::process::exit;
use std::{thread, time};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

#[tokio::main]
pub async fn make_request() {
    // let stream = ipfs.cat_unixfs(path, None).await.unwrap_or_else(|e| {
    //     eprintln!("Error: {}", e);
    //     exit(1);
    // });

    // tokio::pin!(stream);

    // let mut stdout = tokio::io::stdout();

    // loop {
    //     match stream.next().await {
    //         Some(Ok(bytes)) => {
    //             stdout.write_all(&bytes).await.unwrap();
    //         }
    //         Some(Err(e)) => {
    //             eprintln!("Error: {}", e);
    //             exit(1);
    //         }
    //         None => break,
    //     }
    // }
}

pub fn wait_for_request() {
    println!("  Waiting for a remote command...\n");

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();
        
        thread::sleep(ten_seconds);
        
        assert!(now.elapsed() >= ten_seconds);

        println!("  waiting...\n");
    }
}
