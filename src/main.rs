use std::sync::{Arc, Mutex};
use std::thread;
use rphide::config::Config;
use rphide::systray;

fn main() {
    let config = Arc::new(Mutex::new(Config::load()));

    {
        let config = Arc::clone(&config);
        let systray = thread::spawn(|| {
            systray::init(config);
        });
        systray.join().unwrap()
    }
}
