fn get_index(x:usize, y:usize) -> usize{
    y*(COLS+1)+x
}

fn get_tot_sum(data: &[u8; (COLS+1)*COLS]) -> usize{
    let mut tot_sum = 0;
    for x in 0..COLS{
        //println!("col {x}");
        let mut sum = 0;
        let mut next_sum = COLS+1;
        for y in 0..COLS{
            match data[get_index(x, y)]{
                b'O' => {next_sum-=1; sum += next_sum; }
                b'.' => (),
                b'#' => next_sum = COLS-y,
                b'\n' => assert!(false),
                _ => assert!(false),
            }
            //println!("sum for {y} is {sum} next is {next_sum}");
        }
        tot_sum += sum;
    }
    println!("tot_sum {tot_sum}");
    tot_sum
}

fn print(data: &[u8; (COLS+1)*COLS]){
    println!();
    for y in 0..COLS{
        
        for x in 0..COLS{
            print!("{}", data[get_index(x, y)] as char);
        }
        println!("");
    }
    println!();
}

const COLS: usize = 10;
fn main() {
    let data = include_bytes!("../../../data/test.txt");

    assert_eq!(data.len()%(COLS+1), 0);

    print(data);
    get_tot_sum(data);
}
