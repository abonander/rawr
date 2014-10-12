#![feature(slicing_syntax)]

extern crate rawr;

use std::io::{print, println};
use rawr::{sub, login};

fn main() {
    let mut stdin = std::io::stdin();

    println("Welcome to the RAWR Test post program.");
    print_flush("Subreddit: ");    

    let sub_name = stdin.read_line().unwrap();

    let subreddit = sub(sub_name.as_slice()).unwrap();    

    print_flush("Username: ");
    let username = stdin.read_line().unwrap();
    
    print_flush("Password: ");
    let password = stdin.read_line().unwrap();
    
    let session = login(username[], password[], false).unwrap();
                

    let title = stdin.read_line().unwrap();
    
                     
}

fn print_flush(s: &str) {
    print(s);
    std::io::stdio::flush();     
}
