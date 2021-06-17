//https://www.hackerearth.com/practice/data-structures/arrays/1-d/tutorial/

// INPUT
/**
 * 5
 * 4
 * 12
 * 7
 * 15
 * 9
 */

// OUTPUT
/**
 * 5
 * 4
 * 12
 * 7
 * 15
 * 9
 */

use std::io;
use std::io::prelude::*;

fn main() {
    let mut vec_len = String::new();
    io::stdin().read_line(&mut vec_len).unwrap();
    let number: i32 = vec_len.trim().parse().unwrap();
    let mut iterator = number;
    let mut current_num;
    let mut vec = Vec::new();
    while iterator != 0 {
        current_num = String::new();
        io::stdin().read_line(&mut current_num).unwrap();
        let temp = current_num.trim().parse::<i32>().unwrap();
        vec.push(temp);
        iterator -= 1;
    }

    iterator = number;

    while iterator != 0 { 
        println!("{}", vec.pop().unwrap());
        iterator -= 1;
    }
             
    // Writing output to STDOUT
}
