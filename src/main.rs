use std::sync::{Arc, Mutex};
use std::thread;
use rphide::config::Config;
use rphide::{systray, trigger};

fn main() {
    let config = Arc::new(Mutex::new(Config::load()));

    let rx = {
        let config = Arc::clone(&config);
        trigger::start_scan(config)
    };

    let config = Arc::clone(&config);
    systray::init(config);
}
