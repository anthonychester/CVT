use crate::*;

use csv::StringRecord;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

pub struct CFR {
  
}

impl CFR {
  pub fn _new() -> Self {
    return CFR{};
  }
  pub fn run() -> Result<Vec<Episode>, Box<dyn Error>> {
    //let mut oss = OsString::new();
    let file_path: OsString = OsString::from("one_piece.csv");//get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut ep_vec: Vec<Episode> = vec!();
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);
        ep_vec.push(CFR::strrectoep(record));
        //ep_vec.push(Episode::new(record[5], record[0], record[6], record[8], record[7]));
    }
    Ok(ep_vec)
}

   fn strrectoep(record: StringRecord) -> Episode {
  let name = record[5].to_string();
  let ep = record[4].parse::<i32>().unwrap();
  let year = record[6].parse::<i32>().unwrap();
  let rank = record[8].parse::<f32>().unwrap();
  let votes: i32;
  votes = match record[7].parse::<i32>() {
        Ok(n)   => n,
        Err(_e)  => 0,
    };
  //let votes = record[7].parse::<i32>().unw;
  return Episode::new(name, ep, year, rank, votes);
}
}