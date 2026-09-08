#![allow(unused)] 
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // _____________________________________________________________________________________________
    // TAKING ARGS, ENV VARS, DIRECTORIES

    // Taking arguments
    use std::env;
    let args: Vec<String> = env::args().collect();

    // Taking env vars
    env::var("CLIEND_ID");  // env var      (Option)
    env::vars();            // all env vars (iterator)

    // Current, home, temp dirs
    env::current_dir();
    env::home_dir();
    env::temp_dir();
    
    // _____________________________________________________________________________________________
    // UUID
    use uuid::Uuid;
    let id = Uuid::new_v4();
    id.to_string();

    // _____________________________________________________________________________________________
    // RANDOM
    // `cargo add rand`

    // quick one
    rand::random_range(1..100);     // rand integer
    rand::random_range(0.0..1.0);   // rand float

    // reusable one
    use rand::RngExt;
    
    let mut rng = rand::rng();
    rng.random_range(1..=6);      // rand integer
    rng.random_range(0.0..1.0);   // rand float
    
    // _____________________________________________________________________________________________
    // SLEEP
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(1));       // sleep for 1 second
    sleep(Duration::from_millis(500));   // sleep for 500 milliseconds

    // _____________________________________________________________________________________________
    // LOGGING
    // TODO - double check those notes, they are very incomplete
    
    // `cargo add log`
    use log::{trace, debug, info, warn, error};

    trace!("this is a trace message");
    debug!("this is a debug message");
    info!("this is an info message");
    warn!("this is a warning message");
    error!("this is an error message");

    // _____________________________________________________________________________________________
	// Timing things
	let before = Instant::now();
	// do some work here
	let elapsed = before.elapsed();
	let elapsed_s = elapsed.as_secs();
	let elapsed_ms = elapsed.as_millis();

	// _____________________________________________________________________________________________
	// Getting timestamps
	let ts = SystemTime::now().duration_since(UNIX_EPOCH).expect("asoij").as_micros();

}






