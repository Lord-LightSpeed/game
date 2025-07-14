extern crate termion;

use std::io::{Write, stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap(); // The screen must be in raw mode.
    let mut player_pos = (0,0);

    // Forever loop, watching for keypresses.
    for k in stdin.keys() { // std::io::stdin seems to be modified by the 'use..TermRead'. W/o it, ".keys()' doesn't work.
        match k.as_ref().unwrap() { // Check keypress for a match.
            Key::Char('q') | Key::Char('Q') => { // quit loop
                break;
            },
            Key::Left => {player_pos.0 -= 1},
            Key::Right => {player_pos.0 += 1},
            Key::Up => {player_pos.1 -= 1},
            Key::Down => {player_pos.1 += 1},
            _ => {},
        }
        write!(stdout, "{}{}{}@", termion::clear::All, termion::cursor::Goto(player_pos.0, player_pos.1), termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();
    }
}
