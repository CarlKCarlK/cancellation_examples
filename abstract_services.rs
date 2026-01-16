// Two abstract services sharing a resource
// Both try to do_long_work (which holds a lock)
// Monitor times out when it can't get the lock (select! + timeout)
// Jack's scenario: monitor task blocked by long task's lock

use std::sync::Arc;
use tokio::{select, sync::Mutex, time::{sleep, Duration}};

// Abstract Service: Encapsulates work that needs a shared resource
#[derive(Clone)]
struct WorkService {
    resource: Arc<Mutex<()>>,
}

impl WorkService {
    fn new(resource: Arc<Mutex<()>>) -> Self {
        Self { resource }
    }

    async fn do_long_work(&self, label: &str) {
        println!("{label}: acquiring lock...");
        let _guard = self.resource.lock().await;
        println!("{label}: got lock! working for 10ms...");
        sleep(Duration::from_millis(50)).await;
        println!("{label}: done!");
    }
}

#[tokio::main]
async fn main() {
    println!("Abstract Services: both try to do work on shared resource\n");

    // Create the shared resource once
    let resource = Arc::new(Mutex::new(()));
    let service = WorkService::new(resource);

    // Long task starts first and holds the lock for 50ms
    println!("Long task starting...");
    select! {
        _ = service.do_long_work("long") => {
            println!("\nlong finished");
        }
        _ = async {
            // Monitor tries repeatedly every 5ms, times out while lock is held
            for attempt in 1..=10 {
                sleep(Duration::from_millis(5)).await;
                println!("\n=== Monitor attempt {} (5ms timeout) ===", attempt);

                // Try to do work with a timeout
                match select! {
                    result = service.do_long_work("monitor") => {
                        Some(result)
                    }
                    _ = sleep(Duration::from_millis(5)) => {
                        None
                    }
                } {
                    Some(_) => {
                        println!("monitor: got to do work!");
                        break;
                    }
                    None => println!("monitor: TIMEOUT! resource locked"),
                }
            }
        } => {}
    }

    println!("\n✓ Monitor tried multiple times but timed out until long task released lock.");
}


