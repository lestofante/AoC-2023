struct Point{
    x: usize,
    y: usize,
}

impl Point{
    fn distance(&self, p: &Point) -> usize{
        ((self.x as isize - p.x as isize).abs() + (self.y as isize - p.y as isize).abs()) as usize
    }
}

fn expand_horizontal(data: &mut Vec<Vec<bool>>){
    //add horizontal expansion
    let mut add_at: Vec<usize> = vec!();
    for y in 1..data.len() {
        println!("parsing {}", y-1);
        let mut all_void = true;
        for c in &data[y - 1] {
            if *c{
                all_void = false;
            }
        }
        if all_void{
            println!("add line");
            add_at.push(y);
        }
    }

    for (n, y) in add_at.iter().enumerate(){
        data.insert(y+n, vec![false; data[0].len()]);
    }
}

fn traspose(data: Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut transposed_matrix = vec![vec![false; data.len()]; data[0].len()];
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            transposed_matrix[j][i] = data[i][j];
        }
    }
    transposed_matrix 
}

fn print(data: &Vec<Vec<bool>>){
    for lines in data{
        for v in lines{
            if *v{
                print!("X");
            }else{
                print!(".");
            }
        }
        println!();
    }
    println!("-------------------");
}

fn main() {
    let data = include_str!("../../../data/input.txt");

    let mut data: Vec<Vec<bool>> = data.lines().enumerate().map(|(y, line)|{
        line.chars().enumerate().map(move |(x, c)| c == '#').collect()
    }).collect();

    //print(&data);

    expand_horizontal(&mut data);
    //print(&data);
    
    let mut data = traspose(data);

    expand_horizontal(&mut data);

    let data = traspose(data);
    //print(&data);
    

    let data: Vec<Point> = data.iter().enumerate().flat_map(
        |(y, line)|{
        line.iter().enumerate().filter(|(_, val)| **val).map(move |(x, _)|Point{x, y})
        }
    ).collect();

    let mut sum = 0;
    for a in 0..data.len()-1{ //skip last point
        for b in a+1..data.len(){
            let dist = data[a].distance(&data[b]);
            /*
            if dist < min_dist{
                min_dist = dist;
                min_dist_id = b;
            }
            */
            sum += dist;
            //println!("dist {a} {b} is {dist}");
        }
    }

    println!("sum {sum}");
}
