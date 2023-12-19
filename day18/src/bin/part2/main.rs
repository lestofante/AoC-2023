#[derive(Debug, Clone, Copy)]
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

const DATA: &str = include_str!("../../../data/input.txt");

fn main() {
    let mut sumperimeter: [usize; 4] = [0; 4];
    let data: Vec<Corner> = DATA.lines().map(|p| {
        let mut str = p.split("#").skip(1).next().expect("must be there").chars();

        let mut sum: usize = 0;
        for _ in 0..5{
            let d = str.next().expect("must be there");
            sum *= 16;

            sum += d.to_digit(16).expect("must be hex") as usize;
        }

        let dir = match str.next().expect("must have data") {
            '3' => Direction::UP,
            '1' => Direction::DOWN,
            '2' => Direction::LEFT,
            '0' => Direction::RIGHT,
            _ => {
                assert!(false);
                Direction::RIGHT
            },
        };

        sumperimeter[dir.clone() as usize] += sum;
        
        assert_eq!(str.next().expect("should have a )"), ')');
        Corner{
            dir,
            len: sum 
        }
    }).collect();

    for i in sumperimeter{
        println!("p {i}");
    }

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

    let area = calculate_polygon_area(&ris);
    println!("area is {area}");

    for i in sumperimeter{
        println!("p {}", i + area as usize);
    }
    println!("pp {}", sumperimeter[0] + sumperimeter[2] + area as usize + 1);
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