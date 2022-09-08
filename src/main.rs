extern crate core;


use std::{env, fs, thread};
use std::fs::remove_file;
use std::ops::Add;
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use log::info;
use rphide::self_ipc::{knock, listen_primary_ipc, SecondaryTask};
use rphide::tray::tray;
use rphide::ui;

fn main() {
    env_logger::init();
    info!("rphide version {}", env!("CARGO_PKG_VERSION"));
    #[cfg(target_family = "unix")]
        let ipc_name = env::var("XDG_RUNTIME_DIR")
        .or_else(|_| env::var("TMPDIR"))
        .or_else(|_| env::var("TMP"))
        .or_else(|_| env::var("TEMP"))
        .unwrap_or(String::from("/tmp"))
        .add("/rphide-ipc");
    #[cfg(target_family = "windows")]
        let ipc_name = format!("\\\\?\\pipe\\rphide-ipc");

    let ipc_conn_res = LocalSocketStream::connect(ipc_name.as_str());
    if let Err(_) = ipc_conn_res {
        remove_file(ipc_name.as_str());
    }

    let ipc_res = LocalSocketListener::bind(ipc_name.as_str());
    if let Ok(ipc) = ipc_res { // Current process is first process
        info!("Running initial process");
        let (ui_update_tx, ui_update_rx) = mpsc::channel();
        let (ui_result_tx, ui_result_rx) = mpsc::channel();
        // Worker thread
        let ui_update_tx_clone = mpsc::Sender::clone(&ui_update_tx);
        thread::spawn(move || {
            info!("Occupying discord-ipc-0...");
            occupy_ipc_0(&ui_update_tx_clone, &ui_result_rx).unwrap();
        });
        // IPC Receiver thread
        thread::spawn(move || {
            info!("Listening to rphide-ipc...");
            listen_primary_ipc(ipc, ui_update_tx);
        });
        thread::sleep(Duration::from_millis(500));
        //Command::new(env::current_exe().unwrap().to_str().unwrap()).spawn().unwrap();
        // Main thread - UI
        info!("Starting UI");
        ui::Ui::launch(ui_update_rx, ui_result_tx);
    } else {
        info!("Running secondary process");
        let mut ipc = LocalSocketStream::connect(ipc_name).unwrap();
        info!("Connected from secondary to primary");
        let task = { knock(&mut ipc) };
        info!("Got task {:?}", task);
        match task {
            SecondaryTask::Tray => { tray(ipc); }
            _ => {}
        }
    }
}

fn occupy_ipc_0(tx: &mpsc::Sender<ui::UiUpdate>, rx: &mpsc::Receiver<ui::UiResult>) -> Result<LocalSocketListener, std::io::Error> {
    #[cfg(target_family = "unix")]
        let name = env::var("XDG_RUNTIME_DIR")
        .or_else(|_| env::var("TMPDIR"))
        .or_else(|_| env::var("TMP"))
        .or_else(|_| env::var("TEMP"))
        .unwrap_or(String::from("/tmp"))
        .add("/discord-ipc-0");
    #[cfg(target_family = "windows")]
        let name = format!("\\\\?\\pipe\\discord-ipc-0");

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
                    ui::UiResult::RestartDiscord => {}
                    _ => {}
                }
            } else {}
            listener
        }
        #[cfg(target_family = "windows")]
        Err(ref err) if err.kind() == std::io::ErrorKind::AddrInUse => {
            todo!();
        }
        Err(err) => Err(err),
    }
}

