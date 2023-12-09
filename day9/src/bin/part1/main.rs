
fn load(section: &str) -> Vec<Vec<i64>> {
   section.lines().map(|line| line.split_ascii_whitespace().map(|v| v.parse::<i64>().unwrap()).collect()).collect()
}

fn estimate(report: Vec<i64>) -> i64{
    let mut tmp: Vec<Vec<i64>> = vec!();
    tmp.push(report);

    let mut ris: i64 = 0;
    loop{
        let last = tmp.last().unwrap();
        let mut new: Vec<i64> = vec!();
        let mut all_zero = true;
        for i in 1..last.len(){
            let ris = last[i] - last[i-1];
            new.push( ris );
            if ris != 0{
                all_zero = false;
            }
        }
        ris += last.last().unwrap();
        if all_zero{
            break;
        }
        tmp.push(new);
    }
    println!("result: {ris}");
    ris
}

fn main() {
    let input_file_sections = include_str!("../../../data/input.txt");

    let data = load(input_file_sections);

    let mut sum:i64= 0;
    for report in data{
        sum += estimate(report);
    }
    println!("Final sum: {sum}");

}