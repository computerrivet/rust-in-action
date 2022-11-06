use std::fs::File;
use std::io::BufReader; 
use std::io::prelude::*;

fn main() {

    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

   // let mut line = String::new();


    for line_result in reader.lines() {
        let line = line_result.unwrap();

        println!("{} ({} bytes long)", line, line.len())
    }


    // loop {
    //     let len = reader.read_line(&mut line) 
    //         .unwrap();

    //     if len == 0 {
    //         break;
    //     }

    //     println!("{} ({} bytes long)", line, len);

    //     line.truncate(0);
    // }

}
