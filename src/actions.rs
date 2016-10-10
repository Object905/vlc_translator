use srt::Subtitles;
use srt::Timestamp;

use std::collections::HashMap;

#[path="vlc_controll.rs"]
mod vlc_controll;

#[path="utils.rs"]
mod utils;

use settings;


fn get_current_time() -> Timestamp {
    let current_time = vlc_controll::get_time();
    println!("current_time {:?}", current_time);
    Timestamp::from_microseconds(current_time as u64)
}

fn get_current_text(subtitles: &Subtitles) -> String {
    let current_time = get_current_time();
    println!("current_time Timestamp {:?}", current_time);
    let current = subtitles.nearest_by_time(current_time).unwrap();
    println!("current {:?}", current);
    utils::prepare_text(&current.text)
}

fn get_current_text_with_translation(subtitles: &Subtitles) -> (String, String) {
    let current = get_current_text(subtitles);
    let translation = utils::translate(&current);
    (current, translation)
}

fn push_translated_current(subtitles: &Subtitles) {
    println!("Push notification: current line");

    if settings::PAUSE_ON_PUSH {
        vlc_controll::pause();
    }

    let (current, translation) = get_current_text_with_translation(subtitles);
    utils::show_desktop_notification(&translation, &current);
}

fn google_translate_current(subtitles: &Subtitles) {
    println!("Open google translator: current line");

    if settings::PAUSE_ON_GOOGLE {
        vlc_controll::pause();
    }

    let current = get_current_text(subtitles);
    utils::open_google_translator(&current);
}

fn repeat_current_line(subtitles: &Subtitles) {
    println!("Repeat current line");

    let current_time = get_current_time();
    let latest = subtitles.nearest_by_time(current_time).unwrap();

    let seek_offset = latest.start.total_miliseconds() - settings::JUMP_OFFSET -
                      current_time.total_miliseconds();

    vlc_controll::seek(seek_offset as i64);
}

fn jump_to_previous(subtitles: &Subtitles) {
    println!("Jump to previous line");

    let current_time = get_current_time();

    let latest = subtitles.nearest_by_time(current_time).unwrap();
    let prev_index = (latest.index - 1) as usize;
    let previous = subtitles.by_index(prev_index).unwrap();

    let seek_offset = previous.start.total_miliseconds() - settings::JUMP_OFFSET -
                      current_time.total_miliseconds();

    vlc_controll::seek(seek_offset as i64);
}

fn show_current(subtitles: &Subtitles) {
    println!("Show current line");

    if settings::PAUSE_ON_SHOW {
        vlc_controll::pause();
    }

    let text = get_current_text(subtitles);
    utils::show_desktop_notification(&text, "");
}

lazy_static! {
    pub static ref ACTIONS: HashMap<u8, fn(&Subtitles)> = {
        let mut map = HashMap::<u8, fn(&Subtitles)>::new();
        map.insert(1, push_translated_current);
        map.insert(2, google_translate_current);
        map.insert(3, repeat_current_line);
        map.insert(4, jump_to_previous);
        map.insert(5, show_current);
        map
    };
}
