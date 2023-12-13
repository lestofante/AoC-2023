
struct Map{
    rows: Vec<usize>,
    cols: Vec<usize>,
    rows_len: usize,
    cols_len: usize,
}

impl Map {
    fn new() -> Map {
        Map{
            rows: vec!(),
            cols: vec!(),
            rows_len: 0,
            cols_len: 0,
        }
    }
}

fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn parse(data:&str) -> Vec<Map>{
    let mut ris: Vec<Map> = vec!();
    let mut map:Map = Map::new();
    for line in data.lines(){

        // empty line, start of new map
        if line.is_empty(){
            ris.push(map);
            map = Map::new();
            continue;
        }

        let mut sum_line: usize = 0;

        for (i, c) in line.chars().enumerate(){
            
            let sum = if c == '#'{
                1
            }else{
                0
            };

            sum_line = sum_line << 1;
            sum_line += sum;

            if let Some(value) = map.cols.get_mut(i) {
                *value = *value<<1;
                *value += sum;
            } else {
                map.cols.insert(i, sum);
                map.cols_len += sum;
            }
        }
        map.rows_len = line.len();
        map.rows.push(sum_line);
    }
    ris.push(map);

    ris
}

fn print(map: &Map){
    println!("--------");
    for n in &map.rows{
        println!("{n:0width$b}", width=map.rows_len);
    }
}

fn find_possible_match(map: &Vec<usize>) -> Option<usize>{
    let len = map.len();
    for a in 0..len - 1 {
        if map[a] == map[a+1]{
            println!("possible match at {a} {:b} {:b}", map[a], map[a+1]);
            if check_mirror(a, map){
                return Some(a+1);
            }
        }
    }
    return None;
}

fn check_mirror(a: usize, map: &[usize]) -> bool {
    let len = map.len();
    let b = a+1;
    let min = std::cmp::min(b, len-b);
    println!("min {min} at {} {}", a, len-b);
    for i in 1..min{
        println!("checkin match {i} at {} {} {:b} {:b}", a-i, b+i, map[a-i], map[b+i]);
        if map[a-i] != map[b+i]{
            return false;
        }
    }
    true
}

fn main() {
    let data = include_str!("../../../data/input.txt");

    let data = parse(data);

    let mut sum = 0;
    for v in &data{
        print(v);
        let a = find_possible_match(&v.rows);
        if let Some(a) = a{
            sum += 100*a;
        }
        println!("possible rows split {a:?}");
        let b = find_possible_match(&v.cols);
        if let Some(b) = b{
            sum += b;
        }
        println!("possible cols split {b:?}");
    }
    println!("sum {sum}");
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        let numbers = vec![1, 2, 4, 6, 8, 16, 24, 32, 33, 63, 64, 65];

        for &number in &numbers {
            if is_power_of_two(number) {
                println!("{} is a power of 2", number);
            } else {
                println!("{} is not a power of 2", number);
            }
        }
      
    }
}