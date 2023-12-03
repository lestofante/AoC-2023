#![feature(extract_if)]

#[derive(Debug)]
struct Point{
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line{
    start: Point,
    len: u32,
}

fn touch(line: &Line, p: &Point) -> bool{

    //found symbol at 85 2
    //touched number: Number { val: 381, pos: Line { start: Point { x: 81, y: 1 }, len: 3 } } fromSymbol { pos: Point { x: 85, y: 2 } }


    // 85 < 81-1
    if line.start.x > 0 && p.x < line.start.x - 1{
        return false;
    }
    
    // 85 >= 81+3+1
    if p.x > line.start.x + line.len{
        return false;
    }
    
    if line.start.y > 0 && p.y < line.start.y - 1{
        return false;
    }
    
    if p.y > line.start.y + 1{
        return false;
    }
    true
}

#[derive(Debug)]
struct Symbol{pos: Point}

#[derive(Debug)]
struct Number{val: u32, pos: Line}

struct Map{
    sum: u32,

    last_line_sym: Vec<Symbol>,
    current_line_sym: Vec<Symbol>,

    last_line_num: Vec<Number>,
    current_line_num: Vec<Number>,

    remaining: Vec<Number>,
}

impl Map{
    fn new() -> Map{
        Map{sum: 0, last_line_sym: vec!(), current_line_sym: vec!(), last_line_num: vec!(), current_line_num: vec!(), remaining: vec!()}
    }

    fn add_num(&mut self, val: Number){
        for sym in &self.last_line_sym{
            if touch(&val.pos, &sym.pos){
                //they touch! do not add the number to the list, it is consumed
                self.sum += val.val;
                println!("touched number: {val:?} from{sym:?}");
                return;
            }
        }
        for sym in &self.current_line_sym{
            if touch(&val.pos,&sym.pos){
                //they touch! do not add the number to the list, it is consumed
                self.sum += val.val;
                println!("touched number: {val:?} from{sym:?}");
                return;
            }
        }
        //not touch, add to the list in case get touched later
        self.current_line_num.push(val);
    }

    fn add(&mut self, sym: Symbol){
        for touched in self.last_line_num.extract_if(|x| touch(&x.pos, &sym.pos)){
            //they touch! number removed from the list, it is consumed
            self.sum += touched.val;
            println!("touched number: {touched:?} from{sym:?}");
        }
        for touched in self.current_line_num.extract_if(|x| touch(&x.pos, &sym.pos)){
            //they touch! number removed from the list, it is consumed
            self.sum += touched.val;
            println!("touched number: {touched:?} from{sym:?}");
        }
        self.current_line_sym.push(sym);
    }

    fn next_line(mut self) -> Map{
        self.remaining.append(&mut self.last_line_num);
        self.last_line_num = self.current_line_num;
        self.current_line_num = vec!();

        self.last_line_sym = self.current_line_sym;
        self.current_line_sym = vec!();

        self
    }

    fn print_remaining(&self){
        println!("\n\n----Unused numbers: ");
        for val in &self.remaining{
            println!("{val:?}");
        }
    }
}

fn main() {
    let input_file = include_bytes!("../../../data/input.txt");
    
    parse_data(input_file);
}

fn parse_data(input_file: &[u8]){
    let mut mappa = Map::new();

    let mut x = 0;
    let mut y = 0;

    let mut num: Option<Number> = None;
    for c in input_file{
        if let b'0'..=b'9' = c{
            if let Some(ref mut n) = num{
                n.val *= 10;
                n.val += (c-b'0') as u32;
                n.pos.len += 1;
            }else{
                num = Some(Number{
                    val: (c-b'0') as u32,
                    pos: Line{
                        start: Point{x:x, y:y}, 
                        len: 1}, 
                    }
                );
            }
        }else{
            if let Some(n) = num{
                println!("found number {} at {:?}", n.val, n.pos);
                mappa.add_num(n);
                num = None;
            }
            
            if *c != b'.' && *c != b'\n'{
                println!("found symbol at {x} {y}");
                mappa.add(Symbol { pos: Point { x: x, y: y } });
            }
        }
        if *c == b'\n'{
            println!("---- end {x}, {y}");
            x = 0;
            y += 1;
            mappa = mappa.next_line();
        }else{
            x += 1;
        }
    }
    mappa.print_remaining();
    println!("sum is: {}", mappa.sum);
}


#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_touch() {
        let l1 = Line{
            start: Point{x:3,y:2},
            len: 4,
        };

        // left
        for x in 0..2{
            for y in 0..=10{
                let p = Point{x: x, y: y};
                assert!(!touch(&l1, &p), "expected false, fail for {l1:?} and {p:?}");
            }
        }

        // top
        for x in 0..10{
            for y in 0..1{
                let p = Point{x: x, y: y};
                assert!(!touch(&l1, &p), "expected false, fail for {l1:?} and {p:?}");
            }
        }

        //hit!
        for x in 2..=8{
            for y in 1..=3{
                let p = Point{x: x, y: y};
                assert!(touch(&l1, &p), "expected true, fail for {l1:?} and {p:?}");
            }
        }
    }

    #[test]
    fn test_touch2() {
        let l1 = Line{
            start: Point{x:81,y:1},
            len: 3,
        };
        let p = Point{x:85,y:2};
        assert!(!touch(&l1, &p), "expected true, fail for {l1:?} and {p:?}");
    }
        

    #[test]
    fn test_map() {
        let input_file = include_bytes!("../../../data/test.txt");
    
        parse_data(input_file);
    }
}