#![deny(trivial_casts)]

use std::io::prelude::*;
use std::io::stdout;

extern crate crossterm;
use crossterm::event::{read, Event};
use crossterm::{
    cursor, queue,
    style::Print,
    terminal,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

mod keycodes;
use keycodes::extract;

fn main() -> Result<()> {
    let _screenoffset: usize = 0;
    let mut command = String::new();
    let mut autoparse = String::new();

    let mut out = stdout();

    let screensize = crossterm::terminal::size()?;
    let screenheight = screensize.1;

    queue!(out, terminal::Clear(terminal::ClearType::All))?;
    queue!(out, cursor::MoveTo(0, 0))?;
    queue!(out, Print("Screenheight is ".to_string()))?;
    queue!(out, Print(screenheight.to_string()))?;
    out.flush()?;

    enable_raw_mode()?;

    let mut quitnow = false;
    while quitnow == false {
        if autoparse.is_empty() {
            let key = read()?;
            let mut keycode: char = '\u{00}';
            // This is close to the old c-style 'getch()':
            match key {
                Event::Key(event) => {
                    keycode = extract(event.code).unwrap_or('\u{00}');
                }
                Event::Mouse(_event) => (), // This can be handled later
                Event::FocusGained => (),   // This can be handled later
                Event::FocusLost => (),     // This can be handled later
                Event::Paste(_text) => (),  // This can be handled later
                Event::Resize(_width, _height) => (), // This can be handled later
            };
            command.push_str(&keycode.clone().to_string());
        } else {
            command.push(autoparse.chars().nth(0).unwrap());
            autoparse.remove(0);
        }

        let text = format!("{}", command);
        queue!(out, terminal::Clear(terminal::ClearType::All))?;
        queue!(out, cursor::MoveTo(0, 0))?;
        queue!(out, Print(text))?;

        queue!(out, cursor::MoveTo(0, 0))?; // style::PrintStyledContent( " ".magenta()))?;
        out.flush()?;

        if command.len() >= 5 {
            quitnow = true;
        } else {
            quitnow = false;
        }
    }
    queue!(out, terminal::Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}
