extern crate core;


use std::{env, thread};
use std::fs::remove_file;
use std::ops::Add;
use std::path::Path;
use std::sync::mpsc;
use interprocess::local_socket::{LocalSocketListener};
use log::info;
use rphide::ui;

fn main() {
    env_logger::init();
    info!("rphide version {}", env!("CARGO_PKG_VERSION"));
    /*
    gtk::init().unwrap();
    let mut tray = TrayItem::new("rphide", "rphide").unwrap();
    tray.add_label("Tray Label").unwrap();
    tray.add_menu_item("Hello", || {
        println!("Hello, World!");
    }).unwrap();
    gtk::main();

     */

    let (txu, rxu) = mpsc::channel();
    let (txr, rxr) = mpsc::channel();
    thread::spawn(move || {
        occupy_ipc_0(&txu, &rxr).unwrap();
    });
    ui::Ui::launch(rxu, txr);
}

fn occupy_ipc_0(tx: &mpsc::Sender<ui::UiUpdate>, rx: &mpsc::Receiver<ui::UiResult>) -> Result<LocalSocketListener, std::io::Error> {
    #[cfg(target_family="windows")]
        let name = format!("\\\\?\\pipe\\discord-ipc-0");

    #[cfg(target_family="unix")]
        let name = env::var("XDG_RUNTIME_DIR")
        .or_else(|_| env::var("TMPDIR"))
        .or_else(|_| env::var("TMP"))
        .or_else(|_| env::var("TEMP"))
        .unwrap_or(String::from("/tmp"))
        .add("/discord-ipc-0");

    match LocalSocketListener::bind(name.as_str()) {
        Ok(listener) => Ok(listener),
        #[cfg(target_family = "unix")]
        Err(ref err) if err.kind() == std::io::ErrorKind::AddrInUse => {
            // We can delete the socket, but a discord restart is required so that it can open a new socket
            remove_file(Path::new(name.as_str()))?;
            let listener = occupy_ipc_0(tx, rx);
            tx.send(ui::UiUpdate::AskRestart).unwrap();
            if let Ok(recv) = rx.recv() {
                match recv {
                    ui::UiResult::RestartDiscord => {},
                    _ => {}
                }
            } else {
            }
            listener
        }
        Err(err) => Err(err),
    }
}

