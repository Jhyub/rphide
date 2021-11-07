use std::sync::{Arc, Mutex};
use tray_item::TrayItem;
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
    let mut tray = TrayItem::new("rphide", "icon.rc").unwrap();

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

    tray.set_icon("icon.rc").unwrap();

    std::io::stdin().read_line(&mut String::new()).unwrap();
}
