extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use csv::StringRecord;

fn main() {
    match run() {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
        Ok(values) => {
            println!("{:#?}", values);
        },
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn run() -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    
    let mut rdr = csv::Reader::from_reader(file);
    let mut values = Vec::new();

    {
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }


    for result in rdr.records() {
        
        let entry: StringRecord  = result?;

        values.push(entry);
    }

    Ok(values)
}
