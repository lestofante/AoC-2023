use std::{collections::HashMap, usize};

const BASE: usize = (b'Z' - b'A' + 1) as usize;

fn load(section: &str) -> (String, HashMap<usize, (usize, usize)>) {
    let mut lines = section.lines();

    let directions = lines.next().unwrap();

    let empty = lines.next().unwrap();
    assert!(empty == "", "found -{empty}-");
    
    let mut my_map: HashMap<usize, (usize, usize)> = HashMap::new();

    for code in lines{
        let raw_dogging_the_parse = code.as_bytes();
        assert!(raw_dogging_the_parse.len() == 16, "len is {}", raw_dogging_the_parse.len());
        
        //println!();
        //println!();
        let mut a: usize = 0;
        for index in 0..3{
            //print!("{}", raw_dogging_the_parse[index] as char);
            a *= BASE;
            a += (raw_dogging_the_parse[index] - b'A') as usize;
        }

        let mut b: usize = 0;
        for index in 7..10{
            //print!("{}", raw_dogging_the_parse[index] as char);
            b *= BASE;
            b += (raw_dogging_the_parse[index] - b'A') as usize;
        }
        //println!();

        let mut c: usize = 0;
        for index in 12..15{
            //print!("{}", raw_dogging_the_parse[index] as char);
            c *= BASE;
            c += (raw_dogging_the_parse[index] - b'A') as usize;
        }
        //println!();
        
        my_map.insert(a, (b, c));
    }

    (directions.to_owned(), my_map)
}

fn w(mut n: usize) -> String{
    let mut ris: [char;3] = ['\0';3];
    for i in 0..3{
        ris[2-i] = (n % BASE + (b'A' as usize)) as u8 as char;
        n /= BASE;
    }
    ris.iter().collect()
}

fn main() {
    let input_file_sections = include_str!("../../../data/input.txt");

    let data = load(input_file_sections);

    let mut where_am_i = 0;
    let mut where_i_go = 0;

    for _ in 0..3{
        where_i_go *= BASE;
        where_i_go += BASE-1 as usize;
    }
    println!("start {} end {}", w(where_am_i), w(where_i_go));

    
    let mut  direction = data.0.chars().cycle();
    let map = data.1;
    let mut step = 0;
    while where_am_i != where_i_go{
        let d = direction.next().expect("there shold be data in the array");
        step += 1;
        let new_pos = map.get(&where_am_i).expect("should never fail");
        
        if d == 'L'{
            where_am_i = new_pos.0;
        }else if d == 'R'{
            where_am_i = new_pos.1;
        }else{
            assert!(false, "direction not found");
        }
        if where_am_i == where_i_go{
            break;
        }

        //println!("I am at {}", w(where_am_i));
    }

    println!("Stepped {step}");
}