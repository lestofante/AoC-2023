use std::collections::{VecDeque, HashSet};

use pathfinding::prelude::astar;

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
      self.x < 13 && self.y < 13
    }
}

#[derive(Debug, Clone)]
struct PointHistory {
  actual: Point,
  history: VecDeque<Point>,
  full_history: HashSet<Point>,
}

impl PointHistory {
  fn new(p: Point, history: VecDeque<Point>, full_history: HashSet<Point>) -> PointHistory{
    PointHistory{actual:p, history, full_history}
  }

  fn successors(&self, grid:&Vec<Vec<u32>>) -> Vec<(PointHistory, u32)> {
    let mut successors: Vec<(PointHistory, u32)> = vec!();

    //println!("{:?} history {:?}", self.actual, self.history);

    let mut visited = Point{x:0, y:0};
    assert!(self.history.len() <= 2, "history is {}", self.history.len());
    // we expect only 2
    for v in &self.history{
      visited.x += self.actual.x - v.x;
      visited.y += self.actual.y - v.y;
    }

    //println!("visited {visited:?}");

    let mut new_history = self.history.clone();
    while new_history.len() > 1{
      new_history.pop_front();
    }
    new_history.push_back(self.actual);
    let mut new_full_history = self.full_history.clone();
    new_full_history.insert(self.actual);

    if visited.x < 4{
      // travel +x ok
      let next = Point::new(self.actual.x + 1, self.actual.y);
      if next.is_valid() && !new_full_history.contains(&next) {
        successors.push((PointHistory::new(next, new_history.clone(), new_full_history.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    if visited.x > -4{
      // travel -x ok
      let next = Point::new(self.actual.x - 1, self.actual.y);
      if next.is_valid() && !new_full_history.contains(&next){
        successors.push((PointHistory::new(next, new_history.clone(), new_full_history.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    if visited.y < 4{
      // travel +y ok
      let next = Point::new(self.actual.x, self.actual.y + 1);
      if next.is_valid() && !new_full_history.contains(&next){
        successors.push((PointHistory::new(next, new_history.clone(), new_full_history.clone()), grid[next.y as usize][next.x as usize]));
      }
    }

    if visited.y > -4{
      // travel -y ok
      let next = Point::new(self.actual.x, self.actual.y - 1);
      if next.is_valid() && !new_full_history.contains(&next){
        successors.push((PointHistory::new(next, new_history.clone(), new_full_history.clone()), grid[next.y as usize][next.x as usize]));
      }
    }
/*
    print!("valid paths ");
    for f in &successors{
      print!("({},{}) ", f.0.actual.x, f.0.actual.y);
    }
    */
    //print!("new_history {new_history:?}");
    //println!("");
    //println!("");

    successors
  }

  fn abs_diff(&self, goal: &PointHistory, grid:&Vec<Vec<u32>>) -> u32 {
      /*
      let x = if self.actual.x > goal.actual.x{
        self.actual.x - goal.actual.x
      }else{
        goal.actual.x - self.actual.x
      };

      let y = if self.actual.y > goal.actual.y{
        self.actual.y - goal.actual.y
      }else{
        goal.actual.y - self.actual.y
      };
      (x+y) as u32
    */
      let mut sum = 0;
      for x in self.actual.x..=goal.actual.x{
        sum += grid[self.actual.y as usize][x as usize];
      }
      for x in goal.actual.x..=self.actual.x{
        sum += grid[goal.actual.y as usize][x as usize];
      }
      for y in self.actual.y..=goal.actual.y{
        sum += grid[y as usize][self.actual.x as usize];
      }
      for y in goal.actual.y..=self.actual.y{
        sum += grid[y as usize][goal.actual.x as usize];
      }
      println!("diff: {:?} {:?} {sum}", self.actual, goal.actual);
      //grid[goal.actual.y as usize][goal.actual.x as usize] + grid[self.actual.y as usize][self.actual.x as usize]
      sum
  }
}

impl PartialEq for PointHistory {
  fn eq(&self, other: &Self) -> bool {
      self.actual == other.actual 
      && self.history == other.history
  }
}

impl Eq for PointHistory {

}

impl std::hash::Hash for PointHistory{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actual.hash(state);
    }
}

const DATA: &str = include_str!("../../../data/test.txt");
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct PointWeight{
  p: PointHistory,
  w: u32,
}

fn bruteforce(actual: &PointHistory,grid:&Vec<Vec<u32>>, end: &Point, cost: u32, cheaper: &mut u32){
  for path in actual.successors(grid){
    if path.0.actual == *end{
      let cost = cost + path.1;
      if cost < *cheaper{
        *cheaper = cost;
        println!("found path, cost {}", cost + path.1);
      }
    }else{
      return bruteforce(&path.0, &grid, &end, cost + path.1, cheaper);
    }
  }
}

fn bruteforce2(start: &PointHistory, grid: &Vec<Vec<u32>>, end: &Point) {
  let mut stack: VecDeque<PointWeight> = VecDeque::new();
  let mut visited: HashSet<PointWeight> = HashSet::new();

  stack.push_back(PointWeight{p:start.clone(), w:grid[start.actual.y as usize][start.actual.x as usize]});

  while let Some(current) = stack.pop_back() {
    //println!("at: {:?}", current.p.actual);
    if current.p.actual == *end {
      println!(">>>>>>>>>.Found path, cost: {} {:?}", current.w, current.p.full_history);
      //return; // Found the path, so stop searching
    }

    for path in current.p.successors(grid) {
      let ris = PointWeight{p:path.0, w: path.1 + current.w};
      
      if !visited.contains(&ris) {
          stack.push_back(ris.clone());
          visited.insert(ris);
      }
    }
  }

  println!("No valid path found");
}


fn main() {
    // Define your grid with movement costs

    let grid:Vec<Vec<u32>> = DATA.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let start = PointHistory::new(Point{ x: 0, y: 0 }, vec![].into(), HashSet::new());
    let goal = PointHistory::new(Point{ x: 12, y: 12 }, vec![].into(), HashSet::new());

    bruteforce2(&start, &grid, &Point{ x: 12, y: 12 });

    return;

    // Use A* algorithm to find the path
    let result = astar(
        &start,
        |p| p.successors(&grid),
        |p| p.abs_diff(&goal, &grid),
        |p| *p == goal,
    );

    //let result.unwrap().iter().map(|p| println!("({},{})", p.1));
    let result = result.expect("No solution found");
    println!("Path:");
    for p in result.0{
      println!("{},{} {}", p.actual.x, p.actual.y, grid[p.actual.y as usize][p.actual.x as usize]);
    }
    
    println!("sum: {}", result.1);
}
