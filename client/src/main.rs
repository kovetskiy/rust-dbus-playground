extern crate dbus;

use dbus::{Connection, ConnectionItem, BusType, NameFlag};
use dbus::tree::Signal;

fn main() {
    let connection = Connection::get_private(BusType::Session).unwrap();

    let rule = "type='signal',interface='com.github.kovetskiy.rustabus',member='Ping'".to_string();

    let mut message = dbus::Message::new_method_call(
        "org.freedesktop", "/org/freedesktop",
        "org.freedesktop.DBus", "AddMatch"
    ).unwrap();

    message.append_items(&[dbus::MessageItem::Str(rule.to_string())]);

    connection.send(message);

    for n in connection.iter(1000) {
        match n {
            ConnectionItem::MethodCall(mut m) => {
                println!("MethodCall: {:?}", m);
            },
            ConnectionItem::Signal(m) => {
                println!("Signal: {:?}", m);
            },
            ConnectionItem::MethodReturn(m) => {
                println!("MethodReturn: {:?}", m);
            },
            ConnectionItem::WatchFd(m) => {
                println!("WatchFd: {:?}", m);
            },
            ConnectionItem::Nothing => (),
        }
    }
}
