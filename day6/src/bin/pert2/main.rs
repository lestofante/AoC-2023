#[derive(Debug, Clone, Copy)]
struct Data{
    time: usize,
    distance: usize,
}

fn load(section: &str) -> Data {
    let mut lines = section.lines();

    let mut data: Data = Data{
        time : 0,
        distance: 0,
    };

    data.time = lines.next().unwrap().split(':').skip(1).map(|s| s.replace(" ", "").parse::<usize>().unwrap()).next().unwrap();
    
    data.distance = lines.next().unwrap().split(':').skip(1).map(|s| s.replace(" ", "").parse::<usize>().unwrap()).next().unwrap();

    data
}

fn main() {
    let input_file_sections = include_str!("../../../data/input.txt");

    let data = load(input_file_sections);
    println!("data {data:#?}");
    
    let sqrt = ((data.time*data.time - 4 * data.distance) as f64).sqrt();
    let xp = (data.time as f64 + sqrt) / 2.0;
    let xm = (data.time as f64 - sqrt) / 2.0;
    println!("\n0 xm {xm} xp {xp}");
    if xp < xm{
        let xc = xp as usize;
        let xt = xm.ceil() as usize;
        let sum = xt - xc - 1;
        println!("1 xc {xc:#?} xt {xt} sum {sum}");
    }else{
        let xc = xm as usize;
        let xt = xp.ceil() as usize;
        let sum = xt - xc - 1;
        println!("2 xc {xc:#?} xt {xt} sum {sum}");
    }
}
