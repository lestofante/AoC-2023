struct Point{
    x: usize,
    y: usize,
}

impl Point{
    fn distance(&self, p: &Point) -> usize{
        let x = if self.x > p.x {self.x - p.x} else{p.x - self.x};
        let y = if self.y > p.y {self.y - p.y} else{p.y - self.y};
        x+y
    }
}

struct Map{
    cols:usize,
    rows:usize,
    data: Vec<Point>
}
impl Map{
    fn print(&self){
        let mut cursor = Point{x:0, y:0};
        for p in &self.data{
            while cursor.y < p.y{
                for _ in cursor.x..self.cols{
                    print!(".");
                    cursor.x += 1;
                }
                print!("\n");
                cursor.x = 0;
                cursor.y += 1;
            }
            while cursor.x < p.x{
                print!(".");
                cursor.x += 1;
            }
            print!("#");
            cursor.x += 1;
        }

        while cursor.y < self.rows{
            for _ in cursor.x..self.cols{
                print!(".");
                cursor.x += 1;
            }
            print!("\n");
            cursor.x = 0;
            cursor.y += 1;
        }
        println!("-------------------");
    }

    fn expand(&mut self){
        let mut empty_row = vec!(true; self.rows);
        let mut empty_cols = vec!(true; self.cols);

        for p in &self.data{
            empty_row[p.y] = false;
            empty_cols[p.x] = false;
        }

        let filler = 1000000-1;

        for (y_empty, r) in empty_row.iter().enumerate().rev(){
            if *r{
                println!("filling y {y_empty}");
                self.rows += filler;
                for p in &mut self.data{
                    if p.y > y_empty{
                        p.y += filler;
                    }
                }
            }
        }

        for (x_empty, c) in empty_cols.iter().enumerate().rev(){
            if *c{
                println!("filling x {x_empty}");
                self.cols += filler;
                for p in &mut self.data{
                    if p.x > x_empty{
                        p.x += filler;
                    }
                }
            }
        }
    }

    fn sum(&self){
        let mut sum = 0;
        for a in 0..self.data.len()-1{ //skip last point
            for b in a+1..self.data.len(){
                if a == b{
                    assert!(false, "should never happen");
                }
                let dist = self.data[a].distance(&self.data[b]);
                sum += dist;
                //println!("dist {a} {b} is {dist}");
            }
        }
        println!("sum {sum}");
    }

}

fn main() {
    let data = include_str!("../../../data/input.txt");

    let rows = data.lines().count();
    let cols = data.len() / rows;

    println!("cols {cols} rows {rows}");

    let data: Vec<Point> = data.lines().enumerate().flat_map(|(y, line)|{
        line.chars().enumerate().filter(|(_, c)| *c == '#').map(move |(x, _)| Point{x, y})
    }).collect();

    let mut data = Map{data, cols, rows};
    //data.print();

    data.expand();

    //data.print();

/*

    let data: Vec<Point> = data.iter().enumerate().flat_map(
        |(y, line)|{
        line.iter().enumerate().filter(|(_, val)| **val).map(move |(x, _)|Point{x, y})
        }
    ).collect();
*/
    data.sum();

}
