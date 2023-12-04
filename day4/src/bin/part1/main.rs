fn main() {
    let input_file = include_str!("../../../data/input.txt");

    let mut sum_won = 0;
    for (index, line) in input_file.lines().enumerate() {
        let title: Vec<&str> = line.split(':').collect();
        assert!(title.len() == 2);
        let lists: Vec<&str> = title[1].split('|').collect();
        assert!(lists.len() == 2);
        let winning: Vec<i32> = lists[0].split_whitespace().map(|s| s.parse().expect("parse error")).collect();
        println!("winning {winning:?}");
        let extracted: Vec<i32> = lists[1].split_whitespace().map(|s| s.parse().expect("parse error")).collect();
        println!("extracted {extracted:?}");
        let mut won = 0;
        for ext in extracted{
            if winning.contains(&ext){
                print!(" found  {ext:?}");
                if won == 0{
                    won = 1;
                }else{
                    won *= 2;
                }
            }
        }
        sum_won += won;
        println!("parital {won:?}");
    }
    println!("won {sum_won}");
}
