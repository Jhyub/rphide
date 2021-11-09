use std::sync::{Arc, Mutex};
#[cfg(target_os = "linux")]
use tray_item::TrayItem;
#[cfg(target_os = "windows")]
use trayicon::*;
#[cfg(target_os = "windows")]
use core::mem::MaybeUninit;
#[cfg(target_os = "windows")]
use winapi::um::winuser;
use crate::config::Config;

#[cfg(target_os = "linux")]
pub fn init(config: Arc<Mutex<Config>>) {
    gtk::init().unwrap();

    let mut tray = TrayItem::new("rphide", "rphide").unwrap();

    tray.add_label(format!("rphide version {}", env!("CARGO_PKG_VERSION")).as_str()).unwrap();
    tray.add_label("by Jhyub").unwrap();


    tray.add_menu_item("Reload config", move|| {
        let mut config = config.lock().unwrap();
        *config = Config::load();
    }).unwrap();
        tray.add_menu_item("Edit config", move|| {
        open::that(Config::config_path());
    }).unwrap();
    tray.add_menu_item("Exit", move|| {
        std::process::exit(0);
    });

    gtk::main();
}

#[cfg(target_os = "windows")]
pub fn init(config: Arc<Mutex<Config>>) {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Events {
        Reload,
        Edit,
        Exit,
    }

    let (s, r) = std::sync::mpsc::channel::<Events>();
    let icon = include_bytes!("../assets/rphide.ico");

    let mut tray = TrayIconBuilder::new()
        .sender(s)
        .icon_from_buffer(icon)
        .tooltip("rphide")
        .menu(
            MenuBuilder::new()
                .item("Reload config", Events::Reload)
                .item("Edit config", Events::Edit)
                .item("Exit", Events::Exit)
        )
        .build().unwrap();

    std::thread::spawn(move || {
        r.iter().for_each(|m| match m {
            Events::Exit => { std::process::exit(0); },
            Events::Edit => { open::that(Config::config_path()); },
            Events::Reload => {
                let mut config = config.lock().unwrap();
                *config = Config::load();
            }
        })
    });

    loop {
        unsafe {
            let mut msg = MaybeUninit::uninit();
            let bret = winuser::GetMessageA(msg.as_mut_ptr(), 0 as _, 0, 0);
            if(bret > 0) {
                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageA(msg.as_ptr());
            } else {
                break;
            }
        }
    }
}
