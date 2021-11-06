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

    gtk::main();
}
