mod pet;

use pet::Pet;
use std::io;

fn main() {
    // name
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    let mut pet = Pet::new(name);
    
    pet
        .status()
        .play()
        .play()
        .feed()
        .status();
}