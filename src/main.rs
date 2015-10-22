extern crate dbus;

use dbus::{Connection, BusType, NameFlag};
use dbus::tree::Signal;

use std::thread;

fn main() {
    let connection = Connection::get_private(BusType::Session).unwrap();

    let rule = "ping from rustabus".to_string();


    while true {
        let mut message = dbus::Message::new_signal(
            "/com/github/kovetskiy/rustabus",
            "com.github.kovetskiy.rustabus",
            "Ping"
        ).unwrap();

        message.append_items(&[dbus::MessageItem::Str(rule.to_string())]);

        connection.send(message);

        thread::sleep_ms(1000);
    }
}
