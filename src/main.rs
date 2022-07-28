mod line;
use line::*;

extern crate termion;

use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::io;
use std::io::{stdin, stdout, Write};

//tutorial-read-01.rs
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use csv::StringRecord;

fn run() -> Result<Vec<Episode>, Box<dyn Error>> {
    //let mut oss = OsString::new();
    let file_path: OsString = OsString::from("one_piece.csv");//get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut ep_vec: Vec<Episode> = vec!();
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);
        ep_vec.push(strrectoep(record));
        //ep_vec.push(Episode::new(record[5], record[0], record[6], record[8], record[7]));
    }
    Ok(ep_vec)
}

pub fn strrectoep(record: StringRecord) -> Episode {
  let name = record[5].to_string();
  let ep = record[4].parse::<i32>().unwrap();
  let year = record[6].parse::<i32>().unwrap();
  let rank = record[8].parse::<f32>().unwrap();
  let mut votes: i32;
  votes = match record[7].parse::<i32>() {
        Ok(n)   => n,
        Err(e)  => 0,
    };
  //let votes = record[7].parse::<i32>().unw;
  return Episode::new(name, ep, year, rank, votes);
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
/*
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
*/

pub struct Pos {
  x: u16,
  y: u16,
  s: u16,
  mx: u16,
  my: u16
}

#[derive(Debug, Clone)]
pub struct Episode {
  name: String,
  episode_number: i32,
  year: i32,
  rank: f32,
  votes: i32
  
}

impl Episode {
    pub fn new(init_name : String, init_episode_number: i32, init_year: i32, init_rank: f32, init_votes: i32) -> Self {return Episode{name: init_name,episode_number: init_episode_number,year: init_year,rank: init_rank,votes: init_votes};}
}

pub struct Square {
  w: u16,
  h: u16,
  c: u8
}

impl Square {
  pub fn new(init_w: u16, init_h: u16, init_c: u8) -> Self {return Square{w: init_w, h: init_h, c: init_c};}
}

fn main() {

  let mut stdout = stdout().into_raw_mode().unwrap();
  let stdin = stdin();

  let (x, y) = termion::terminal_size().unwrap();
  let mut mode = 'm';
  let mut pos = Pos{x: 1, y: 1, s: 1, mx: x, my: y};

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
  let title_x = (x/2)-(40/2);
  let title_start = 6;

  print!("{}", termion::cursor::Hide);
  write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
  stdout.flush().unwrap();
  for i in 0..title.len() {
    println!("{}{}{}", color::Fg(color::Green), termion::cursor::Goto(title_x, (title_start + i).try_into().unwrap()), title[i]);
  }
  println!("{}{}Start", color::Fg(color::Red), termion::cursor::Goto((x/2)-2, 21));
  println!("{}{}Settings", color::Fg(color::Red), termion::cursor::Goto((x/2)-4, 23));
  println!("{}{}Quit", color::Fg(color::Red), termion::cursor::Goto((x/2)-2, 25));

  print!("{}{}➤",termion::cursor::Goto((pos.mx/2)-3, 20), color::Fg(color::Blue));
stdout.flush().unwrap();

  //load 
  let result = run();

  let ep_vec = match result {
    Ok(v) => v,
    Err(m) => { println!("{}", m);
                process::exit(1) },
  };

    let mut count = 0;
    let mut c_ep = 0;
  
    //detecting keydown events
    for c in stdin.keys() {
          let mut enter = false;
        match c.unwrap() {
            //Key::Ctrl('h') => println!("Hello world!"),
            Key::Ctrl('q') => {println!("{}", termion::cursor::Goto(1, y)); break},
            Key::Up => pos = movec('u', mode, pos),
            Key::Down => pos = movec('d', mode, pos),
            Key::Left => pos = movec('l', mode, pos),
            Key::Right => pos = movec('r', mode, pos),
            Key::Char('\n') => enter = true,
            _ => (),
        }

        if enter {
          if pos.s == 1 {
            write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
            stdout.flush().unwrap();
            mode = 'd';
            //Start
            println!("{}{}▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄", color::Fg(color::Black), termion::cursor::Goto(x-29, 4));
            for i in 0..11 {
              println!("{}█                      █", termion::cursor::Goto(x-29, 4+1+i));
            }
            println!("{}▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀", termion::cursor::Goto(x-29, 15));
            println!("{}{}Name: ", color::Fg(color::LightBlack), termion::cursor::Goto(2, 5));
            println!("{}EP: ", termion::cursor::Goto(2, 8));
            println!("{}Year: ", termion::cursor::Goto(2, 9));
            println!("{}Rank: ", termion::cursor::Goto(2, 10));
            println!("{}Votes: ", termion::cursor::Goto(2, 11));
          } else {
            break;
          }
        }
        //Update
        let ep = ep_vec[c_ep].clone();//Episode::new("I'm Luffy! The Man Who Will Become the Pirate King!".to_string(), 1, 1999, 7.6, 647);
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
      let sq = to_sq(ep);
      let ws: u16 = (12 - (sq.w/2)) + x-29;
      let hs: u16 = (5 - (sq.h/2)) + 5;
      for xc in 0..sq.w {
        for yc in 0..sq.h {
          print!("{}{}█", color::Fg(color::Rgb(0, 0, 55+(sq.c * 10))), termion::cursor::Goto(ws+xc, hs+yc));
        }
      }

        if count % 1 == 0 {
          c_ep += 1;
        } else {
          count += 1;
          for i in 0..11 {
              println!("{}{}█                      █",color::Fg(color::Black), termion::cursor::Goto(x-29, 4+1+i));
            }
        }
      
        stdout.flush().unwrap();
    }
}
pub fn movec(dir: char, mode: char, mut pos: Pos) -> Pos {
  if mode == 'm' {

    if pos.s == 1 {
    print!("{}{} ",termion::cursor::Goto((pos.mx/2)-3, 20), color::Fg(color::Blue));
    } else if pos.s == 2 {
      print!("{}{} ",termion::cursor::Goto((pos.mx/2)-5, 22), color::Fg(color::Blue));
    } else if pos.s == 3 {
      print!("{}{} ",termion::cursor::Goto((pos.mx/2)-3, 24), color::Fg(color::Blue));
    }
    
    if dir == 'd' && pos.s < 3 {
      pos.s += 1;
    }
    if dir == 'u' && pos.s > 1 {
      pos.s -= 1;
    } 
    
    if pos.s == 1 {
    print!("{}{}➤",termion::cursor::Goto((pos.mx/2)-3, 20), color::Fg(color::Blue));
    } else if pos.s == 2 {
      print!("{}{}➤",termion::cursor::Goto((pos.mx/2)-5, 22), color::Fg(color::Blue));
    } else if pos.s == 3 {
      print!("{}{}➤",termion::cursor::Goto((pos.mx/2)-3, 24), color::Fg(color::Blue));
    }
    
  }
  return pos;
}

pub fn to_sq(ep: Episode) -> Square {
  //Width votes
  // Height rank
  // Color year
  //24 by 31
  let h = ep.rank.trunc() as u16;
  let mut w = (ep.votes / 31) as u16;
  let c = (ep.year - 1999) as u8;
  if w > 24 { w = 24 }
  return Square::new(w, h, c);
}

//chmod u+x cvt