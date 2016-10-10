#[macro_use]
extern crate lazy_static;
extern crate srt;
extern crate dbus;
extern crate regex;
extern crate notify_rust;
extern crate rustc_serialize;

use std::process::exit;
use std::path::Path;
use std::os::unix::net::UnixStream;
use std::io::{Write, stderr};

use srt::Subtitles;

mod settings;
mod server;

fn send_command_and_exit(command_id: u8) {
    let mut stream = UnixStream::connect(settings::SOCKET_ADDRESS).unwrap();
    stream.write(&[command_id]).unwrap();
    exit(0);
}

fn print_err_and_exit(msg: &str) -> ! {
    stderr().write_fmt(format_args!("{}\n", msg)).unwrap();
    exit(1);
}

fn show_help() -> ! {
    println!("\
        USAGE:\
        \n\tvlct [COMMAND]\
        \n\tvlct [STR_FILE]\
        \nCOMMANDS:\
        \n\ttranslate - push current line with translation through notification\
        \n\ttranslate_google - open google translator with current line\
        \n\trepeat - jump to the start of current line\
        \n\tprevious - jump to the start of previous line\
        \n\tshow - show current line without translation through notification\
        \n\thelp - show this help");
    println!("");
    exit(0);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        print_err_and_exit("Not enough arguments.\nSee \"vlct help\" for help");
    };

    match args[1].as_ref() {
        "translate" => send_command_and_exit(1),
        "translate_google" => send_command_and_exit(2),
        "repeat" => send_command_and_exit(3),
        "previous" => send_command_and_exit(4),
        "show" => send_command_and_exit(5),
        "help" => show_help(),
        "raw" => {
            send_command_and_exit(args[2].parse().unwrap_or_else(|_| {
                print_err_and_exit("arg after raw must be number in [0;255] range")
            }))
        }
        _ => {}
    }

    let srt_path = Path::new(&args[1]);
    if srt_path.exists() {

        let subtitles = Subtitles::from_file(srt_path)
            .unwrap_or_else(|_| print_err_and_exit("Can't open given subtitles file."));
        server::start(subtitles);
    }

    print_err_and_exit("Wrong arguments. Use \"vlct help\" for help.");
}
