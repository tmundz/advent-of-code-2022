/*
 * Step 1 parse file
 * Step 2 addvalues from the file sperating by a blank new line \n 
 * Step 3 compare values 
 * Step 4 return elf with the most calories
 * */
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::vec::Vec;


fn parse_phrase<'a, Iter: Iterator<Item = &'a str>>(split: Iter) -> Vec<i32> {
    let mut total: i32 = 0;
    let mut vec: Vec<i32> = Vec::new();

    for i in split {
        if i == "" {
            vec.push(total);
            total = 0; 
            continue;
        } else {
                        
            total +=  i32::from_str(i).unwrap_or(0);
        }
    }
    vec
}

//opens file 
fn get_phrase() -> Vec<i32> {
    let mut file = File::open("calories.txt").expect("when");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Does not exist");
    let split_content = content.split("\n");
    let vec:Vec<i32> = parse_phrase(split_content);
    vec
}

// logic for the calories question 
pub fn run_calories(){
    let mut vec: Vec<i32> = get_phrase();
    let max = vec.iter().max().unwrap();
    println!("{}", max);
    let mut last3: i32 = 0; 
    vec.sort();
    for i in 1..4 {
        let index:usize = vec.len() - i;
        last3 += vec[index];

    }
    println!("{}", last3);
    




}
