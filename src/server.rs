use std::os::unix::net::UnixListener;
use std::io::Read;
use std::fs::remove_file;
use std::thread::spawn;
use std::sync::Arc;

use srt::Subtitles;

#[path="actions.rs"]
mod actions;

use settings;
use self::actions::ACTIONS;

pub fn start(subtitles: Subtitles) {
    println!("Starting server on {}", settings::SOCKET_ADDRESS);
    let listener = match UnixListener::bind(&*settings::SOCKET_ADDRESS) {
        Ok(i) => i,
        Err(_) => {
            remove_file(settings::SOCKET_ADDRESS).unwrap();
            UnixListener::bind(&*settings::SOCKET_ADDRESS).unwrap()
        }
    };
    println!("Server started.\nListening for incoming connections...\n");

    let subtitles = Arc::new(subtitles);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0];
        stream.read_exact(&mut buf).unwrap();

        let subtitles = subtitles.clone();
        spawn(move || {
            handle_command(buf[0], subtitles);
        });
    }
}

fn handle_command(command_id: u8, subtitles: Arc<Subtitles>) {
    println!("Received: {}", command_id);

    let action = ACTIONS.get(&command_id).unwrap();
    action(&subtitles);

    println!("----------");
}
