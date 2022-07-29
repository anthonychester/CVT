pub mod keys;
pub mod cv;
pub mod cfr;

#[derive(Debug, Clone)]
pub struct Episode {
  pub name: String,
  pub episode_number: i32,
  pub year: i32,
  pub rank: f32,
  pub votes: i32
  
}
  
impl Episode {
    pub fn new(init_name : String, init_episode_number: i32, init_year: i32, init_rank: f32, init_votes: i32) -> Self {return Episode{name: init_name,episode_number: init_episode_number,year: init_year,rank: init_rank,votes: init_votes};}
}

pub struct Square {
  pub w: u16,
  pub h: u16,
  pub c: u8
}

impl Square {
  pub fn new(init_w: u16, init_h: u16, init_c: u8) -> Self {return Square{w: init_w, h: init_h, c: init_c};}

  pub fn from_ep(ep: Episode) -> Square {
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
}

pub struct Pos {
  _x: u16,
  _y: u16,
  pub s: u16,
}