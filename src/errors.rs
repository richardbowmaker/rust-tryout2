
use std::{fs::File, io};
use std::io::ErrorKind;
use core::num::ParseIntError;
use std::fmt;


// use macro_rules! <name of macro>{<Body>}
macro_rules! add{
    // match like arm for macro
       ($a:expr,$b:expr)=>{
    // macro expand to this code
        {
   // $a and $b will be templated using the value/variable provided to macro
            $a+$b
        }
       };
       ($a:expr,$b:expr,$c:expr)=>{
        // macro expand to this code
            {
       // $a and $b will be templated using the value/variable provided to macro
                $a+$b+$c
            }
        }
    }
   

macro_rules! error_catch{
        // match like arm for macro
           ($call:expr,$mess:expr)=>{
               {
                   match $call {
                        Ok(res) => res,
                        Err(err) => {
                            println!("{} {} {} {}", &$mess, err, file!(), line!());
                            return Err(AppError::from(err))
                        }
                   }
               }
           }
        }



pub enum AppErrorT {
    MyIoError(ErrorKind),
    MyParseIntError,
}

pub struct AppError {
    kind: AppErrorT,
    description: String,
}

// convert from io::Error to MyError
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        let s = format!("io::Error {}", err);
        AppError { kind: AppErrorT::MyIoError(err.kind()), description: s }
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        let s = format!("ParseIntError {} {} {}", err, file!(), line!());
        AppError { kind: AppErrorT::MyParseIntError, description: s }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppError, {}", &self.description)
    }
}

pub fn main() {

    // let s = add!(1, 2, 3);

    // let _file = File::open("nonexistent_file.txt")?; // This generates an io::Error. But because of return type is Result<(), AppError>, it converts to AppError

    match file_open_errs("nosuchfile.txt") {
        Ok(_) => println!("File opened OK"),
        Err(err) => println!("errors::main() {}", &err.description)
    }

    // let f = match File::open("filenanme") {
    //     Ok(f) => f,
    //     Err(err) => 
    //     {
    //         println!("Error in {} at {}", file!(), line!());
    //         return ()
    //     }
    // };

    // let file = error_catch!(File::open("nonexistent_file.txt"), format!("File open error {}", "nonexistent_file.txt"));

    let a = 1 + 2;
}

fn file_open_errs(filename: &str) -> Result<(), AppError> {

    //let f = File::open(filenanme)?;

    let file = error_catch!(File::open(&filename), &format!("File open error {}", &filename));



    // let my_int: i32 = "xyz".parse()?;


    Ok(())


    // Ok(File::open(filenanme)?)


    // let file = File::open(filenanme)?;

    // match file {
    //     Ok(file) => {
    //         println!("Opened file Ok");
    //         Ok(file)
    //     }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => {
    //             println!("File not found error");
    //             Err(MyError)
    //         },
    //         other_error => {
    //             println!("Problem opening the file: {:?}", other_error);
    //             Err(MyError)
    //         }
    //     }
    // }
}