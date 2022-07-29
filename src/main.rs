mod line;
use line::*;
use cv::CV;
use keys::KEYS;
use cfr::CFR;
use line::Episode;
extern crate termion;

use termion::input::TermRead;

use std::process;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

use std::time::Duration;

fn main() {

  
  let mut mode = 'm';
  let mut lmode = mode.clone();
  let mut c_ep = 0;
  
  let mut cv = CV::new();
  
  cv = cv.build_title();

  //load 
  let result = CFR::run();

  let ep_vec = match result {
    Ok(v) => v,
    Err(m) => { println!("{}", m);
                process::exit(1) },
  };

  let (tx, rx) = mpsc::channel();
  let (tx1, rx1): (Sender<usize>, Receiver<usize>) = mpsc::channel();
  let (tx2, rx2): (Sender<i16>, Receiver<i16>) = mpsc::channel();

  KEYS::spwan(tx);
  thread::spawn(move || {
    let _r = rx2.recv();
    thread::sleep(Duration::from_millis(200));
    loop {
    let mut c_ep = 0;
        if c_ep < 917 {
        c_ep += 1;
        tx1.send(c_ep).unwrap();
      }    thread::sleep(Duration::from_millis(100));
    }
    });

    let mut update;
    loop {
      update = false;
      
      match rx.try_recv() {
        Ok(c) => (cv, mode) = KEYS::key(cv, mode, c),
        Err(_e) => ()
      }

      match rx1.try_recv() {
        Ok(_c) => {c_ep += 1; update = true; },
        Err(_e) => ()
      }

      if mode == 'd' && mode != lmode {
        tx2.send(0).unwrap();
        lmode = mode.clone();
      }
      
      if update && mode == 'd' {
        let ep = ep_vec[c_ep-1].clone();
        let sq = Square::from_ep(ep.clone());
        cv = cv.clear();
        cv = cv.update(ep, sq);

        }
      
    std::thread::sleep(std::time::Duration::from_millis(5));
    }
        

}




//chmod u+x cvt