use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Element{
    line:String,
    groups: Vec<usize>,
}

fn all_combinations(num_hash: usize, total_length: usize) -> Vec<usize> {
    let num = 2_u64.pow(num_hash as u32) as usize - 1;
    //let max = num << (total_length - num_hash);

    //let mut combinations = vec!();
    let mut combinations = HashSet::new();
    //combinations.push(max);
    //println!("max {max:0width$b}", width=total_length);
    
    //println!("expeceted shift {}",total_length - num_hash);
    for shift in 0..=(total_length - num_hash){
        let mut num = num << shift;
        combinations.insert(num);
        //println!("num a {num:0width$b}", width=total_length);
        for i in 0..num_hash {
            // remove rightmost to leftmost
            num &= !(1<<i+shift);

            for y in 1..shift{
                let y = y-1;
                num |= 1<<y;
                //println!("num b {num:0width$b}", width=total_length);
                combinations.insert(num);
                num &= !(1<<y);
            }

            for y in (shift+num_hash)..total_length{
                num |= 1<<y;
                //println!("num c {num:0width$b}", width=total_length);
                combinations.insert(num);
                num &= !(1<<y);
            }

            num |= 1<<i+shift;
        }
        //combinations.push(num);
        /*
        for i in shift..=(total_length - num_hash) {
            num &= !(1<<i);
            //println!("num b {num:0width$b} {shift}", width=total_length);
            for j in num_hash+shift..=total_length-1{
                num |= 1<<j;
                println!("num c {num:0width$b}", width=total_length);

                combinations.push(num);
                
                num &= !(1<<j);
                //println!("num d {num:b}");
            }
            num |= 1<<i;
            //println!("num e {num:b}");
        }
        */
        //println!("NEW NUM {num:b}");
    }

    combinations.into_iter().collect()
}

fn check_group(bytes: &Vec<u8>, d: &Element) -> bool{
    let mut in_group = false;
    let mut sum = 0;
    //let mut sum_groups: Vec<usize>=vec!();

    //println!("checking {}", String::from_utf8(bytes.clone()).unwrap());
    let mut iter_group = d.groups.iter();
    for b in bytes{
        if in_group && *b == b'#'{
            sum += 1;
        }
        if !in_group && *b == b'#'{
            in_group = true;
            sum += 1;
        }
        if in_group && *b == b'.'{
            in_group = false;
            let expected = iter_group.next();
            //println!("checking group {sum} {}", *expected.unwrap_or(&0));
            if sum != *expected.unwrap_or(&0){
                //println!("Invalid");
                return false;
            }
            sum = 0;
        }
    }
    if in_group{
        let expected = iter_group.next();
        if sum != *expected.unwrap_or(&0){
            //println!("Invalid groups {sum} {}", *expected.unwrap_or(&0));
            return false;
        }
    }
//    println!("got combination: {:?}", sum_groups);
    //println!("valid combination: {}", String::from_utf8(bytes.clone()).unwrap());
    return true;
}

fn find_combinations_old(d: &Element)->usize{
    let sum_expected_hash: usize = d.groups.iter().sum();
    let sum_unk = d.line.chars().filter(|c| *c == '?').count();
    let sum_known_hash = d.line.chars().filter(|c| *c == '#').count();

    let missing_hash = sum_expected_hash - sum_known_hash;
    let min = 2_usize.pow(missing_hash as u32) - 1;
    let max =2_usize.pow(sum_unk as u32) - 1;
    println!("------\n{missing_hash} from {min:b} to {max:b} for {} {:?}", d.line, d.groups);
    let str = d.line.to_owned();

    let mut bytes = str.into_bytes();
    let chars: Vec<usize> = bytes.iter().enumerate().filter(|(_, c)| **c == b'?').map(|(n, _)| n).collect();
    
    let mut sum_combination = 0;

    for mut n in min..=max{
        let mut sum_hash = 0;
        for c in &chars{
            if n & 1 == 1{
                bytes[*c] = b'#';
                sum_hash+=1;
            }else{
                bytes[*c] = b'.';
            }
            n = n >> 1;
        }
        
        //println!("sum_hash {sum_hash} missing_hash {missing_hash} for {}", String::from_utf8(bytes.clone()).unwrap());
        if sum_hash != missing_hash{
            continue;
        }

    }
    println!("gen complete");
    for mut n in min..=max{

        let mut sum_hash = 0;
        for c in &chars{
            if n & 1 == 1{
                bytes[*c] = b'#';
                sum_hash+=1;
            }else{
                bytes[*c] = b'.';
            }
            n = n >> 1;
        }
        
        //println!("sum_hash {sum_hash} missing_hash {missing_hash} for {}", String::from_utf8(bytes.clone()).unwrap());
        if sum_hash != missing_hash{
            continue;
        }

        let mut in_group = false;
        let mut sum = 0;
        let mut iter_groups = d.groups.iter();
        for b in &bytes{
            if in_group && *b == b'#'{
                sum += 1;
            }
            if !in_group && *b == b'#'{
                in_group = true;
                sum += 1;
            }
            if in_group && *b == b'.'{
                in_group = false;
                let expected = iter_groups.next();
                if *expected.unwrap_or(&0) != sum{
                    continue;
                }
                sum = 0;
            }
        }
        if in_group{
            let expected = iter_groups.next();
            if *expected.unwrap_or(&0) != sum{
                continue;
            }
        }
        sum_combination += 1;
    }
    println!("check complete");
    return sum_combination;
}

fn find_combinations(d: &Element)->usize{
    let sum_expected_hash: usize = d.groups.iter().sum();
    let sum_unk = d.line.chars().filter(|c| *c == '?').count();
    let sum_known_hash = d.line.chars().filter(|c| *c == '#').count();

    let missing_hash = sum_expected_hash - sum_known_hash;
    let min = 2_usize.pow(missing_hash as u32) - 1;
    let max =2_usize.pow(sum_unk as u32);
    println!("------\n{missing_hash} from {min:b} to {max:b} for {} {:?}", d.line, d.groups);
    let str = d.line.to_owned();

    let mut bytes = str.into_bytes();
    let chars: Vec<usize> = bytes.iter().enumerate().filter(|(_, c)| **c == b'?').map(|(n, _)| n).collect();

    let ris = all_combinations(missing_hash, chars.len());
    
    let mut sum_combination = 0;

    for c in ris{
        let mut v = c;
        let mut i = 0;
        println!("checking {v:0width$b}", width=sum_unk);
        for _ in 0..chars.len(){
            //println!("v {v:b}");
            if v & 1 == 1{
                bytes[chars[i]] = b'#';
            }else{
                bytes[chars[i]] = b'.';
            }
            v = v >> 1;
            i += 1;
        }
        sum_combination += if check_group(&bytes, &d) {1}else{0};
    }
    println!("sum_combination:{sum_combination}");
    return sum_combination;
}

fn parse(data:&str) -> Vec<Element>{
    data.lines().map(|line| {
        let line = line.split(" ").collect::<Vec<&str>>();
        assert!(line.len() == 2);
        let groups:Vec<usize> = line[1].split(",").map(|v|v.parse::<usize>().unwrap()).collect();
        Element{
            line: line[0].to_owned()+"?"+line[0]+"?"+line[0]+"?"+line[0]+"?"+line[0],
            groups: (0..5).flat_map(|_| groups.iter().cloned()).collect(),
        }
    }
    ).collect()
}

fn main() {
    let data = include_str!("../../../data/test.txt");

    let data:Vec<Element> = parse(data);
    
    let mut sum = 0;
    for d in data{
        sum += find_combinations_old(&d);
    }
    println!("{sum}");
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {

        assert_eq!(find_combinations_old(&Element{
            groups: vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3],
            line: "???.###????.###????.###????.###????.###".to_owned(),
        }), 1);

        assert_eq!(find_combinations_old(&Element{
            groups: vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3],
            line: ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.".to_owned(),
        }), 1);

        assert_eq!(find_combinations_old(&Element{
            groups: vec![1,1,3],
            line: "???.###".to_owned(),
        }), 1);

        assert_eq!(find_combinations_old(&Element{
            groups: vec![1,1,3],
            line: ".??..??...?##.".to_owned(),
        }), 4);
        assert_eq!(find_combinations_old(&Element{
            groups: vec![1,3,1,6],
            line: "?#?#?#?#?#?#?#?".to_owned(),
        }), 1);
        assert_eq!(find_combinations_old(&Element{
            groups: vec![4,1,1],
            line: "????.#...#...".to_owned(),
        }), 1);
        assert_eq!(find_combinations_old(&Element{
            groups: vec![1,6,5],
            line: "????.######..#####.".to_owned(),
        }), 4);
        assert_eq!(find_combinations_old(&Element{
            groups: vec![3,2,1],
            line: "?###????????".to_owned(),
        }), 10);     
         
    }
}