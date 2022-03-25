
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;



pub struct Database {
    database: Vec<FootballResult>
}

impl Database {

    pub fn new() -> Database {
        Database { database: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.database.len()
    }

    // returns number of records read
    pub fn load_database(&mut self, filename: &str) -> Option<usize> {

        let file = File::open(filename).ok()?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // NB line can propagate Err<>, whereas from_csv() don't propagate None
            if let Some(fr) = FootballResult::from_csv(&line.ok()?) {
                let s = format!("{}", fr);
                self.database.push(fr);    
            }
        }

        if self.database.len() > 0 {
            Some(self.database.len())
        }
        else {
            None
        }
    }
}

// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Odds {
    profit: u32,
    stake: u32,
}

impl Odds {
    pub fn new(profit: u32, stake: u32) -> Odds {
        Odds {
            profit,
            stake,
        }
    }

    pub fn from(odds: &str) -> Option<Odds> {
        let ts: Vec<&str> = odds.split('/').collect();
        if ts.len() == 2 {
            match ts[0].parse() {
                Ok(profit) => { 
                    match ts[1].parse() {
                        Ok(stake) => Some(Odds::new(profit, stake)),
                        Err(_) => None }
                    },
                Err(_) => None
            }
        }
        else {
            None
        }        
    }

    pub fn winnings(&self, bet: f32, win: bool) -> f32 {
        if win {
            bet * (((self.stake + self.profit) as f32) / (self.stake as f32))
        }
        else {
            0.0
        }
    }
}

impl fmt::Display for Odds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Odds: {}/{}", self.profit, self.stake)?;
        if self.profit < self.stake {
            write!(f, ", Odds on")
        }
        else if self.profit > self.stake {
            write!(f, ", Odds against")
        }
        else {
            write!(f, ", Odds even")
        }
    }
}

//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct FootballResult {
    date: NaiveDateTime,
    home_team: String,
    away_team: String,
    for_: u8,
    against: u8,
    result: String,
    win: Odds,
    draw: Odds,
    loss: Odds
}

impl FootballResult {

    pub fn from_csv(csv_line: &str) -> Option<Self> {
        // 26/07/2020 15:00,Arsenal,Watford,3,2,Win,51/50,14/5,261/100,2019/20,Premiership
        let ts: Vec<&str> = csv_line.split(',').collect();

        if ts.len() == 11 {
            Some(FootballResult::from_data(
                NaiveDateTime::parse_from_str(ts[0], "%d/%m/%Y %H:%M").ok()?,
                &ts[1], 
                &ts[2],
                ts[3].parse().ok()?,
                ts[4].parse().ok()?,
                ts[5],
                Odds::from(&ts[6])?,
                Odds::from(&ts[7])?,
                Odds::from(&ts[8])?)
            )
        }
        else {
            None
        }
    }

    pub fn from_data(date: NaiveDateTime, home: &str, away: &str, for_: u8, against: u8, result: &str, win: Odds, draw: Odds, loss: Odds ) -> Self {
        FootballResult {
            date,
            home_team: String::from(home),
            away_team: String::from(away),
            for_,
            against,
            result: String::from(result),
            win,
            draw,
            loss
        }
    }

}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for FootballResult {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Display -> Date: {}", self.date.format("%d/%m/%y %H:%M"))?;
        write!(f, " Home: {}, Away: {}", self.home_team, self.away_team)?;
        write!(f, " For: {}, Against: {}", self.for_, self.against)?;
        write!(f, " Win {} Draw {} Loss {}", self.win, self.draw, self.loss)
    }

}






    
