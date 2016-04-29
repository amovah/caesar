extern crate caesar;

use caesar::{ encrypt as enc, decrypt as dec };

fn main() {
    let mut mode = String::new();
    let mut text = String::new();

    for (index, arg) in std::env::args().skip(1).enumerate() {
        if index == 0 {
            mode = arg;
        } else {
            text = text + &arg + &" ";
        }
    }

    if mode == "encrypt".to_string() {
        println!("{:?}", enc(&text));
    }

    if mode == "decrypt".to_string() {
        println!("{:?}", dec(&text));
    }
}
