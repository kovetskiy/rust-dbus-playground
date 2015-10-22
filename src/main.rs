extern crate dbus;

use dbus::{Connection, BusType, NameFlag};
use dbus::tree::Signal;

fn main() {
    let connection = Connection::get_private(BusType::Session).unwrap();

    let rule = "type='signal',interface='com.github.kovetskiy',member='Ping'".to_string();

    let mut message = dbus::Message::new_method_call(
        "com.github.kovetskiy.rustabus",
        "/com/github/kovetskiy/rustabus",
        "com.github.kovetskiy.rustabus",
        "Ping"
    ).unwrap();

    message.append_items(&[dbus::MessageItem::Str(rule.to_string())]);

    connection.send(message);
}
