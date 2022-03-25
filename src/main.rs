

// https://doc.rust-lang.org/stable/std/

mod data;
mod errors;

//use data::*;

fn main() {

    errors::main();


    // let mut db = data::Database::new();


    // let o = data::Odds::from("12/23").unwrap();
    // println!("{} {} {} {}", o, o.winnings(23.0, true), o.winnings(46.0, true), o.winnings(34.0, false));


    // if db.load_database("Database.txt").is_some() {
    //     println!("Read {} database entries", db.size())
    // }
    // else {
    //     println!("Error reading database")
    // }


}
