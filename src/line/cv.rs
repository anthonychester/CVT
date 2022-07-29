extern crate termion;

use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color};

use std::io::{stdout, Stdout, Write};

use crate::*;

pub struct CV {
    stdout: RawTerminal<Stdout>,
    x: u16,
    _y: u16,
    pub pos: Pos,
}

impl CV {
    pub fn new() -> Self {
        let stdout = stdout().into_raw_mode().unwrap();
        let (x, y) = termion::terminal_size().unwrap();
        let pos = Pos {
            _x: 1,
            _y: 1,
            s: 1,
        };
        return CV {
            stdout: stdout,
            x: x,
            _y: y,
            pos: pos,
        };
    }

    pub fn build_title(mut self) -> CV {
        let title = [
            r" ________      ___      ___ _________   ",
            r"|\   ____\    |\  \    /  /|\___   ___\ ",
            r"\ \  \___|    \ \  \  /  / ||___ \  \_| ",
            r" \ \  \        \ \  \/  / /     \ \  \  ",
            r"  \ \  \____  __\ \    / /__     \ \  \ ",
            r"   \ \_______\\__\ \__/ /\__\     \ \__\",
            r"    \|_______\|__|\|__|/\|__|      \|__|",
            r"",
        ];
        let title_x = (self.x / 2) - (40 / 2);
        let title_start = 6;
        print!("{}", termion::cursor::Hide);
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        self.stdout.flush().unwrap();
        for i in 0..title.len() {
            println!(
                "{}{}{}",
                color::Fg(color::Green),
                termion::cursor::Goto(title_x, (title_start + i).try_into().unwrap()),
                title[i]
            );
        }
        println!(
            "{}{}Start",
            color::Fg(color::Red),
            termion::cursor::Goto((self.x / 2) - 2, 20)
        );
        println!(
            "{}{}Settings",
            color::Fg(color::Red),
            termion::cursor::Goto((self.x / 2) - 4, 22)
        );
        println!(
            "{}{}Quit",
            color::Fg(color::Red),
            termion::cursor::Goto((self.x / 2) - 2, 24)
        );

        print!(
            "{}{}➤",
            termion::cursor::Goto((self.x / 2) - 3, 20),
            color::Fg(color::Blue)
        );
        self.stdout.flush().unwrap();
      return self;
    }

    pub fn movec(mut self, dir: char, mode: char) -> CV {
  if mode == 'm' {

    if self.pos.s == 1 {
    print!("{}{} ",termion::cursor::Goto((self.x/2)-3, 20), color::Fg(color::Blue));
    } else if self.pos.s == 2 {
      print!("{}{} ",termion::cursor::Goto((self.x/2)-5, 22), color::Fg(color::Blue));
    } else if self.pos.s == 3 {
      print!("{}{} ",termion::cursor::Goto((self.x/2)-3, 24), color::Fg(color::Blue));
    }
    
    if dir == 'd' && self.pos.s < 3 {
      self.pos.s += 1;
    }
    if dir == 'u' && self.pos.s > 1 {
      self.pos.s -= 1;
    } 
    
    if self.pos.s == 1 {
    print!("{}{}➤",termion::cursor::Goto((self.x/2)-3, 20), color::Fg(color::Blue));
    } else if self.pos.s == 2 {
      print!("{}{}➤",termion::cursor::Goto((self.x/2)-5, 22), color::Fg(color::Blue));
    } else if self.pos.s == 3 {
      print!("{}{}➤",termion::cursor::Goto((self.x/2)-3, 24), color::Fg(color::Blue));
    }
    
  }
  self.stdout.flush().unwrap();
  return self;
}
  pub fn start(mut self) -> CV {
      if self.pos.s == 1 {
            write!(self.stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
            self.stdout.flush().unwrap();
            //Start
            self = self.clear();
            println!("{}{}Name: ", color::Fg(color::LightBlack), termion::cursor::Goto(2, 5));
            println!("{}EP: ", termion::cursor::Goto(2, 8));
            println!("{}Year: ", termion::cursor::Goto(2, 9));
            println!("{}Rank: ", termion::cursor::Goto(2, 10));
            println!("{}Votes: ", termion::cursor::Goto(2, 11));
          }
    return self;
  }
  
  pub fn update(mut self, ep: Episode, sq: Square) -> CV {
    let mut p1_arr: Vec<char> = vec!();
      let mut p2_arr: Vec<char> = vec!();
      let mut p3_arr: Vec<char> = vec!();
      let name_arr: Vec<char> = ep.name.chars().collect();
      for i in 0..name_arr.len() {
        if i < 15 {
          p1_arr.push(name_arr[i]);
        } else if i < 35 {
          p2_arr.push(name_arr[i]);
        } else if i < 55 {
          p3_arr.push(name_arr[i]);
        }
      }
      let p1: String = p1_arr.iter().collect();
      let p2: String = p2_arr.iter().collect();
      let p3: String = p3_arr.iter().collect();
      println!("{}{}{}", color::Fg(color::White), termion::cursor::Goto(8, 5), p1);
      println!("{}{}{}", color::Fg(color::White), termion::cursor::Goto(2, 6), p2);
      println!("{}{}{}", color::Fg(color::White), termion::cursor::Goto(2, 7), p3);
            println!("{}{}", termion::cursor::Goto(6, 8), ep.episode_number);
            println!("{}{}", termion::cursor::Goto(8, 9), ep.year);
            println!("{}{}", termion::cursor::Goto(8, 10), ep.rank);
            println!("{}{}", termion::cursor::Goto(9, 11), ep.votes);

  let ws: u16 = (12 - (sq.w/2)) + self.x-29;
      let hs: u16 = (5 - (sq.h/2)) + 5;
      for xc in 0..sq.w {
        for yc in 0..sq.h {
          print!("{}{}█", color::Fg(color::Rgb(0, 0, 55+(sq.c * 10))), termion::cursor::Goto(ws+xc, hs+yc));
        }
  }
    self.stdout.flush().unwrap();
   return self;
}

  pub fn clear(self) -> CV {
    println!("{}{}▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄", color::Fg(color::Black), termion::cursor::Goto(self.x-29, 4));
            for i in 0..11 {
              println!("{}█                      █", termion::cursor::Goto(self.x-29, 4+1+i));
            }
            println!("{}▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀", termion::cursor::Goto(self.x-29, 15));
    return self;
  }

  
}