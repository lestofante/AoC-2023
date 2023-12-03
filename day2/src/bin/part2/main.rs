use std::collections::HashMap;

macro_rules! struct_map {
    (struct $name:ident: $ftype:ty { $($fname:ident),* $(,)* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn from_string(self, maybe_key: &str) -> Option<$ftype> {
                match maybe_key{
                    $(
                        stringify!($fname) => Some(self.$fname),
                    )*
                    _ => None,
                }
            }
        }
    }
}

struct_map! {
    struct Mappa: u32 {
        red,
        blue,
        green,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_map() {
        let my_instance = Mappa { red: 42, blue: 24, green: 12 };
        let result = my_instance.from_string("blue");
        assert_eq!(result.unwrap(), 24);
    }

}

fn main() {
    let input_file = include_str!("../../../data/input.txt");
    // Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red
    let mut sum_power = 0;
    for (index, line) in input_file.lines().enumerate() {
        let game: Vec<&str> = line.split(": ").collect();
        assert!(game.len() == 2, "Unexpected format of games");

        let game = game[1].split([',', ';']);
        let mut max_color = HashMap::from([
            ("red", 0),
            ("blue", 0),
            ("green", 0),
        ]);
        for extraction in game{
            println!("extraction is {extraction}");
            let extraction: Vec<&str> = extraction.trim().split(' ').collect();
            assert!(extraction.len() == 2, "Unexpected format of extraction");
            
            let mut num = 0;
            for c in extraction[0].chars(){
                if c.is_digit(10){
                    num *= 10;
                    num += c.to_digit(10).unwrap();
                }else{
                    break;
                }
            }
            max_color.entry(extraction[1]).and_modify(|val| if *val < num{*val = num});

        }
        let power = *max_color.get("red").unwrap() * *max_color.get("green").unwrap() * *max_color.get("blue").unwrap();
        println!("game {index} power is {power} {} {} {}", *max_color.get("red").unwrap(), *max_color.get("green").unwrap(), *max_color.get("blue").unwrap());
        sum_power += power;
    }
    println!("sum is {sum_power}");
}
