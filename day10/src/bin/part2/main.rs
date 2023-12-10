use core::fmt;

#[derive(Debug)]
struct Point{
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x as isize, y: y as isize }
    }
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
                    (Direction::N, Direction::N) | 
                    (Direction::S, Direction::S) | 
                    (Direction::E, Direction::E) |
                    (Direction::W, Direction::W) => {assert!(false, "cant happen");write!(f, "X")},

                    (Direction::N, Direction::E) |
                    (Direction::E, Direction::N) => write!(f, "╝"),

                    (Direction::N, Direction::W) |
                    (Direction::W, Direction::N) => write!(f, "╚"),

                    (Direction::N, Direction::S) |
                    (Direction::S, Direction::N) => write!(f, "║"),

                    (Direction::E, Direction::W) |
                    (Direction::W, Direction::E) => write!(f, "═"),

                    (Direction::E, Direction::S) |
                    (Direction::S, Direction::E) => write!(f, "╗"),

                    (Direction::W, Direction::S) |
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
    &map[posizione.y as usize][posizione.x as usize]
}

fn formica(entrata: Direction, mut posizione: Point, map: &Vec<Vec<Tube>>) -> Option<(Direction, Point)>{
    //println!("I am at {} // {} {} coming from {entrata:?}", get(map, &posizione), posizione.x, posizione.y);
    match get(map, &posizione){
        Tube::Pipe (dir1, dir2) => 
            {
                if *dir1 == entrata{
                    step(&mut posizione, dir2);
                    //println!("going {:?} {posizione:?}", dir2);
                    Some((dir2.inverse(), posizione))
                }else{
                    step(&mut posizione, dir1);
                    //println!("going {:?} {posizione:?}", dir1);
                    Some((dir1.inverse(), posizione))
                }
                
            }
        Tube::Start => None,
        Tube::NC => {assert!(false, "should never be here"); None}
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Col{
    L,
    R,
    N,
    VISITED
}

fn set(map: &mut Vec<Vec<Col>>, p: &Point, col: Col){
    if p.x < 0 || p.x >= map[0].len() as isize{
        return;
    }
    if p.y < 0 || p.y >= map.len() as isize{
        return;
    }
    let cell = &mut map[p.y as usize][p.x as usize];
    if *cell != Col::VISITED{
        *cell = col;
    }
}

fn flood_tile(map: &mut Vec<Vec<Col>>, p: Point, c: Col){
    for y in p.y-1..=p.y+1{
        if y < 0 || y >= map.len() as isize{
            continue;
        }
        for x in p.x-1..=p.x+1{
            if x < 0 || x >= map[y as usize].len() as isize{
                continue;
            }
            if map[y as usize][x as usize] == Col::N {
                //look around
                map[y as usize][x as usize] = c;
                flood_tile(map, Point{x, y}, c);
            } 
        }
    }
}

fn flood(mut map_color: &mut Vec<Vec<Col>>){
    let mut y = 0;
    while y < map_color.len(){
        let mut x = 0;
        while x < map_color[y].len(){
            if map_color[x][y] == Col::L {
                //look around
                flood_tile(&mut map_color, Point::new(x, y), Col::L);
            }
            if map_color[x][y] == Col::R {
                //look around
                flood_tile(&mut map_color, Point::new(x, y), Col::R);
            }
            x+=1;
        }
        y+=1;
    }
}

fn run(mut d: Direction, mut p: Point, map: &Vec<Vec<Tube>>){
    let mut map_color : Vec<Vec<Col>> = vec![vec![Col::N; map[0].len()]; map.len()];

    let mut count = 0;

    while let Some(tmp) = formica(d, p, &map){
        count += 1;
        p = tmp.1;
        d = tmp.0;
        set(&mut map_color, &p, Col::VISITED);
        match get(map, &p){
            Tube::Pipe(a, b) => match (a, b){
                (Direction::N, Direction::S) | (Direction::S, Direction::N) => 
                if d == Direction::S{
                    set(&mut map_color, &Point{y: p.y,x:p.x-1}, Col::L);
                    set(&mut map_color, &Point{y: p.y,x:p.x+1}, Col::R);
                }else{
                    set(&mut map_color,&Point{y: p.y,x:p.x-1}, Col::R);
                    set(&mut map_color,&Point{y: p.y,x:p.x+1}, Col::L);
                },

                (Direction::E, Direction::W) | (Direction::W, Direction::E) => 
                if d == Direction::E{
                    set(&mut map_color,&Point{y: p.y-1,x:p.x}, Col::L);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x}, Col::R);
                }else{
                    set(&mut map_color,&Point{y: p.y-1,x:p.x}, Col::R);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x}, Col::L);
                },

                (Direction::N, Direction::E) | (Direction::E, Direction::N) =>
                if d == Direction::E{
                    set(&mut map_color,&Point{y: p.y-1,x:p.x-1}, Col::L);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x+1}, Col::R);
                    set(&mut map_color,&Point{y: p.y,x:p.x+1}, Col::R);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x}, Col::R);
                }else{
                    set(&mut map_color,&Point{y: p.y-1,x:p.x-1}, Col::R);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x+1}, Col::L);
                    set(&mut map_color,&Point{y: p.y,x:p.x+1}, Col::L);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x}, Col::L);
                },

                (Direction::N, Direction::W) | (Direction::W, Direction::N) => 
                if d == Direction::N{
                    set(&mut map_color,&Point{y: p.y-1,x:p.x+1}, Col::L);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x-1}, Col::R);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x}, Col::R);
                    set(&mut map_color,&Point{y: p.y, x:p.x-1}, Col::R);
                }else{
                    set(&mut map_color,&Point{y: p.y-1,x:p.x+1}, Col::R);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x-1}, Col::L);
                    set(&mut map_color,&Point{y: p.y+1,x:p.x}, Col::L);
                    set(&mut map_color,&Point{y: p.y,x:p.x-1}, Col::L);
                },

                (Direction::E, Direction::S) | (Direction::S, Direction::E) => 
                if d == Direction::S{
                    set(&mut map_color,&Point{y: p.y+1,x:p.x-1}, Col::L);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x+1}, Col::R);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x}, Col::R);
                    set(&mut map_color,&Point{y: p.y, x:p.x+1}, Col::R);
                }else{
                    set(&mut map_color,&Point{y: p.y+1,x:p.x-1}, Col::R);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x+1}, Col::L);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x}, Col::L);
                    set(&mut map_color,&Point{y: p.y, x:p.x+1}, Col::L);
                },

                (Direction::W, Direction::S) | (Direction::S, Direction::W) => 
                if d == Direction::W{
                    set(&mut map_color,&Point{y: p.y+1,x:p.x+1}, Col::L);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x-1}, Col::R);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x}, Col::R);
                    set(&mut map_color,&Point{y: p.y, x:p.x-1}, Col::R);
                }else{
                    set(&mut map_color,&Point{y: p.y+1,x:p.x+1}, Col::R);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x-1}, Col::L);
                    set(&mut map_color,&Point{y: p.y-1,x:p.x}, Col::L);
                    set(&mut map_color,&Point{y: p.y, x:p.x-1}, Col::L);
                },
                
                _ => {},
            },
            Tube::Start => {},
            Tube::NC => {},
        }
    }
    println!("count {count} {}", (count+1)/2);

    flood(&mut map_color);

    let mut count_g= 0;
    let mut count_r= 0;

    for (y, r) in map.iter().enumerate(){
        for (x, e) in r.iter().enumerate(){
            match map_color[y][x] {
                Col::L => {
                    print!("\x1b[31m{e}\x1b[0m"); //red
                    count_r += 1;
                },
                Col::R => {
                    print!("\x1b[32m{e}\x1b[0m"); //green
                    count_g += 1;
                },
                Col::N => print!("\x1b[34m{e}\x1b[0m"), // blue
                Col::VISITED => print!("\x1b[33m{e}\x1b[0m"), //yellow
            }
        }
        print!("\n");
    }

    let mut color_solution = Col::N;

    for r in &map_color[0]{
        if r == &Col::L || r == &Col::L{
            color_solution = Col::R;
            break;
        }
        if r == &Col::R || r == &Col::R{
            color_solution = Col::L;
            break;
        }
    }
    if color_solution == Col::N{
        for r in map_color{
            if r.first() == Some(&Col::L) || r.last() == Some(&Col::L){
                color_solution = Col::R;
                break;
            }
            if r.first() == Some(&Col::R) || r.last() == Some(&Col::R){
                color_solution = Col::L;
                break;
            }
        }
    }

    println!("Total R: {count_g}\nTotal L :{count_r}\nSolution is letter {color_solution:?}");
}

fn main() {
    const INPUT_FILE: &str = include_str!("../../../data/input.txt");
    
    let rows_len = INPUT_FILE.len() as isize;

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
            //print!("{e}");
            if *e == Tube::Start{
                p = Some(Point::new(x, y));
            }
        }
        //print!("\n");
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
    if p.y < map.len() as isize-1{
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
