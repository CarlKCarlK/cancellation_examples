use tokio::time::{Duration, sleep, timeout};

async fn foo() {
    println!("foo started");
    sleep(Duration::from_secs(2)).await;
    println!("foo finished");
}

#[tokio::main]
async fn main() {
    println!("A regular call to foo:");
    foo().await;

    println!("\nCalling foo with a timeout:");
    if let Err(_) = timeout(Duration::from_secs(1), foo()).await {
        println!("foo timed out");
    }

    println!("Even though the second foo gets cancelled, the process continues...");
    sleep(Duration::from_secs(10)).await;
}
