enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

struct Corner{
    dir: Direction,
    len: usize,
}

#[derive(Clone, Debug)]
struct Point{
    x: isize,
    y: isize,
}

fn calculate_polygon_area(points: &[Point]) -> isize {
    let mut sum: isize = 0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += (points[j].x + points[i].x + 1) * (points[j].y - points[i].y);
    }

    sum.abs() / 2
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Type{
    VOID,
    PERIMETER,
    FILL
}

fn fill(perimeter: &mut [[Type;SIZE]; SIZE], start: &Point ){
    let x = start.x as usize;
    let y = start.y as usize;
    println!("attempting to fill {:?}", start);
    if perimeter[y][x] == Type::VOID{
        perimeter[y][x] = Type::FILL;
    }else{
        return;
    }
    fill(perimeter, &Point{x:start.x-1, y:start.y});
    fill(perimeter, &Point{x:start.x+1, y:start.y});
    fill(perimeter, &Point{x:start.x, y:start.y-1});
    fill(perimeter, &Point{x:start.x, y:start.y+1});
}

const DATA: &str = include_str!("../../../data/input.txt");
const SIZE: usize = 500;

fn main() {
    let data: Vec<Corner> = DATA.lines().map(|p| {
        let mut str = p.chars();
        let dir = match str.next().expect("must have data") {
            'U' => Direction::UP,
            'D' => Direction::DOWN,
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => {
                assert!(false);
                Direction::RIGHT
            },
        };

        assert_eq!(str.next().expect("hopuld have a space"), ' ');

        let mut sum: usize = 0;
        while let Some(d) = str.next(){
            if d == ' '{
                break;
            }
            sum *= 10;
            sum += (d as u8 - b'0') as usize;
        }

        Corner{
            dir,
            len: sum 
        }
    }).collect();

    let mut minx = isize::MAX;
    let mut miny = isize::MAX;
    let mut maxx = 0;
    let mut maxy = 0;
    let mut actual = Point{x:1, y:1};
    let ris: Vec<Point> = data.iter().map(|p| {
        match p.dir {
            Direction::UP => actual.y -= p.len as isize,
            Direction::DOWN => actual.y += p.len as isize,
            Direction::LEFT => actual.x -= p.len as isize,
            Direction::RIGHT => actual.x += p.len as isize,
        }
        if actual.x < minx{
            minx = actual.x;
        }
        if actual.y < miny{
            miny = actual.y;
        }
        if actual.x > maxx{
            maxx = actual.x;
        }
        if actual.y > maxy{
            maxy = actual.y;
        }
        actual.clone()
    }).collect();
    //ris.push(Point{x:0, y:0});

    //println!("{minx} {maxx} {miny} {maxy} {:?}", ris);
    //assert!(false);

    let mut perimeter: [[Type;SIZE]; SIZE] = [[Type::VOID;SIZE]; SIZE];
    let mut actual = Point{x: 170, y: 300};
    for p in data{
        match p.dir {
            Direction::UP => {
                for _ in 0..p.len{
                    actual.y -= 1;
                    perimeter[actual.y as usize][actual.x as usize] = Type::PERIMETER;
                }
            },
            Direction::DOWN => {
                for _ in 0..p.len{
                    actual.y += 1;
                    perimeter[actual.y as usize][actual.x as usize] = Type::PERIMETER;
                }
            },
            Direction::LEFT => {
                for _ in 0..p.len{
                    actual.x -= 1;
                    perimeter[actual.y as usize][actual.x as usize] = Type::PERIMETER;
                }
            },
            Direction::RIGHT => {
                for _ in 0..p.len{
                    actual.x += 1;
                    perimeter[actual.y as usize][actual.x as usize] = Type::PERIMETER;
                }
            },
        }
    } 

    for y in &perimeter{
        for x in y{
            match *x{
                Type::VOID => print!("."),
                Type::PERIMETER => print!("X"),
                Type::FILL => print!("#"),
            }
        }
        println!();
    }
/*
    let mut count = 0;
    for y in &mut perimeter{
        let mut inside = false;
        let mut possible_change = false;
        let mut ignore = false;
        for x in y{
            if *x == Type::PERIMETER{
                if ignore || possible_change{
                    ignore = true;
                    possible_change = false;
                }else{
                    possible_change = true;
                }
                count += 1;
            }else{
                ignore = false;
                if possible_change{
                    possible_change = false;
                    inside = !inside;
                }
                if inside{
                    count += 1;
                    *x = Type::FILL;
                }
            }
        }
        println!();
    }
    */
    fill(&mut perimeter, &Point{x:171,y:301});

    for y in &perimeter{
        for x in y{
            match *x{
                Type::VOID => print!("."),
                Type::PERIMETER => print!("X"),
                Type::FILL => print!("#"),
            }
        }
        println!();
    }

    for p in &ris{
        print!("({},{}) ", p.x, p.y);
    }

    let mut sum = 0;
    for y in &perimeter{
        for x in y{
            if *x != Type::VOID{
                sum += 1;
            }
        }
    }
    println!("sum is {sum}");

}


#[test]
fn a(){
    let ris = vec![
        Point{x: 0, y: 0},
        Point{x: 0, y: 3},
        Point{x: 3, y: 3},
        Point{x: 3, y: 0},
    ];

    for p in &ris{
        print!("({},{}) ", p.x, p.y);
    }
    let area = calculate_polygon_area(&ris); 
    println!("area {area}")
}

#[test]
fn b(){
    let ris = vec![
        Point{x: 0, y: 0},
        Point{x: 0, y: 1},
        Point{x: 1, y: 1},
        Point{x: 1, y: 0},
    ];

    for p in &ris{
        print!("({},{}) ", p.x, p.y);
    }
    let area = calculate_polygon_area(&ris); 
    println!("area {area}")
}