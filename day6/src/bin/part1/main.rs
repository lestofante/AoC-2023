#[derive(Debug, Clone, Copy)]
struct Data{
    time: usize,
    distance: usize,
}

const SIZE:usize = 4;

fn load(section: &str) -> [Data; SIZE] {
    let mut lines = section.lines();

    let mut data: [Data; SIZE] = [
        Data{
            time : 0,
            distance: 0,
        }; 
        SIZE
    ];

    let mut values = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap());
    for i in 0..SIZE{
        data[i].time = values.next().unwrap();
    }
    let mut values = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse::<usize>().unwrap());
    for i in 0..SIZE{
        data[i].distance = values.next().unwrap();
    }

    data
}

fn main() {
    let input_file_sections = include_str!("../../../data/input.txt");

    let data = load(input_file_sections);
    println!("data {data:#?}");

    let mut mult: usize = 1;
    for set in &data{
        let sqrt = ((set.time*set.time - 4 * set.distance) as f64).sqrt();
        let xp = (set.time as f64 + sqrt) / 2.0;
        let xm = (set.time as f64 - sqrt) / 2.0;
        println!("\n0 xm {xm} xp {xp}");
        if xp < xm{
            let xc = xp as usize;
            let xt = xm.ceil() as usize;
            let sum = xt - xc - 1;
            mult *= sum;
            println!("1 xc {xc:#?} xt {xt} sum {sum}");
        }else{
            let xc = xm as usize;
            let xt = xp.ceil() as usize;
            let sum = xt - xc - 1;
            mult *= sum;
            println!("2 xc {xc:#?} xt {xt} sum {sum}");
        }
        
    }
    
    println!("mult {mult}");
}
