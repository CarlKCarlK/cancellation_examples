use tokio::time::{Duration, sleep};

async fn foo() -> u64 {
    println!("foo started");
    sleep(Duration::from_secs(2)).await;
    println!("foo finished");
    42
}

#[tokio::main]
async fn main() {
    tokio::select! {
        x = foo() => println!("foo returned {x}"),
        _ = sleep(Duration::from_secs(1)) => println!("foo timed out"),
    }
}
