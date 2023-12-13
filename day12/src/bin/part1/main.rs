struct Element<'a>{
    line: &'a str,
    groups: Vec<usize>,
}

fn find_combinations(d: &Element)->usize{
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
        for c in &chars{
            if n & 1 == 1{
                bytes[*c] = b'#';
            }else{
                bytes[*c] = b'.';
            }
            n = n >> 1;
        }
        let sum_hash = bytes.iter().filter(|b| **b == b'#').count();
        let sum_expected_hash: usize = d.groups.iter().sum();
        //println!("sum_hash {sum_hash} sum_expected_hash {sum_expected_hash} for {}", String::from_utf8(bytes.clone()).unwrap());
        if sum_hash == sum_expected_hash{
            let mut in_group = false;
            let mut sum = 0;
            let mut sum_groups: Vec<usize>=vec!();
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
                    sum_groups.push(sum);
                    sum = 0;
                }
            }
            if in_group{
                sum_groups.push(sum);
            }
            //println!("got combination: {:?}", sum_groups);
            if d.groups == sum_groups{
                //println!("valid combination: {}", String::from_utf8(bytes.clone()).unwrap());
                sum_combination += 1;
            }
        }
    }
    return sum_combination;
}

fn main() {
    let data = include_str!("../../../data/input.txt");

    let data:Vec<Element> = data.lines().map(|line| {
            let line = line.split(" ").collect::<Vec<&str>>();
            assert!(line.len() == 2);
            let groups:Vec<usize> = line[1].split(",").map(|v|v.parse::<usize>().unwrap()).collect();
            Element{
                line: line[0],
                groups,
            }
        }
    ).collect();
    
    let mut sum = 0;
    for d in data{
        sum += find_combinations(&d);
    }
    println!("{sum}");
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test() {
        assert_eq!(find_combinations(&Element{
            groups: vec![1,1,3],
            line: "???.###",
        }), 1);
        assert_eq!(find_combinations(&Element{
            groups: vec![1,1,3],
            line: ".??..??...?##.",
        }), 4);
        assert_eq!(find_combinations(&Element{
            groups: vec![1,3,1,6],
            line: "?#?#?#?#?#?#?#?",
        }), 1);
        assert_eq!(find_combinations(&Element{
            groups: vec![4,1,1],
            line: "????.#...#...",
        }), 1);
        assert_eq!(find_combinations(&Element{
            groups: vec![1,6,5],
            line: "????.######..#####.",
        }), 4);
        assert_eq!(find_combinations(&Element{
            groups: vec![3,2,1],
            line: "?###????????",
        }), 10);
         
    }
}