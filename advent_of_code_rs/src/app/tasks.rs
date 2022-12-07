use std::fs::File;
use std::io::{prelude::*, BufReader};

struct TaskRange {
    start_task: i32,
    end_task: i32,
}

impl TaskRange {
    fn contain(&self, other: &Self) -> bool {
        self.start_task <=other.start_task && self.end_task >= other.end_task
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start_task <= other.end_task && other.start_task <= self.end_task            
    }

}

trait Converts {
    fn get_task(range: &str) -> Self;
}

impl Converts for TaskRange {
    fn get_task(range: &str) -> TaskRange {
        let (str_start, str_end) = if let Some((str_start, str_end)) = range.split_once("-") {
            (str_start, str_end) 
        } else { todo!() };
        
        let start: i32 = str_start.parse::<i32>().unwrap();
        let end: i32 = str_end.parse::<i32>().unwrap();
        TaskRange { start_task: (start), end_task: (end) }
    }
}

fn find_same() -> i32 {
    let file = File::open("tasks.txt").expect("not found");
    let groups = BufReader::new(file).lines();
    groups.filter_map(|line| {
        if let Some((tasks1, tasks2)) = line.expect("Error").split_once(",") { (tasks1, tasks2);
            let elf1: TaskRange = Converts::get_task(tasks1);
            let elf2: TaskRange = Converts::get_task(tasks2);
            (elf1.contain(&elf2) || elf2.contain(&elf1)).then_some(())
        } else { todo!()}
    }).count().try_into().unwrap()
}

fn find_overlap() -> i32 {
    let file = File::open("tasks.txt").expect("not found");
    let groups = BufReader::new(file).lines();
    groups.filter_map(|line| {
        if let Some((tasks1, tasks2)) = line.expect("Error").split_once(",") { (tasks1, tasks2);
            let elf1: TaskRange = Converts::get_task(tasks1);
            let elf2: TaskRange = Converts::get_task(tasks2);
            (elf1.overlaps(&elf2) || elf2.overlaps(&elf1)).then_some(())
        } else { todo!()}
    }).count().try_into().unwrap()
}

pub fn run_tasks() {
    let total: i32 = find_same();
    println!("{}", total); 
    let overlap_total: i32 = find_overlap();
    println!("{}", overlap_total);
}



