use core::fmt;

#[derive(Debug)]
struct Point{
    x: usize,
    y: usize,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction{
    N,
    E,
    W,
    S,
}

impl Direction{
    fn inverse(&self) -> Direction{
        match *self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
        }
    }
}

#[derive(PartialEq)]
enum Tube{
    Pipe(Direction, Direction), // |
    Start,
    NC, //not connect
}

impl Tube{
    fn contains(&self, dir: Direction) -> bool{
        match *self{
            Tube::Pipe(a, b) => a == dir || b == dir,
            Tube::Start => false,
            Tube::NC => false,
        }
    }
}

impl fmt::Display for Tube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Tube::Pipe(a, b) => 
                match (a,b) {
                    (Direction::N, Direction::N) => write!(f, "X"),
                    (Direction::S, Direction::S) => write!(f, "X"),
                    (Direction::E, Direction::E) => write!(f, "X"),
                    (Direction::W, Direction::W) => write!(f, "X"),

                    (Direction::N, Direction::E) => write!(f, "╝"),
                    (Direction::E, Direction::N) => write!(f, "╝"),

                    (Direction::N, Direction::W) => write!(f, "╚"),
                    (Direction::W, Direction::N) => write!(f, "╚"),

                    (Direction::N, Direction::S) => write!(f, "║"),
                    (Direction::S, Direction::N) => write!(f, "║"),

                    (Direction::E, Direction::W) => write!(f, "═"),
                    (Direction::W, Direction::E) => write!(f, "═"),

                    (Direction::E, Direction::S) => write!(f, "╗"),
                    (Direction::S, Direction::E) => write!(f, "╗"),

                    (Direction::W, Direction::S) => write!(f, "╔"),
                    (Direction::S, Direction::W) => write!(f, "╔"),
                },
            Tube::Start => write!(f, "S"),
            Tube::NC => write!(f, "."),
        }
    }
}

fn step(posizione: &mut Point, step: &Direction){
    // 0,0 is NE
    match step {
        Direction::N => {posizione.y -= 1},
        Direction::E => {posizione.x -= 1},
        Direction::W => {posizione.x += 1},
        Direction::S => {posizione.y += 1},
    }
}

fn get<'a>(map: &'a Vec<Vec<Tube>>, posizione: &Point) -> &'a Tube{
    map.get(posizione.y).unwrap().get(posizione.x).unwrap()
}

fn formica(entrata: Direction, mut posizione: Point, map: &Vec<Vec<Tube>>) -> Option<(Direction, Point)>{
    println!("I am at {} // {} {} coming from {entrata:?}", get(map, &posizione), posizione.x, posizione.y);
    match get(map, &posizione){
        Tube::Pipe (dir1, dir2) => 
            {
                if *dir1 == entrata{
                    step(&mut posizione, dir2);
                    println!("going {:?} {posizione:?}", dir2);
                    Some((dir2.inverse(), posizione))
                }else{
                    step(&mut posizione, dir1);
                    println!("going {:?} {posizione:?}", dir1);
                    Some((dir1.inverse(), posizione))
                }
                
            }
        Tube::Start => None,
        Tube::NC => {assert!(false, "should never be here"); None}
    }
}

fn run(mut d: Direction, mut p: Point, map: &Vec<Vec<Tube>>){
    let mut map_visited : Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut count = 0;
    map_visited[p.y][p.x] = true;
    while let Some(tmp) = formica(d, p, &map){
        count += 1;
        p = tmp.1;
        d = tmp.0;
        map_visited[p.y][p.x] = true;
    }
    println!("count {count} {}", (count+1)/2);

    for (y, r) in map.iter().enumerate(){
        for (x, e) in r.iter().enumerate(){
            if map_visited[y][x]{
                print!("\x1b[31m{e}\x1b[0m");
            }else{
                print!("\x1b[34m{e}\x1b[0m");
            }
        }
        print!("\n");
    }
}
fn main() {
    const INPUT_FILE: &str = include_str!("../../../data/input.txt");
    
    let rows_len = INPUT_FILE.len();

    let map: Vec<Vec<Tube>> = INPUT_FILE.lines().map(|row| row.chars().map(|byte|
        match byte {
            '|' => Tube::Pipe(Direction::N, Direction::S),
            '-' => Tube::Pipe(Direction::E, Direction::W),
            '7' => Tube::Pipe(Direction::E, Direction::S),
            'F' => Tube::Pipe(Direction::W, Direction::S),
            'J' => Tube::Pipe(Direction::N, Direction::E),
            'L' => Tube::Pipe(Direction::N, Direction::W),
            'S' => Tube::Start,
            '.' => Tube::NC,
            _ => {assert!(false, "should nevber happen"); Tube::NC},
        }
    ).collect()).collect();

    println!("MAP EXTRACTED, size: {}, {}", map.len(), map[0].len());

    let mut p: Option<Point> = None;
    for (y, r) in map.iter().enumerate(){
        for (x, e) in r.iter().enumerate(){
            print!("{e}");
            if *e == Tube::Start{
                p = Some(Point{x, y});
            }
        }
        print!("\n");
    }

    let p = p.expect("cant find entrance!");

    println!("Entrance: {p:?}");
    
    if p.y > 0{
        let p = Point{x:p.x, y:p.y-1};
        let cell_top = get(&map, &p);
        if cell_top.contains(Direction::S) {
            run(Direction::S, p, &map);
            return;
        }
    }
    if p.y < map.len()-1{
        let p = Point{x:p.x, y:p.y+1};
        let cell_top = get(&map, &p);
        if cell_top.contains(Direction::N) {
            run(Direction::N, p, &map);
            return;
        }
    }

    if p.x > 0{
        let p = Point{x:p.x-1, y:p.y};
        let cell_top = get(&map, &p);
        if cell_top.contains(Direction::W) {
            run(Direction::W, p, &map);
            return;
        }
    }

    if p.x < rows_len-1{
        let p = Point{x:p.x+1, y:p.y};
        let cell_top = get(&map, &p);
        if cell_top.contains(Direction::E) {
            run(Direction::E, p, &map);
            return;
        }
    }
    /*
    let mut d = map.get(p.y).unwrap().get(p.x).unwrap().

    
    */

}
