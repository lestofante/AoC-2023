use std::collections::{VecDeque, HashSet};

// Define a struct to represent cell coordinates
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y:isize) -> Point{
      Point{x, y}
    }

    fn is_valid(&self) -> bool{
      self.x >= 0 && self.y >= 0 &&
      self.x < MAP_SIZE && self.y < MAP_SIZE
    }

    fn len(&self, goal: &Point) -> isize {
        (self.x - goal.x).abs() + (self.y - goal.y).abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction{
  UP,
  DOWN,
  LEFT,
  RIGHT
}

#[derive(Debug, Clone)]
struct PointHistory {
  actual: Point,
  history: VecDeque<Direction>,
  full_history: HashSet<Point>,
  full_history_ordered: Vec<Point>,
}

fn print_partial(){

}

impl PointHistory {
  fn new(p: Point, history: VecDeque<Direction>, full_history: HashSet<Point>, full_history_ordered: Vec<Point>) -> PointHistory{
    PointHistory{actual:p, history, full_history, full_history_ordered}
  }

  fn successors(&self, grid:&Vec<Vec<u32>>) -> Vec<(PointHistory, u32)> {
    let mut successors: Vec<(PointHistory, u32)> = vec!();

    let mut print = false;
    // let desired: Vec<Point> = vec![
    //   //Point{x:10, y:2},
    //   //Point{x:10, y:3},
    //   //Point{x:10, y:4}
    //   //Point{x:10, y:4}
    // ];
    // if desired.contains(&self.actual){
    //   print = true;
    // }

    if print{
      println!("{:?} history {:?}", self.actual, self.history);
    }

    assert!(self.history.len() <= 3, "history is {}", self.history.len());

    let mut sum_direction:[usize; 4] = [0; 4];
    for v in &self.history{
      let index = v.clone();
      sum_direction[index as usize] += 1;
    }

    for i in 0..4{
      if sum_direction[i] >= 3{
        if print{
          println!("Invalid direction: {i}");
        }
      }
    }
    if print{
      println!("last direction is {:?}", self.history.back());
    }
    
    let new_history = {
      let history_len = self.history.len();
      if history_len < 3{
        self.history.clone()
      }else{
        let mut h = VecDeque::new();
        h.push_back(self.history.get(history_len - 2).expect("i checked for size before!").clone());
        h.push_back(self.history.get(history_len - 1).expect("i checked for size before!").clone());
        h
      }
    };
    
    //let new_full_history = HashSet::new();
    let mut new_full_history = self.full_history.clone();
    new_full_history.insert(self.actual);

    let new_full_history_ord = vec!();
    //let mut new_full_history_ord = self.full_history_ordered.clone();
    //new_full_history_ord.push(self.actual);

    if sum_direction[Direction::RIGHT as usize] < 3 && !self.history.back().is_some_and(|v| *v == Direction::LEFT){
      // travel +x ok
      let next = Point::new(self.actual.x + 1, self.actual.y);
      if next.is_valid() && !new_full_history.contains(&next) {
        if print{
          println!("going RIGHT");
        }
        let mut n = new_history.clone();
        n.push_back(Direction::RIGHT);
        successors.push((PointHistory::new(next, n, new_full_history.clone(), new_full_history_ord.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    if sum_direction[Direction::DOWN as usize] < 3 && !self.history.back().is_some_and(|v| *v == Direction::UP){
      // travel +y ok
      let next = Point::new(self.actual.x, self.actual.y + 1);
      if next.is_valid() && !new_full_history.contains(&next){
        if print{
          println!("going DOWN");
        }
        let mut n = new_history.clone();
        n.push_back(Direction::DOWN);
        successors.push((PointHistory::new(next, n, new_full_history.clone(), new_full_history_ord.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    if sum_direction[Direction::LEFT as usize] < 3 && !self.history.back().is_some_and(|v| *v == Direction::RIGHT){
      // travel -x ok
      let next = Point::new(self.actual.x - 1, self.actual.y);
      if next.is_valid() && !new_full_history.contains(&next){
        if print{
          println!("going LEFT");
        }
        let mut n = new_history.clone();
        n.push_back(Direction::LEFT);
        successors.push((PointHistory::new(next, n, new_full_history.clone(), new_full_history_ord.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    if sum_direction[Direction::UP as usize] < 3 && !self.history.back().is_some_and(|v| *v == Direction::DOWN){
      // travel -y ok
      let next = Point::new(self.actual.x, self.actual.y - 1);
      if next.is_valid() && !new_full_history.contains(&next){
        if print{
          println!("going UP");
        }
        let mut n = new_history.clone();
        n.push_back(Direction::UP);
        successors.push((PointHistory::new(next, n, new_full_history.clone(), new_full_history_ord.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    successors
  }

}

impl PartialEq for PointHistory {
  fn eq(&self, other: &Self) -> bool {
      self.actual == other.actual 
      && self.history == other.history
      //&& self.full_history_ordered == other.full_history_ordered
  }
}

impl Eq for PointHistory {

}

impl std::hash::Hash for PointHistory{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actual.hash(state);
        self.history.hash(state);
        //self.full_history_ordered.hash(state);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PointWeight{
  p: PointHistory,
  w: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PointVisited{
  p: Point,
  d: VecDeque<Direction>,
  w: u32,
}

fn bruteforce2(start: &PointHistory, grid: &Vec<Vec<u32>>, end: &Point) {
  let mut stack: VecDeque<PointWeight> = VecDeque::new();
  let mut visited: HashSet<PointVisited> = HashSet::new();

  let worst_cost = ((MAP_SIZE + MAP_SIZE) * 9) as u32;

  stack.push_back(PointWeight{p:start.clone(), w:0});

  let mut cheaper = worst_cost;
  let mut solution: Vec<PointWeight> = vec![];
  while let Some(current) = stack.pop_back() {
    //println!("at: {:?}", current.p.actual);


    if current.p.actual == *end {
      if current.w < cheaper{
        cheaper = current.w;
        solution.push(current.clone());
        
        let mut i = 0;
        let mut purged = 0;
        while i < stack.len(){
          if stack[i].w > cheaper{
            stack.remove(i);
            purged+=1;
          }else{
            i+=1;
          }
        }
        println!("Found path, cost: {} {:?} {:?} purged {purged}", current.w, current.p.full_history_ordered, current.p.actual);
      }
    }

    for path in current.p.successors(grid) {

      if path.0.actual.len(end) + path.1 as isize + current.w as isize > cheaper as isize{
        //println!("ignored");
        continue;
      }

      let mut print = false;
      // let desired: Vec<Point> = vec![
      //   Point{x:0, y:0},
      //   Point{x:1, y:0},
      //   Point{x:2, y:0},
      //   Point{x:2, y:1},
        
      //   Point{x:3, y:1},
        
      //   Point{x:4, y:1},
      //   Point{x:5, y:1},
        
      //   Point{x:5, y:0},
      //   Point{x:6, y:0},
      //   Point{x:7, y:0},
        
      //   Point{x:8, y:0},
      //   Point{x:8, y:1},
      //   Point{x:8, y:2},
      //   Point{x:9, y:2},
      //   Point{x:10, y:2},
      //   Point{x:10, y:3},
      //   Point{x:10, y:4},
      //   Point{x:11, y:4},
      //   Point{x:11, y:5},
      //   Point{x:11, y:6},
      //   Point{x:11, y:7},

      //   Point{x:12, y:7},
      //   Point{x:12, y:8},
      //   Point{x:12, y:9},
      //   Point{x:12, y:10},

      //   Point{x:11, y:10},
      //   Point{x:11, y:11},
      //   Point{x:11, y:12},

      // ];
      // if current.p.actual == (Point{x:12, y:12}) && current.p.full_history_ordered == desired{
      //   print = true;
      //   println!("\np1: {:?} {} {:?}", current.p.actual, current.w, current.p.history);
      //   let mut sum = 0;
      //   for i in &current.p.full_history_ordered{
      //     let cost = grid[i.y as usize][i.x as usize];
      //     sum += cost;
      //     println!("p2: {i:?} {cost} {sum}");
      //   }
      //   let cost = grid[current.p.actual.y as usize][current.p.actual.x as usize];
      //   sum += cost;
      //   println!("p3: {:?} {cost} {sum} {:?}", current.p.actual, current.p.history);
      //   //assert!(false);
      // }

      let cost = path.1 + current.w;
      if cost > cheaper{
        if print{
          println!("ignoring expansive path");
        }
        continue;
      }
      //let ris = PointWeight{p:path.0, w: path.1 + current.w};
      let ris = PointVisited{p: path.0.actual.clone(), d: path.0.history.clone(), w: path.1 + current.w};
      
      if !visited.contains(&ris) {
        if print{
          println!("visiting {:?}", ris);
        }
        stack.push_back(PointWeight{p:path.0, w: path.1 + current.w});
        //stack.push_back(ris.clone());
        visited.insert(ris);
      }else{
        if print{
          let get = visited.get(&ris);
          println!("visited already {:?} {:?}", ris, get);
        }
      }
    }
  }

  for current in solution{
    println!("Found path, cost: {} {:?} {:?}", current.w, current.p.full_history_ordered, current.p.actual);
    println!();
  }
  println!("No valid path found");
}



const DATA: &str = include_str!("../../../data/input.txt");
const MAP_SIZE:isize = 141;

fn main() {
    // Define your grid with movement costs

    let grid:Vec<Vec<u32>> = vec![
      vec![1, 3, 2, 4],
      vec![2, 5, 3, 2],
      vec![3, 2, 5, 1],
      vec![4, 3, 2, 1],
    ];


    let grid:Vec<Vec<u32>> = DATA.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    assert_eq!(grid[0].len(), MAP_SIZE as usize);

    for y in &grid{
      for x in y{
        print!("{x}");
      }
      println!();
    }
    
    let start = PointHistory::new(Point{ x: 0, y: 0 }, vec![].into(), HashSet::new(), vec!());

    bruteforce2(&start, &grid, &Point{ x: MAP_SIZE-1, y: MAP_SIZE-1 });

}
