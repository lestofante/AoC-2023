use std::collections::HashMap;

fn main() {
    let input_file = include_str!("../../../data/input.txt");
    // Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red
    let mut sum_id = 0;
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
        if *max_color.get("red").unwrap() <= 12 &&
           *max_color.get("green").unwrap() <= 13 &&
           *max_color.get("blue").unwrap() <= 14
        {
            sum_id += index + 1; //index + 1 = game_id
        }
    }
    println!("sum is {sum_id}");
}
