struct Map{
    cols: usize,
    data: Vec<Tile>,
}

#[derive(Debug)]
enum Direction{
    Up,
    Down,
}

#[derive(Debug)]
enum Tile{
    Empty,
    Mirror(Direction),
    Split(Direction),
}

#[derive(Debug)]
struct Point{
    x:isize,
    y:isize,
}

impl Map{

    fn new(original: &[u8]) -> Map{
        let ris: Vec<Tile> = original.iter().filter(|c| **c != b'\n').map(|c|
            match *c{
                b'.' => Tile::Empty,
                b'/' => Tile::Mirror(Direction::Up),
                b'\\' => Tile::Mirror(Direction::Down),
                b'-' => Tile::Split(Direction::Up),
                b'|' => Tile::Split(Direction::Down),
                _ => {assert!(false); Tile::Empty},
            }
        ).collect();
        let cols = (ris.len() as f64).sqrt() as usize;
        println!("map is {cols} column");
        return Map{
            data: ris,
            cols
        };
    }


    fn get(&self, p: &Point) -> Option<&Tile>{
        if let Some(i) = self.get_index(p){
            Some(&self.data[i])
        }else{
            None
        }
    }

    fn get_index(&self, p: &Point) -> Option<usize>{
        if p.x >= 0 && p.y >= 0{
            let x = p.x as usize;
            let y = p.y as usize;
            if x < self.cols && y < self.cols{
                return Some(y*(self.cols)+x);
            }
        }
        None
    }

    fn print(&self){
        for (n, v) in self.data.iter().enumerate(){
            if n % self.cols == 0{
                println!();
            }
            match v {
                Tile::Empty => print!("."),
                Tile::Mirror(dir) => match dir{
                    Direction::Up => print!("/"),
                    Direction::Down => print!("\\"),
                }
                Tile::Split(dir) => match dir{
                    Direction::Up => print!("-"),
                    Direction::Down => print!("|"),
                },
            }
            
        }
        println!();
        println!("~~~~~~~~~~~~~~~~~~~~~");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LightPath{
    LR,
    RL,
    NS,
    SN,
}

#[derive(Debug)]
struct Versor{
    dir: LightPath,
    p: Point,
}

struct MapVisited{
    cols: usize,
    data: Vec<Vec<LightPath>>,
}

enum Visited{
    YesSameDirection,
    Yes,
    No,
}

impl MapVisited {

    fn new(cols: usize)-> MapVisited{
        let mut v: Vec<Vec<LightPath>> = vec!();
        for _ in 0..cols*cols{
            v.push(vec!());
        }
        MapVisited{
            cols:cols,
            data: v,
        }
    }

    fn visit(&mut self, ver: &Versor) -> Visited{
        if ver.p.x >= 0 && ver.p.y >= 0{
            let x = ver.p.x as usize;
            let y = ver.p.y as usize;
            let i = y * self.cols + x;
            if i < self.data.len(){
                if self.data[i].contains(&ver.dir){
                    return Visited::YesSameDirection;
                }
                self.data[i].push(ver.dir);
                if self.data[i].len() > 1{
                    return Visited::Yes;
                }
            }
        }
        return Visited::No;
    }
}

fn step(ver: Versor, tile: &Tile) -> Vec<Versor>{
    match tile {
        Tile::Empty => match ver.dir {
            LightPath::LR => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x+1, y:ver.p.y}}],
            LightPath::RL => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x-1, y:ver.p.y}}],
            LightPath::NS => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x, y:ver.p.y+1}}],
            LightPath::SN => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x, y:ver.p.y-1}}],
        },
        Tile::Mirror(surface) => match surface {
            Direction::Up => match ver.dir { // /
                LightPath::LR => vec![Versor{dir: LightPath::SN, p: Point{x:ver.p.x, y:ver.p.y-1}}],
                LightPath::RL => vec![Versor{dir: LightPath::NS, p: Point{x:ver.p.x, y:ver.p.y+1}}],
                LightPath::NS => vec![Versor{dir: LightPath::RL, p: Point{x:ver.p.x-1, y:ver.p.y}}],
                LightPath::SN => vec![Versor{dir: LightPath::LR, p: Point{x:ver.p.x+1, y:ver.p.y}}],
            }
            Direction::Down => match ver.dir { // \
                LightPath::LR => vec![Versor{dir: LightPath::NS, p: Point{x:ver.p.x, y:ver.p.y+1}}],
                LightPath::RL => vec![Versor{dir: LightPath::SN, p: Point{x:ver.p.x, y:ver.p.y-1}}],
                LightPath::NS => vec![Versor{dir: LightPath::LR, p: Point{x:ver.p.x+1, y:ver.p.y}}],
                LightPath::SN => vec![Versor{dir: LightPath::RL, p: Point{x:ver.p.x-1, y:ver.p.y}}],
            }
        },
        Tile::Split(surface) => match surface {
            Direction::Up => match ver.dir { // -
                LightPath::LR => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x+1, y:ver.p.y}}],
                LightPath::RL => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x-1, y:ver.p.y}}],
                LightPath::NS | LightPath::SN => vec![
                    Versor{dir: LightPath::LR, p: Point{x:ver.p.x+1, y:ver.p.y}},
                    Versor{dir: LightPath::RL, p: Point{x:ver.p.x-1, y:ver.p.y}}
                    ],
            },
            Direction::Down => match ver.dir { // |
                LightPath::LR | LightPath::RL=> vec![
                    Versor{dir: LightPath::NS, p: Point{x:ver.p.x, y:ver.p.y+1}},
                    Versor{dir: LightPath::SN, p: Point{x:ver.p.x, y:ver.p.y-1}},
                    ],
                LightPath::NS => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x, y:ver.p.y+1}}],
                LightPath::SN => vec![Versor{dir: ver.dir, p: Point{x:ver.p.x, y:ver.p.y-1}}],
            }
        },
    }
}

fn find_next_cell(ver: Versor, map: &Map, map_visit: &mut MapVisited, count: usize) -> usize{
    if let Some(tile) = map.get(&ver.p){
        /*
        for _ in 0..count{
            print!(" ");
        }
        println!("at {ver:?} found {tile:?}");
        */

        let visited = 
        match map_visit.visit(&ver) {
            Visited::YesSameDirection => return 0,
            Visited::Yes => true,
            Visited::No => false,
        };

        let ris = step(ver, tile);
        let mut sum = 0;
        if !visited && ris.len() > 0{
            sum += 1;
        }
        for next in ris{
            sum += find_next_cell(next, map, map_visit, count+1);
        }
        return sum;
    }else{
        //println!("at {ver:?} DEAD END");
        return 0;
    }
}


const DATA: &[u8] = include_bytes!("../../../data/input.txt");

fn main() {
    
    let data = Map::new(DATA);

    data.print();

    let mut max = 0;

    for x in 0..data.cols{
        let start = Versor{
            dir: LightPath::NS,
            p: Point{x:x as isize, y:0}
        };
        let mut data_visit = MapVisited::new(data.cols);
        let sum = find_next_cell(start, &data, &mut data_visit, 0);
        println!("sum {x},0 {sum}");
        if sum > max{
            max = sum;
        }

        let start = Versor{
            dir: LightPath::SN,
            p: Point{x:x as isize, y:data.cols as isize -1}
        };
        let mut data_visit = MapVisited::new(data.cols);
        let sum = find_next_cell(start, &data, &mut data_visit, 0);
        println!("sum {x},{} {sum}", data.cols-1);
        if sum > max{
            max = sum;
        }
    }

    for y in 0..data.cols{
        let start = Versor{
            dir: LightPath::LR,
            p: Point{x:0 as isize, y:y as isize}
        };
        let mut data_visit = MapVisited::new(data.cols);
        let sum = find_next_cell(start, &data, &mut data_visit, 0);
        println!("sum 0,{y} {sum}");
        if sum > max{
            max = sum;
        }

        let start = Versor{
            dir: LightPath::RL,
            p: Point{x:data.cols as isize -1 as isize, y:y as isize}
        };
        let mut data_visit = MapVisited::new(data.cols);
        let sum = find_next_cell(start, &data, &mut data_visit, 0);
        println!("sum {},{y} {sum}", data.cols-1);
        if sum > max{
            max = sum;
        }
    }
    println!("max is {max}");
}
