/*
 * Step 1 parse file
 * Step 2 addvalues from the file sperating by a blank new line \n 
 * Step 3 compare values 
 * Step 4 return elf with the most calories
 * */
use std::fs::File;
use std::io::prelude::*;


struct elf{
    elf_number: i32
    calories: Vec<i32>,


}



fn parse_phrase<'a, Iter: Iterator<Item = &'a str>>(split: Iter) -> Vec<String> {
    let mut phrase: String = "".to_owned();
    let mut vec: Vec<String> = Vec::new();

    for i in split {
        if i == "" {
            vec.push(phrase);
            phrase = "".to_owned();
            continue;
        } else {
            phrase = phrase + i;
            phrase = phrase + " ";
        }
    }
    vec
}

//opens file 
pub fn get_phrase() -> Vec<String> {
    let mut file = File::open("phrases.txt").expect("when");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Does not exist");

    let phrase_vec: Vec<i32> = parse_phrase(split_content);

    phrase_vec
}

// logic for the calories question 
fn run_calories() -> <> {
    
    




}
