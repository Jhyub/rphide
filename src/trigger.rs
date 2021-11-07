use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use regex::Regex;
use sysinfo::{ProcessExt, RefreshKind, System, SystemExt};
use crate::config::Config;

#[derive(Debug)]
pub enum Status {
    Hide,
    Show,
    CheckWindow(Vec<Regex>)
}

pub fn start_scan(config: Arc<Mutex<Config>>) -> mpsc::Receiver<Status> {
    let (tx, rx) = mpsc::channel();
    let cycle = {
        config.lock().unwrap().cycle
    };


    thread::spawn(move || {
        let mut sys = System::new_with_specifics(RefreshKind::new().with_processes());
        loop {
            sys.refresh_processes();
            let processes = sys.processes();
            let triggers = {
                let config = config.lock().unwrap();
                config.triggers()
            };

            let mut best = Status::Show;
            for (_, v) in processes {
                for trigger in triggers.iter() {
                    if v.name() == trigger.process_name.as_str() {
                        match &trigger.window_name {
                            None => {
                                match &best {
                                    Status::Hide => {}
                                    _ => { best = Status::Hide }
                                }
                            }
                            Some(a) => {
                                match &best {
                                    Status::Hide => {}
                                    Status::Show => { best = Status::CheckWindow(vec!(a.clone())) }
                                    Status::CheckWindow(vec) => {
                                        let mut vec = vec.clone();
                                        vec.push(a.clone());
                                        best = Status::CheckWindow(vec);
                                    }
                                }
                            }
                        }
                    }
                    match &best {
                        Status::Hide => { break; }
                        _ => {}
                    }
                }
                match &best {
                    Status::Hide => { break; }
                    _ => {}
                }
            }

            best = match &best {
                Status::CheckWindow(a) => check_title_regex(a),
                _ => best
            };

            tx.send(best);

            thread::sleep(Duration::from_millis(cycle));
        }
    });

    rx
}

fn check_title_regex(vec: &Vec<Regex>) -> Status {
    // WIP!
    Status::Hide
}