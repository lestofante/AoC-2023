use std::{collections::HashMap, usize};

const BASE: usize = (b'Z' - b'A' + 1) as usize;

fn load(section: &str) -> (String, HashMap<usize, (usize, usize)>, Vec<usize>) {
    let mut lines = section.lines();

    let directions = lines.next().unwrap();

    let empty = lines.next().unwrap();
    assert!(empty == "", "found -{empty}-");
    
    let mut my_map: HashMap<usize, (usize, usize)> = HashMap::new();

    let mut starts: Vec<usize> = vec!();

    for code in lines{
        let raw_dogging_the_parse = code.as_bytes();
        assert!(raw_dogging_the_parse.len() == 16, "len is {}", raw_dogging_the_parse.len());
        
        let mut a: usize = 0;
        for index in 0..3{
            a *= BASE;
            a += (raw_dogging_the_parse[index] - b'A') as usize;
        }
        if raw_dogging_the_parse[2] == b'A'{
            starts.push(a);
        }

        let mut b: usize = 0;
        for index in 7..10{
            b *= BASE;
            b += (raw_dogging_the_parse[index] - b'A') as usize;
        }

        let mut c: usize = 0;
        for index in 12..15{
            c *= BASE;
            c += (raw_dogging_the_parse[index] - b'A') as usize;
        }
        
        my_map.insert(a, (b, c));
    }

    (directions.to_owned(), my_map, starts)
}

fn w(mut n: usize) -> String{
    let mut ris: [char;3] = ['\0';3];
    for i in 0..3{
        ris[2-i] = (n % BASE + (b'A' as usize)) as u8 as char;
        n /= BASE;
    }
    ris.iter().collect()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_multiple(values: &[usize]) -> Option<usize> {
    if values.len() < 2 {
        return None;
    }

    let mut result = values[0];
    for &value in values.iter().skip(1) {
        result = lcm(result, value);
    }
    Some(result)
}

fn main() {
    let input_file_sections = include_str!("../../../data/input.txt");

    let data = load(input_file_sections);

    let mut where_am_i = data.2;
    
    let mut  direction = data.0.chars().cycle();
    let map = data.1;
    let mut step = 0;

    let mut rep: Vec<usize> = vec![0; where_am_i.len()];

    println!("solutions {}", where_am_i.len());

    let mut complete = 0;

    while where_am_i.len() != complete {
        step += 1;

        let d = direction.next().expect("there shold be data in the array");

        let mut i = 0;

        while i < where_am_i.len(){
            if where_am_i[i] % BASE == BASE-1{
                i += 1;
                continue;
            }

            let new_pos = map.get(&where_am_i[i]).expect("should never fail");
            
            if d == 'L'{
                where_am_i[i] = new_pos.0;
            }else if d == 'R'{
                where_am_i[i] = new_pos.1;
            }else{
                assert!(false, "direction not found");
            }
            //print!("I am at {}", w(*where_am_i));
            if where_am_i[i] % BASE == BASE-1{
                rep[i] = step;
                println!("{i} step {step} after {} arrived! at {}", step, w(where_am_i[i]));
                //where_am_i.remove(i);
                complete += 1;
            }
            //println!();
            i+=1;
        }
        if step % 100000 == 0{
            println!("END STEP {step}");
        }
    }

    println!("Stepped {step} MCM {:?}", lcm_multiple(&rep));
}