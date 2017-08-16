// https://www.youtube.com/watch?v=VNbkzsnllsU

use std::io::{self, Read};
use std::cmp::max;

fn main() {

    let mut histo = Vec::new();
    let mut buffer = String::new();

    let stdin = io::stdin();

    stdin.lock().read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();

    loop {
        match lines.next() {
            Some(line) => {
                if line.eq("#") {
                    break;
                }

                match line.parse::<i32>() {
                    Ok(number) => histo.push(number),
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    println!("Input: {:?}", histo);

    println!("Output: {:?}", find(&mut histo));
}

fn find(histo: &mut Vec<i32>) -> i32 {

    let mut height_stack: Vec<i32> = Vec::new();
    let mut pos_stack: Vec<i32> = Vec::new();

    let mut temp_height: i32;
    let mut temp_pos: i32 = 0;
    let mut temp_size: i32;
    let mut max_size: i32 = -1;

    for height in histo.into_iter() {

        if height_stack.len() == 0 || *height > height_stack[height_stack.len() - 1] {

            height_stack.push(height.to_owned());
            pos_stack.push(height.to_owned());

        } else if *height < height_stack[height_stack.len() - 1] {

            loop {
                if *height < height_stack[height_stack.len() - 1] {
                    break;
                }

                temp_height = height_stack.pop().unwrap();
                temp_pos = pos_stack.pop().unwrap();

                temp_size = temp_height * (*height - temp_pos);
                max_size = max(temp_size, max_size);
            }

            height_stack.push(*height);
            pos_stack.push(temp_pos);
        }
    }

    loop {
        if height_stack.len() == 0 {
            break;
        }

        temp_height = height_stack.pop().unwrap();
        temp_pos = pos_stack.pop().unwrap();

        temp_size = temp_height * (histo.len() as i32 - temp_pos);
        max_size = max(temp_size, max_size);
    }

    max_size
}