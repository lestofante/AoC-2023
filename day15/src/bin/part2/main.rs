use std::fmt;

fn get_hash(mut sum: usize, c: char) -> usize {
    sum += c as usize;
    sum = sum.overflowing_mul(17).0;
    sum = sum % 256;
    sum
}

#[derive(Clone, Copy, Debug, Default)]
struct Lens{
    name: usize,
    value: u8,
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}={} ", (self.name >> 8) as u8 as char, self.name as u8 as char, self.value)
    }
}

fn main() {
    let data = include_str!("../../../data/input.txt");

    let mut map: Vec<Vec<Lens>> = vec![vec!(); 256];    

    for item in data.split(","){
        let mut iter = item.chars();

        let mut hash = 0;
        let op;
        let mut name: usize = 0;

        loop {
            let letter = iter.next().unwrap();
            if letter != '=' && letter != '-' {
                hash = get_hash(hash, letter);
                name = (name << 8) + letter as usize;
                print!("{letter}");
            }else{
                op = letter;
                break;
            }
        }
/*
        println!("name -{}{}{}{}{}{}{}{}- hash {hash}",
            (name >> 56) as u8 as char,
            (name >> 48) as u8 as char,
            (name >> 40) as u8 as char,
            (name >> 32) as u8 as char,
            (name >> 24) as u8 as char,
            (name >> 16) as u8 as char,
            (name >> 8) as u8 as char, 
            name as u8 as char);
*/
        match op {
            '-' => {
                if let Some(index) = map[hash].iter().position(|e|e.name == name){
                    map[hash].remove(index);
                }
            },
            '=' => {
                let value = iter.next().unwrap() as u8 - b'0';
                if let Some(index) = map[hash].iter().position(|e|e.name == name){
                    map[hash][index].value = value;
                }else{
                    map[hash].push(Lens{name, value});
                }
            },
            a => assert!(false, "nbed char: {a}"),
        };
        /*
        for m in map.iter().enumerate(){
            if m.1.len() <= 0{
                continue;
            }
            print!("box {}: ", m.0);
            for v in m.1{
                print!("{} ", v);
            }
            println!();
        }
        println!();
*/
        if let Some(f) = iter.next(){
            assert_eq!(f, '\n');
        }
    }
    let mut sum_power = 0;
    for (x, boxx) in map.iter().enumerate(){
        let mut box_power = 0;
        for (y, lens) in boxx.iter().enumerate(){

            let power = (1+x) * (1+y) * lens.value as usize;
            box_power += power;
            println!("{x} {} {power} lens.value {}", y+1, lens.value);
        }
        println!("{x} box_power {box_power}");
        sum_power += box_power;
    }

    println!("sum_power {sum_power}");
    

}
