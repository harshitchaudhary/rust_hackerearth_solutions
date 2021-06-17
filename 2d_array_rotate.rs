// https://www.hackerearth.com/practice/data-structures/arrays/multi-dimensional/tutorial/

// INPUT
/**
 * 3 5
 * 13 4 8 14 1
 * 9 6 3 7 21
 * 5 12 17 9 3
 */

// OUTPUT
/**
 * 13 9 5
 * 4 6 12
 * 8 3 17
 * 14 7 9
 * 1 21 3
*/

use std::io;
use std::io::prelude::*;

fn main() {
    let mut line_1 = String::new();
    io::stdin().read_line(&mut line_1).unwrap();
    let line_str = line_1.trim().to_string();
    let str_iter = line_str.split(" ");
    let dimension: Vec<&str> = str_iter.collect();
    let height: usize = dimension[0].parse().unwrap();
    let width: usize = dimension[1].parse().unwrap(); 
    
    let mut initial_vec = vec![vec![0i32; width];height];
    
    let mut index = height;
    let mut vec_index = 0;
   
    while index > 0 {
        let mut current_line = String::new();
        io::stdin().read_line(&mut current_line).unwrap();
        let line_str = current_line.trim().to_string();
        let str_iter = line_str.split(" ");
        initial_vec[vec_index] = str_iter.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        vec_index += 1;
        index -= 1;
    }
    // initial_vec have everything in an array 
    //println!("{:?}", initial_vec);


    let mut index_w: usize = 0;
    let mut index_h: usize = 0;



    while index_w < width {
        while index_h < height {
            print!("{} ", initial_vec[index_h][index_w]);
            index_h += 1;
        }
        println!("");
        index_w  += 1;
        index_h = 0;
    }
        
}
