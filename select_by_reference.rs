use tokio::time::{Duration, sleep};

async fn foo() -> u64 {
    println!("foo started");
    sleep(Duration::from_secs(2)).await;
    println!("foo finished");
    42
}

#[tokio::main]
async fn main() {
    let mut future = std::pin::pin!(foo());
    loop {
        tokio::select! {
            x = &mut future => {
                println!("foo returned {x}");
                break;
            }
            _ = sleep(Duration::from_secs(1)) => println!("foo is slow..."),
        }
    }
}
