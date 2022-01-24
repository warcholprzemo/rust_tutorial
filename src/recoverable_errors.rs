use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn classic_ok_error() {
    let f = File::open("progress.txt");
    let f = match f {
        Ok(file) => { println!("progress.txt found"); file },
        Err(error) => panic!("Problem opening the file {:?}", error)
    };
}

fn classic_nested_ok_error() {
    let f = File::open("unknown.txt");
    let f = match f {
        Ok(file) => { println!("unknown.txt found"); file },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("unknown.txt") {
                Ok(fc) => { println!("unknown.txt created"); fc },
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error);
            }
        },
    };
}

fn unwrap_and_expect() {
    let f1 = File::open("progress.txt").unwrap();  // default message
    let f2 = File::open("progress.txt").expect("Failed to open file progress.txt"); // custom message
    println!("unwrap_and_expect done");
}

fn read_content_from_file_classic() -> Result<String, io::Error> { // Generic type, must return Ok or Err
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_content_from_file_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("username2.txt")?;  // the "?" automatically creates OK / Err
    let mut s = String::new();
    f.read_to_string(&mut s)?; // again return Err if cannot read
    Ok(s)
}

fn read_content_from_file_question_mark_combine() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_to_string_shortcut() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

pub fn main_recoverable_errors() {
    classic_ok_error();
    classic_nested_ok_error();
    unwrap_and_expect();

    let username_result: Result<String, io::Error> = read_content_from_file_classic(); // Ok("joemonster\n") or Err
    println!("username_result {:?}", username_result);
    println!("Just username: {}", username_result.unwrap()); 
    let username_result: Result<String, io::Error> = read_content_from_file_question_mark();
    println!("username_question_mark {:?}", username_result);
    let username_result: Result<String, io::Error> = read_content_from_file_question_mark_combine();
    println!("username_combine {:?}", username_result);
    let username_result: Result<String, io::Error> = read_to_string_shortcut();
    println!("username_shortcut {:?}", username_result);
}
