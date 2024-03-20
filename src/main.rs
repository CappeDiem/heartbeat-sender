use std::{env, thread};
use dotenv::dotenv;
use std::error::Error;
use std::time::{Duration, Instant};

fn main() {
    dotenv().expect("Failed to load .env file");
    let scheduler = thread::spawn(|| {
        loop {
            let start = Instant::now();

            let heartbeat_url = env::var("HEARTBEAT_URL").expect("Expected a token in the environment");
            let interval = env::var("HEARTBEAT_INTERVAL").expect("Expected a token in the environment");

            send_heartbeat(heartbeat_url).expect("TODO: panic message");
            thread::sleep(Duration::from_millis(interval.parse::<u64>().unwrap()));
            let duration = start.elapsed();
            println!("interval: {:?}", duration);
        }
    });
    scheduler.join().expect("Scheduler panicked");
}

fn send_heartbeat(url:String) -> Result<(), Box<dyn Error>> {
    let _resp = reqwest::blocking::get(url)?.text()?;
    //println!("{:#?}", resp);
    println!("INFO: Heartbeat Sent");
    Ok(())
}