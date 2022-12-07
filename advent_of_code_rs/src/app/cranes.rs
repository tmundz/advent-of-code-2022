use std::fs::File;
use std::io::{prelude::*, BufReader};

//Create a stack data structures to help solve day 5 problem

struct Stack <'a> {
    stack: Vec<Vec<&'a str>>,
}


impl <'a>Stack <'a> {
    fn pop(&mut self, index: usize) -> Option<&str> {
        self.stack[index].pop()
    }

    fn push(&mut self, item: &'a str, index:Option<usize>) {
        match index {
            None => self.stack.push(vec!(item)),
            _ => self.stack[index.unwrap()].push(item),
        }
    }
}


fn parse_file() {
    let file = File::open("test-crane.txt").expect("not found");
    let groups = BufReader::new(file).lines();
}

pub fn run_crates() {
    //parse_file();
    let mut stack = Stack{ stack: vec![Vec::new()]};
    //println!("{:#?}", stack);
    stack.push("a", None);
    //println!("{:#?}", stack);
    stack.push("b",Some(0));
    stack.push("c", None);
    stack.push("M", Some(1));
    println!("pop {:#?}", stack.pop(0));
    println!("pop {:#?}", stack.pop(1));
    println!("pop {:#?}", stack.pop(1));

    
}

#[cfg(test)]
#[test]
fn test_crates() { let s = include_str!("test_input");
    assert_eq!(question_1(&s), "CMZ");
}
