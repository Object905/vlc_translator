use dbus::{Connection, BusType, Props, Message, MessageItem};

use settings;


fn get_connection() -> Connection {
    Connection::get_private(BusType::Session).unwrap()
}

pub fn get_time() -> i64 {
    let connection = get_connection();
    let post = Props::new(&connection,
                          "org.mpris.MediaPlayer2.vlc",
                          "/org/mpris/MediaPlayer2",
                          "org.mpris.MediaPlayer2.Player",
                          settings::DBUS_TIMEOUT);

    let response = post.get("Position")
        .expect("Cant get \"Position\" value from dbus interface");
    response.inner().unwrap()
}

pub fn seek(offset_miliseconds: i64) {
    let mut message = Message::new_method_call("org.mpris.MediaPlayer2.vlc",
                                               "/org/mpris/MediaPlayer2",
                                               "org.mpris.MediaPlayer2.Player",
                                               "Seek")
        .unwrap();

    message.append_items(&[MessageItem::Int64(offset_miliseconds * 1000)]);
    let connection = get_connection();
    connection.send_with_reply_and_block(message, settings::DBUS_TIMEOUT).unwrap();
}

pub fn pause() {
    let message = Message::new_method_call("org.mpris.MediaPlayer2.vlc",
                                           "/org/mpris/MediaPlayer2",
                                           "org.mpris.MediaPlayer2.Player",
                                           "Pause")
        .unwrap();
    let connection = get_connection();
    connection.send_with_reply_and_block(message, settings::DBUS_TIMEOUT).unwrap();
}

#[allow(dead_code)]
pub fn play() {
    let message = Message::new_method_call("org.mpris.MediaPlayer2.vlc",
                                           "/org/mpris/MediaPlayer2",
                                           "org.mpris.MediaPlayer2.Player",
                                           "Play")
        .unwrap();
    let connection = get_connection();
    connection.send_with_reply_and_block(message, settings::DBUS_TIMEOUT).unwrap();
}

#[allow(dead_code)]
pub fn play_pause() {
    let message = Message::new_method_call("org.mpris.MediaPlayer2.vlc",
                                           "/org/mpris/MediaPlayer2",
                                           "org.mpris.MediaPlayer2.Player",
                                           "PlayPause")
        .unwrap();
    let connection = get_connection();
    connection.send_with_reply_and_block(message, settings::DBUS_TIMEOUT).unwrap();
}

#[allow(dead_code)]
pub fn is_paused() -> bool {
    let connection = get_connection();
    let post = Props::new(&connection,
                          "org.mpris.MediaPlayer2.vlc",
                          "/org/mpris/MediaPlayer2",
                          "org.mpris.MediaPlayer2.Player",
                          settings::DBUS_TIMEOUT);

    let response = post.get("PlaybackStatus")
        .expect("Cant get \"PlaybackStatus\" value from dbus interface");
    let result: &str = response.inner().unwrap();

    if result == "Playing" { false } else { true }
}
