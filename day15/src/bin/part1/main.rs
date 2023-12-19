fn main() {
    let data = include_bytes!("../../../data/input.txt");


    //let data = "rn".as_bytes();
    let mut total_sum: usize = 0;
    let mut sum: usize = 0;
    for c in data{
        let c = *c;
        if c == b'\n'{
            continue;
        }
        if c == b','{
            total_sum += sum;
            //println!("sum {sum}");
            sum = 0;
            continue;
        }
        //println!("1 {c} {} {sum}", c as char);
        sum += c as usize;
        //println!("2 {c} {} {sum}", c as char);
        sum = sum.overflowing_mul(17).0;

        sum = sum % 256;

        //println!("3 {c} {} {sum}", c as char);
    }
    total_sum += sum;
    //println!("sum {sum}");

    println!("total_sum {total_sum}");

}
