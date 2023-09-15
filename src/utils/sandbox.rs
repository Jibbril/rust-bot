async fn _run_timers() {
    let configs = vec![(2,9), (5,3)];    
    let mut handles = vec![];

    for (interval,stop) in configs {
        let handle = spawn(tracker(interval, stop));
        handles.push(handle)
    }

    for handle in handles {
        handle.await.unwrap()
    }
}

async fn tracker(interval: u64, stop: u64) {
    let mut i = 0;

    loop {
        // Wait for 5 minutes
        sleep(Duration::from_secs(interval)).await;

        println!("Interval {}, iteration: {}", interval, i);

        i += 1;
        if i > stop {
            break;
        }
    }
}