use std::time::SystemTime;

fn get_index(x:usize, y:usize) -> usize{
    y*(COLS+1)+x
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

fn north(data: &mut [u8; (COLS+1)*COLS]) {
    for x in 0..COLS{
        let mut next_free = 0;
        for y in 0..COLS{
            let cell_idx = get_index(x, y);
            match data[cell_idx]{
                b'O' => {
                    if next_free != y{
                        data[get_index(x, next_free)] = data[cell_idx]; data[cell_idx] = b'.'; next_free+=1;
                    }else{
                        next_free+=1;
                    }
                }
                b'.' => (),
                b'#' => next_free = y+1,
                b'\n' => assert!(false),
                _ => assert!(false),
            }
        }
    }
}

fn south(data: &mut [u8; (COLS+1)*COLS]) {
    //println!("south");
    for x in 0..COLS{
        //println!("x is {x}");
        let mut next_free = COLS-1;
        for y in (0..COLS).rev(){
            let cell = get_index(x, y);
            match data[cell]{
                b'O' => {
                    if next_free != y{
                        data[get_index(x, next_free)] = data[cell]; 
                        data[cell] = b'.'; 
                        if next_free > 0 {next_free-=1;}
                    }else{
                        if next_free > 0{
                            next_free-=1;
                        }
                    }
                }
                b'.' => (),
                b'#' => if y>0{next_free = y-1},
                b'\n' => assert!(false),
                _ => assert!(false),
            }
            //println!("y is {y} next_free {next_free} cell was {} is {}", cell as char, data[get_index(x, y)] as char);
        }
    }
}

fn west(data: &mut [u8; (COLS+1)*COLS]) {
    for y in 0..COLS{
        let mut next_free = 0;
        for x in 0..COLS{
            let cell = get_index(x, y);
            match data[cell]{
                b'O' => {
                    if next_free != x{
                        data[get_index(next_free, y)] = data[cell]; data[cell] = b'.'; next_free+=1;
                    }else{
                        next_free+=1;
                    }
                }
                b'.' => (),
                b'#' => next_free = x+1,
                b'\n' => assert!(false),
                _ => assert!(false),
            }
        }
    }
}

fn east(data: &mut [u8; (COLS+1)*COLS]) {
    for y in 0..COLS{
        let mut next_free = COLS-1;
        for x in (0..COLS).rev(){
            let cell = get_index(x, y);
            match data[cell]{
                b'O' => {
                    if next_free != x{
                        data[get_index(next_free, y)] = data[cell]; data[cell] = b'.'; if next_free> 0 {next_free-=1;}
                    }else{
                        if next_free> 0 {next_free-=1;}
                    }
                }
                b'.' => (),
                b'#' => if x > 0 {next_free= x - 1;}
                b'\n' => assert!(false),
                _ => assert!(false),
            }
        }
    }
}

fn spin_cycle(data: &mut [u8; (COLS+1)*COLS]) {
    north(data);
    //print(data);

    west(data);
    //print(data);

    south(data);
    //print(data);

    east(data);
    //print(data);
    //todo!();
}

struct Match{
    data: [u8; (COLS+1)*COLS],
    loop_number: usize,
}

fn get_tot_sum(data: &[u8; (COLS+1)*COLS]) -> usize{
    let mut tot_sum = 0;
    for x in 0..COLS{
        for y in 0..COLS{
            if data[get_index(x, y)] == b'O'{
                tot_sum += COLS - y;
            }
        }
        
    }
    println!("tot_sum {tot_sum}");
    tot_sum
}

const COLS: usize = 10;
fn main() {
    let data = include_bytes!("../../../data/test.txt");

    get_tot_sum(data);

    assert_eq!(data.len()%(COLS+1), 0);

    let mut data = *data;

    let mut start = SystemTime::now();

    const LOOP: usize = 1000000000;
    const LOOP_UPD: usize = 10000;

    //let mut map : HashMap<usize, Vec<[u8; (COLS+1)*COLS]>> = HashMap::new();
    let mut oldies: Vec<Match> = vec!();

    let mut index_match: Option<(usize, usize)> = None;
    for loop_number in 0..LOOP{
        //let clone = data.clone();
        spin_cycle(&mut data);
        
        //let sum: usize = clone.iter().copied().fold(0, |acc, e| acc + e as usize);

        if let Some(mut index) = index_match{
            index.1 += 1;
            index_match=Some(index);
            if oldies[index.0].data == data{
                println!("END OF LOOP REPETITION at loop {loop_number} loop start {}", index.0);
                print(&data);
                get_tot_sum(&data);

                let lenght_repetition = index.1;
                let missing_repetition = (LOOP - loop_number) % lenght_repetition;
                println!("missing repetition {missing_repetition} lenght_repetition {lenght_repetition} final index {}", index.0+missing_repetition-1);
                get_tot_sum(&oldies[index.0+missing_repetition].data);
                get_tot_sum(&oldies[index.0+missing_repetition-1].data);
                //get_tot_sum(&oldies[end_repetition-1].data);
                //get_tot_sum(&oldies[end_repetition+1].data);
                break;
            }
            if oldies[index.0 + index.1].data == data{
                //ok continue to match
                println!("found part of LOOP {} repetition at loop {loop_number}", index.0 + index.1);
                get_tot_sum(&data);
            }else{
                //only broken dreams
                println!("found broken dreams after {} match at loop {loop_number}", index.1);
                get_tot_sum(&data);
                index_match = None;
            }
        }else{
            if let Some(index) = oldies.iter().rev().position(|x| x.data == data){
                println!("{index}");
                let index = oldies.len() - index - 1; //compensate for inversion
                //print(&data);
                println!("found myself in the past at {index} at loop {loop_number}");
                index_match=Some((index, 0));
            }else{
                println!("not found");
                //oldies.push(Match{data:data, loop_number});
            }
        }
        println!("saving loop {loop_number}");
        //get_tot_sum(&data);
        //print(&data);
        oldies.push(Match{data:data, loop_number});
        
        if loop_number % LOOP_UPD == 0{
            let elapsed = SystemTime::now().duration_since(start).unwrap();
            start = SystemTime::now();
            
            let elapsed_s = elapsed.as_secs_f64();
            let missing = LOOP - (loop_number / LOOP_UPD);

            println!("{loop_number}/1000000000 estimated: {}", (missing as f64) * elapsed_s);
        }
    }

}
