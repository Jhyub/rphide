use std::sync::mpsc;
use std::thread;
use byteorder::WriteBytesExt;
use interprocess::local_socket::LocalSocketStream;
use tray_item::TrayItem;
use crate::self_ipc::Action;

pub fn tray(mut ipc: LocalSocketStream) {
    #[cfg(target_os = "linux")]
    gtk::init().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let recv = rx.recv().unwrap();
            ipc.write_u8(recv as u8).unwrap();
        }
    });


    let mut tray = TrayItem::new("rphide", "rphide").unwrap();

    tray.add_label(format!("rphide v{}", env!("CARGO_PKG_VERSION")).as_str()).unwrap();

    let txclone = mpsc::Sender::clone(&tx);
    tray.add_menu_item("Open", move || {
        txclone.send(Action::Unhide).unwrap();
    }).unwrap();

    let txclone = mpsc::Sender::clone(&tx);
    tray.add_menu_item("Hide", move || {
        txclone.send(Action::Hide).unwrap();
    }).unwrap();

    let txclone = mpsc::Sender::clone(&tx);
    tray.add_menu_item("Quit", move || {
        txclone.send(Action::Quit).unwrap();
        #[cfg(target_os="linux")]
        gtk::main_quit();
    }).unwrap();

    #[cfg(target_os="linux")]
    gtk::main();
}