
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

    pub fn load_database(&mut self, filename: &str) -> bool {

        let f = File::open(filename);

        match f {
            Ok(file) => {

                let reader = BufReader::new(file);

                for line in reader.lines() {
                    if line.is_ok() {
                        let fr = FootballResult::from_csv(&line.unwrap());
                        match fr {
                            Some (fr1) => {
                                self.database.push(fr1);
                            }
                            None => {}
                        }
                    }
                }

                true
            },
            Err(_) => false
        }

    
        // if let Ok(lines) = read_lines(filename) {
        //     for l in lines {
        //         if let Ok(line) = l {
        //             FootballResult::from_csv(line)
        //         }
        //     }
        //     return true;
        // }
        // else {
        //     return false;
        // }
    }
}

#[derive(Debug)]
pub struct FootballResult {
    home_team: String,
    away_team: String,
}

impl FootballResult {

    pub fn from_csv(csv_line: &str) -> Option<Self> {
        // 26/07/2020 15:00,Arsenal,Watford,3,2,Win,51/50,14/5,261/100,2019/20,Premiership
        let ts: Vec<&str> = csv_line.split(',').collect();

        if ts.len() == 11 {
            Some(FootballResult::from_data(&ts[0], &ts[1]))
        }
        else {
            None
        }
    }

    pub fn from_data(home: &str, away: &str) -> Self {
        FootballResult {
            home_team: String::from(home),
            away_team: String::from(away),
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
        write!(f, "Display -> Home: {}, Away: {}", self.home_team, self.away_team)
    }
}






    
