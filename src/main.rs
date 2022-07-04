use std::io::{BufRead, BufReader, Read, Write};
use std::{thread, time};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::time::Duration;
use gtk;
use interprocess::local_socket::LocalSocketStream;
use tray_item::TrayItem;
use rphide::discord_data::{DiscordData, OpCode, ReadDataExt};

fn main() {
    println!("Hello, world!");
    /*
    gtk::init().unwrap();
    let mut tray = TrayItem::new("rphide", "rphide").unwrap();
    tray.add_label("Tray Label").unwrap();
    tray.add_menu_item("Hello", || {
        println!("Hello, World!");
    }).unwrap();
    gtk::main();

     */

    let mut stream = LocalSocketStream::connect("/run/user/1000/discord-ipc-0").unwrap();
    let handshake = DiscordData::from(OpCode::HandShake, "{\"v\": 1,\"client_id\": \"782685898163617802\"}");
    let mut handshake_res = DiscordData::empty();
    stream.write_all(&handshake.as_bytes()[..]).unwrap();
    stream.read_to_data(&mut handshake_res).unwrap();
    println!("{:?}", handshake_res);

}
