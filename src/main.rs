

mod data;

//use data::*;

fn main() {

    let mut db = data::Database::new();


    if db.load_database("Database.txt") {
        println!("Read {} database entries", db.size())
    }
    else {
        println!("Error reading database")
    }


}
