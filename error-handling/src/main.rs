use std::env;
use std::fs::{self, File};
use std::process;
use std::io::{self, stdin};

// reding a given file name
fn read_file(file_name: &str) -> Result<File, io::Error> {
    let cwd = env::current_dir().unwrap();
    //println!("Current working directory: {:?}", cwd);

    let result = File::open(file_name);
    match result {
        Ok(file) => Ok(file),
        Err(error) => Err(error)
    }
}

fn write_to_file() -> Result<String, io::Error> {
    //first user input
    println!("What file would you like to write to?");
    let mut buf_file_name: String = String::new();
    let result = stdin().read_line(&mut buf_file_name);
    if let Err(error) = result {
        eprintln!("Some error occured while reading user file_name input, {error:?}");
        return Err(error);
    }

    //second user input and writing it to the file
    println!("What would you like to write to the file?");
    let mut buf_file_content = String::new();
    let result = stdin().read_line(&mut buf_file_content); 
    if let Err(error) = result {
       eprintln!("Some error occured while reading user content input, {error:?}");
        return Err(error);
    }
    
    let result = fs::write(buf_file_name.trim(), buf_file_content.trim());
    if let Err(error) = result {
        eprintln!("Some error occured while writing user content into file, {error:?}");
        return Err(error);
    }

    println!("All operations were successful!");
    Ok(buf_file_name)
    
}

fn main() {

    match write_to_file() {
        Ok(result) => println!("Content saved to file {result}"),
        Err(error) => {
            eprintln!("Something bad happened!, {error:?}");
            process::exit(1);
        }
    }
}
