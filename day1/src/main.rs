use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("start");
    let mut sum = 0;
    if let Ok(lines) = read_lines("./data/inputs.txt") {
        println!("file ok");
        for line in lines {
            if let Ok(line) = line {
                let mut first: Option<u32> = None;
                let mut second: Option<u32> = None;
                for c in line.chars() { 
                    if c >= '0' && c <= '9'{
                        if first.is_none(){
                            first = c.to_digit(10);
                        }else {
                            second = c.to_digit(10);
                        }
                    }
                }
                let first = first.unwrap_or(0);
                let second = second.unwrap_or(first);
                println!("{first}{second}: from {line}");
                sum += first * 10 + second;
            }
        }
    }
    println!("sum is {sum}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}