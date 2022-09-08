use std::{env, fs, io, thread};
use std::io::{BufReader, Cursor, Read, Write};
use std::ops::Add;
use std::sync::mpsc;
use std::time::Duration;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream};
use log::{error, info};
use sysinfo::{PidExt, ProcessRefreshKind, RefreshKind, System, SystemExt, Pid, ProcessExt};
use crate::ui;

#[repr(u8)]
#[derive(Debug)]
pub enum Action {
    Knock = 0,
    Hide = 1,
    Unhide = 2,
    Quit = 3,
}

#[repr(u8)]
#[derive(Debug)]
pub enum SecondaryTask {
    Tray = 0,
    Exit = 1,
    None = 2,
}

fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
    match conn {
        Ok(val) => Some(val),
        Err(err) => {
            error!("Failed incoming ipc connection: {}", err);
            None
        }
    }
}

fn check_tray() -> bool {
    #[cfg(target_family = "unix")]
        let path = env::var("XDG_RUNTIME_DIR")
        .or_else(|_| env::var("TMPDIR"))
        .or_else(|_| env::var("TMP"))
        .or_else(|_| env::var("TEMP"))
        .unwrap_or(String::from("/tmp"))
        .add("/rphide-tray.pid");

    #[cfg(target_family = "windows")]
        let path = env::var("userprofile")
        .add("\\AppData\\Local\\Temp\\rphide-tray.pid");

    let pid = fs::read_to_string(path).map(|it| { it.parse::<i32>() });

    if let Ok(Ok(pid)) = pid {
        let mut system = System::new();
        system.refresh_process(Pid::from(pid));
        system.refresh_process(Pid::from_u32(std::process::id()));
        let process = system.process(Pid::from(pid));
        let self_process = system.process(Pid::from_u32(std::process::id())).unwrap();
        if let Some(process) = process {
            if process.name() == self_process.name() {
                return true;
            }
        }
    }
    false
}

pub fn listen_primary_ipc(listener: LocalSocketListener, tx: mpsc::Sender<ui::UiUpdate>) {
    for mut conn in listener.incoming().filter_map(handle_error) {
        info!("Got connection {:?}", conn);
        let mut buff = [0u8];
        conn.read(&mut buff).unwrap();
        let code = buff[0];
        info!("Got code {:?}", code);
        let action = match code {
            0 => Some(Action::Knock),
            1 => Some(Action::Hide),
            2 => Some(Action::Unhide),
            3 => Some(Action::Quit),
            _ => None,
        };
        info!("Got action {:?}", action);

        if let Some(action) = action {
            match action {
                Action::Knock => {
                    if check_tray() {
                        conn.write_u8(SecondaryTask::Tray as u8).unwrap();
                    } else {
                        conn.write_u8(SecondaryTask::Exit as u8).unwrap();
                    }
                }
                Action::Hide => {
                    tx.send(ui::UiUpdate::Hide).unwrap();
                }
                Action::Unhide => {
                    tx.send(ui::UiUpdate::Unhide).unwrap();
                }
                Action::Quit => {
                    tx.send(ui::UiUpdate::AcutallyQuit).unwrap();
                }
            }
        } else {
            error!("Can't understand action: {}", code);
        }
    }
}

pub fn knock(stream: &mut LocalSocketStream) -> SecondaryTask {
    stream.write_u8(Action::Knock as u8).unwrap();
    info!("Knocked to primary");
    match stream.read_u8().unwrap() {
        0 => SecondaryTask::Tray,
        1 => SecondaryTask::Exit,
        _ => SecondaryTask::None,
    }
}