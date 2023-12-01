use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Potential{
    string: &'static str,
    iter: std::iter::Peekable<std::str::Chars<'static>>,
    value: u32,
}

impl Potential {
    fn new(input_string: &'static str, value: u32) -> Self {
        Potential {
            string: input_string,
            iter: input_string.chars().peekable(),
            value: value,
        }
    }
    
    fn add(&mut self, c: char) -> Option<u32> {
        if let Some(next) = self.iter.next() {
            if next != c{
                self.iter = self.string.chars().peekable();
                if let Some(next) = self.iter.peek(){
                    if *next == c{
                        self.iter.next();
                    }
                }
            }
            if self.iter.peek().is_none(){
                self.iter = self.string.chars().peekable();
                return Some(self.value);
            }
        }
        return None;
    }
}

fn main() {
    
    println!("start");
    let mut sum = 0;
    if let Ok(lines) = read_lines("./data/inputs.txt") {
        println!("file ok");
        for line in lines {
            if let Ok(line) = line {
                println!("");
                let mut potential_string: [Potential; 9] = [
                    Potential::new ("one",  1), 
                    Potential::new ("two",  2), 
                    Potential::new ("three",3),
                    Potential::new ("four", 4), 
                    Potential::new ("five", 5), 
                    Potential::new ("six",  6), 
                    Potential::new ("seven",7), 
                    Potential::new ("eight",8), 
                    Potential::new ("nine", 9), 
                ];
                let mut first: Option<u32> = None;
                let mut second: Option<u32> = None;

                for c in line.chars() {
                    for pot in &mut potential_string{
                        if let Some(value) = pot.add(c){
                            println!("got str value {value}");
                            if first.is_none(){
                                first = Some(value);
                            }else {
                                second = Some(value);
                            }
                        }
                    }
                    if c.is_digit(10){
                        println!("got digit value {c}");
                        if first.is_none(){
                            first = c.to_digit(10);
                        }else {
                            second = c.to_digit(10);
                        }
                    }
                }

                let first = first.unwrap_or(0);
                let second = second.unwrap_or(first);
                println!("{first}{second} from {line}");
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_4() {
        let mut f = Potential::new ("four", 4);
        let mut res = 0 as u32;
        for c in "95pkmbpphdmninenthphrzknhgvzfour".chars(){
            if let Some(v) = f.add(c){
                res = v;
            }
        }
        assert_eq!(res, 4);
    }

    #[test]
    fn test_add_6() {
        let mut f = Potential::new ("six", 6);
        let mut res = 0 as u32;
        for c in "7qcjzchtkssix".chars(){
            if let Some(v) = f.add(c){
                res = v;
            }
        }
        assert_eq!(res, 6);
    }

    

}