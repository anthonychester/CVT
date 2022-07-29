use crate::*;


use termion::event::Key;
use std::io::{stdin, Stdin};

pub struct KEYS {
  
}

impl KEYS {
  pub fn _new() -> Self {
    return KEYS{};
  }

  pub fn key(mut cv: CV, mut mode: char, c: termion::event::Key) -> (CV, char) {
  let mut enter = false;
        match c {
            //Key::Ctrl('h') => println!("Hello world!"),
            Key::Ctrl('q') => panic!(),
            Key::Up => cv = cv.movec('u', mode),
            Key::Down => cv = cv.movec('d', mode),
            Key::Left => cv = cv.movec('l', mode),
            Key::Right => cv = cv.movec('r', mode),
            Key::Char('\n') => enter = true,
            _ => (),
        }

        if enter {
          if cv.pos.s == 1 {
            mode = 'd';
          }
          cv = cv.start();
        }
        //Update
  return (cv, mode);
}
  pub fn spwan(tx: Sender<Key>) {
  thread::spawn(move || {
        let stdin: Stdin = stdin();
         for c in stdin.keys() {
           
        match c {
               Ok(c) => tx.send(c).unwrap(),
               Err(_e) => ()
             }
         }
    });
  }
}
