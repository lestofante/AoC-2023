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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction{
  UP,
  DOWN,
  LEFT,
  RIGHT
}
impl Direction {
  fn inverse(&self) -> Direction {
    match self {
      Direction::UP => Direction::DOWN,
      Direction::DOWN => Direction::UP,
      Direction::LEFT => Direction::RIGHT,
      Direction::RIGHT => Direction::LEFT,
    }
  }
}

#[derive(Debug, Clone)]
struct PointHistory {
  actual: Point,
  history: VecDeque<Direction>,
}

impl PointHistory {
  fn new(p: Point, history: VecDeque<Direction>) -> PointHistory{
    PointHistory{actual:p, history}
  }

  fn from_dir(&self, d: &Direction, new_history: &VecDeque<Direction>, grid:&Vec<Vec<u32>>, minimum_step: isize) -> Vec<(PointHistory, u32)>{
    
    let dir = match d{
        Direction::UP => Point::new(0, -1),
        Direction::DOWN => Point::new(0, 1),
        Direction::LEFT => Point::new(-1, 0),
        Direction::RIGHT => Point::new(1, 0),
    };

    let mut next = self.actual;
    let mut cost = 0;
    for _ in 0..minimum_step{
      next.x += dir.x;
      next.y += dir.y;
      if !next.is_valid(){
        return vec![];
      }
      cost += grid[next.y as usize][next.x as usize];
    }
    
    let mut n = new_history.clone();
    n.push_back(d.clone());
    return vec![(PointHistory::new(next, n), cost)];
  }

  fn successors(&self, grid:&Vec<Vec<u32>>) -> Vec<(PointHistory, u32)> {
    let mut successors: Vec<(PointHistory, u32)> = vec!();

    let mut print = false;

    if print{
      println!("{:?} history {:?}", self.actual, self.history);
    }

    const DESIRED_DIRECTION_HISTORY_LEN: usize = 10;
    assert!(self.history.len() <= DESIRED_DIRECTION_HISTORY_LEN, "history is {}", self.history.len());

    let mut sum_direction:[usize; DESIRED_DIRECTION_HISTORY_LEN] = [0; DESIRED_DIRECTION_HISTORY_LEN];
    for v in &self.history{
      let index = v.clone();
      sum_direction[index as usize] += 1;
    }

    if print{
      println!("last direction is {:?}", self.history.back());
    }
    
    let new_history = {
      let history_len = self.history.len();
      if history_len < DESIRED_DIRECTION_HISTORY_LEN{
        self.history.clone()
      }else{
        let mut h = VecDeque::new();
        for i in (1..DESIRED_DIRECTION_HISTORY_LEN).rev(){
          h.push_back(self.history.get(history_len - i).expect("i checked for size before!").clone());
        }
        h
      }
    };
    
    let mut dir = None;
    if self.history.len() < 4 {
      dir = self.history.back();
    }else{
      let start_index = self.history.len() - 4;
      
      for l in self.history.iter().skip(start_index){
        if l != self.history.back().unwrap(){
          dir = self.history.back();
          break;
        }
      }
    }
    if let Some(dir) = dir{
      if print{
        println!("going forcibly {dir:?}");
      }
      return self.from_dir(dir, &new_history, grid, 1);
    }

    for dir in [Direction::DOWN, Direction::LEFT, Direction::RIGHT, Direction::UP]{
      if sum_direction[dir as usize] < 10 && !self.history.back().is_some_and(|v| *v == dir.inverse()){
        if print{
          println!("going {:?}", dir);
        }
        successors.append(&mut self.from_dir(&dir, &new_history, grid, 1));
      }
    }

    successors
  }

  fn successors2(&self, grid:&Vec<Vec<u32>>) -> Vec<(PointHistory, u32)> {
    let mut successors: Vec<(PointHistory, u32)> = vec!();

    let mut print = false;

    if print{
      println!("{:?} history {:?}", self.actual, self.history);
    }

    const DESIRED_DIRECTION_HISTORY_LEN: usize = 10-4;
    assert!(self.history.len() <= DESIRED_DIRECTION_HISTORY_LEN, "history is {}", self.history.len());

    let mut sum_direction:[usize; DESIRED_DIRECTION_HISTORY_LEN] = [0; DESIRED_DIRECTION_HISTORY_LEN];
    for v in &self.history{
      let index = v.clone();
      sum_direction[index as usize] += 1;
    }

    let last_dir = self.history.back();
    if print{
      println!("last direction is {last_dir:?}");
    }
    
    let new_history = {
      let history_len = self.history.len();
      if history_len < DESIRED_DIRECTION_HISTORY_LEN{
        self.history.clone()
      }else{
        let mut h = VecDeque::new();
        for i in (1..DESIRED_DIRECTION_HISTORY_LEN).rev(){
          h.push_back(self.history.get(history_len - i).expect("i checked for size before!").clone());
        }
        h
      }
    };

    for dir in [Direction::DOWN, Direction::LEFT, Direction::RIGHT, Direction::UP]{
      if sum_direction[dir as usize] < 10-4 && !self.history.back().is_some_and(|v| *v == dir.inverse()){
        if print{
          println!("going {:?}", dir);
        }
        let minimum_step = if last_dir.is_some_and(|d| *d == dir){
          1
        }else{
          4
        };
        successors.append(&mut self.from_dir(&dir, &new_history, grid, minimum_step));
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

    if current.p.actual == *end {
      if current.w < cheaper{
        cheaper = current.w;
        solution.push(current.clone());
        
        let mut i = 0;
        let mut purged = 0;
        while i < stack.len(){
          if stack[i].w + stack[i].p.actual.len(end) as u32 > cheaper{
            stack.remove(i);
            purged+=1;
          }else{
            i+=1;
          }
        }
        println!("Found path, cost: {} {:?} purged {purged}", current.w, current.p.actual);
      }
      continue;
    }

    if current.p.actual.len(end) + current.w as isize + 1 > cheaper as isize{
      continue;
    }

    for path in current.p.successors2(grid) {

      let mut print = false;
      let cost = path.1 + current.w;
      if path.0.actual.len(end) + cost as isize > cheaper as isize{
        if print{
          println!("ignoring expansive path");
        }
        continue;
      }
      let ris = PointVisited{p: path.0.actual.clone(), d: path.0.history.clone(), w: path.1 + current.w};
      
      if !visited.contains(&ris) {
        if print{
          println!("visiting {:?}", ris);
        }
        stack.push_back(PointWeight{p:path.0, w: path.1 + current.w});
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
    println!("Found path, cost: {} {:?}", current.w, current.p.actual);
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
    println!();
    
    let start = PointHistory::new(Point{ x: 0, y: 0 }, vec![].into());
    let goal = Point{ x: MAP_SIZE-1, y: MAP_SIZE-1 };

    //bruteforce2(&start, &grid, &Point{ x: MAP_SIZE-1, y: MAP_SIZE-1 });
    //assert!(false);
    let result = pathfinding::directed::astar::astar(
      &start,
      |p| p.successors2(&grid),
      |p| p.actual.len(&goal) as u32,
      |p| p.actual==goal);

    let result = result.expect("no solution found");

    let mut grid = grid.clone();
    for p in &result.0{
      grid[p.actual.y as usize][p.actual.x as usize] = 0;
    }

    for y in &grid{
      for x in y{
        if *x == 0{
          print!("\x1b[31mX\x1b[0m");
        }else{
          print!("\x1b[32m{x}\x1b[0m");
        }
      }
      println!();
    }
    println!();
    println!("result: {result:?}");
    // let mut sum = 0;
    /*let result = result.unwrap();
    for (p, w) in &result{
      sum += w;
    }
    println!("cost: {sum:?}");
    */
}
