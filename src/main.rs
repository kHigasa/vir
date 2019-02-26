extern crate clap;
extern crate termion;

use std::ffi::OsStr;
use std::fs;
use std::io::{stdin, stdout, Write};
use clap::{App, Arg};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let matches = App::new("vir")
                      .about("Vi Rusted")
                      .bin_name("vir")
                      .arg(Arg::with_name("file"))
                      .get_matches();
    let file_path: Option<&OsStr> = matches.value_of_os("file");

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    write!(stdout, "Hello, world.").unwrap();
    stdout.flush().unwrap();

    for event in stdin.events() {
        if event.unwrap() == Event::Key(Key::Ctrl('c')) {
            return;
        }
    }
}

