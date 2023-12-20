use std::collections::HashMap;

const DATA: &str = include_str!("../../../data/input.txt");

#[derive(Debug, Clone, Copy)]
struct Range{
    min: usize,
    max: usize,
}

fn main() {
    let mut map = HashMap::<String, Workflow>::new();
    for line in  DATA.lines(){
        if line == ""{
            break;
        }
        let ris = extract_workflow(line);
        map.insert(ris.name.clone(), ris);
    };

    // for r in map.values(){
    //     println!("{r:?}");
    // }

    let r = GearInput { data: [Range{min: 1, max: 4000};4] };

    let valid_ranges = execute(r, map.get("in").expect("must exist"), &map, 0);

    //println!("valid_ranges {valid_ranges:?}");

    let mut sum = 0;
    for g in valid_ranges{
        let mut mul = 1;
        for r in g.data{
            mul *= r.max - r.min + 1;
        }
        sum += mul;
    }

    println!("sum is {sum:?}");
    assert_eq!(167409079868000, sum);
}

fn execute(mut r: GearInput, point: &Workflow, map: &HashMap<String, Workflow>, num:usize) -> Vec<GearInput>{
    let mut ris: Vec<GearInput> = vec![];
    for p in &point.instructions{
        for _ in 0..num{
            print!("  ");
        }
        // println!("exec: {p:?}");
        match p {
            WorkflowInstruction::Comparison(c) => match c.operatoion{
                Operator::MAJOR => {
                    let value = r.get(&c.gear);
                    if value.min > c.value{
                        for _ in 0..num{
                            print!("  ");
                        }
                        // println!("execute_basic MAJOR {r:?}");
                        ris.append(&mut execute_basic(&c.jump, &r, map, num));
                    }
                    if value.max > c.value{
                        
                        //time to split
                        {
                            let mut r_max = r.clone();
                            
                            let mut part_max = value;
                            part_max.min = c.value + 1;
                            r_max.set(c.gear, part_max);
                            for _ in 0..num{
                                print!("  ");
                            }
                            // println!("execute_basic MAJOR split1 {r_max:?}");
                            ris.append(&mut execute_basic(&c.jump, &r_max, map, num));
                        }
                        let mut part_min = value;
                        part_min.max = c.value;
                        r.set(c.gear, part_min);
                        for _ in 0..num{
                            print!("  ");
                        }
                        // println!("execute_basic MAJOR split2 {r:?}");
                    }
                },
                Operator::MINOR => {
                    let value = r.get(&c.gear);
                    if value.max < c.value{
                        for _ in 0..num{
                            print!("  ");
                        }
                        // println!("execute_basic MINOR split {r:?} {}", c.value);
                        ris.append(&mut execute_basic(&c.jump, &r, map, num));
                    }
                    if value.min < c.value{
                        
                        //time to split
                        {
                            let mut r_min = r.clone();
                            
                            let mut part_min = value;
                            part_min.max = c.value - 1;
                            r_min.set(c.gear, part_min);

                            for _ in 0..num{
                                print!("  ");
                            }
                            // println!("execute_basic MINOR split1 {r_min:?}");
                            ris.append(&mut execute_basic(&c.jump, &r_min, map, num));
                            for _ in 0..num{
                                print!("  ");
                            }
                            // println!("MINOR split1 result {ris:?}");
                        }
                        let mut part_max = value;
                        part_max.min = c.value;
                        r.set(c.gear, part_max);
                        for _ in 0..num{
                            print!("  ");
                        }
                        // println!("execute_basic MINOR split2 {r:?}");
                    }
                },
            },

            WorkflowInstruction::Basic(b) => {
                ris.append(&mut execute_basic(b, &r, map, num));
                for _ in 0..num{
                    print!("  ");
                }
                // println!("Basic completed {ris:?}");
                return ris;
            }
        }
    }
    //shpould never run out of instructions
    assert!(false);
    return vec![];
}

fn execute_basic(b: &WorkflowBasic, r: &GearInput, map: &HashMap<String, Workflow>, num: usize) -> Vec<GearInput> {
    match b{
        WorkflowBasic::Jump(a) => return execute(*r, map.get(a).expect("must exist"), map, num+1),
        WorkflowBasic::Accept => {
            for _ in 0..num{
                print!("  ");
            }
            println!("ACCEPTED {r:?}");
            return vec![*r];
        },
        WorkflowBasic::Reject => {
            for _ in 0..num{
                print!("  ");
            }
            println!("REJECTED");
            return vec![];
        },
    }
}

#[derive(Debug, Clone, Copy)]
struct GearInput{
    data:[Range;4],
}
impl GearInput {

    fn set(&mut self, i: Gears, sum: Range) {
        self.data[i as usize] = sum;
    }

    fn get(&self, g: &Gears) -> Range {
        self.data[g.clone() as usize]
    }
}

#[derive(Debug)]
struct Workflow{
    name: String,
    instructions: Vec<WorkflowInstruction>,
}

#[derive(Debug)]
enum WorkflowBasic{
    Jump(String),
    Accept,
    Reject
}

#[derive(Debug)]
enum WorkflowInstruction{
    Comparison(Comparison),
    Basic(WorkflowBasic),
}

fn extract_workflow(line: &str) -> Workflow {
    let mut name: String = String::new();
    for c in line.chars(){
        if c == '{'{
            break;
        }
        name.push(c);
    }

    //remove {}
    assert!(line.len() > name.len()+2);
    let line = &line[(name.len()+1)..line.len() - 1];

    let instructions = line.split(",").map(|workflow_part| {
        extract_instruction(workflow_part)
    }).collect();

    Workflow{name: name, instructions: instructions}
}

#[derive(Debug, Clone, Copy)]
enum Gears{
    X,
    M,
    A,
    S,
}

impl Gears {
    fn from(i:usize) -> Gears {
        match i {
            0 => Gears::X,
            1 => Gears::M,
            2 => Gears::A,
            3 => Gears::S,
            _ => {assert!(false); Gears::X}
        }
    }
}

#[derive(Debug)]
enum Operator{
    MAJOR,
    MINOR,
}

#[derive(Debug)]
struct Comparison{
    gear: Gears,
    operatoion: Operator, 
    value: usize, 
    jump: WorkflowBasic,
}
impl Comparison {
    fn new(gear: Gears, operatoion: Operator, value: usize, jump: WorkflowBasic) -> Comparison {
        Comparison{
            gear, operatoion, value, jump
        }
    }
}

fn extract_instruction(workflow_part: &str) -> WorkflowInstruction {

    let split: Vec<&str> = workflow_part.split(":").collect();
    if split.len() == 1{
        return WorkflowInstruction::Basic(extract_instruction_basic(workflow_part));
    }

    let mut iter = split[0].chars();
    let g = match iter.next().expect("must have a gear"){
        'x' => Gears::X,
        'm' => Gears::M,
        'a' => Gears::A,
        's' => Gears::S,
        _ => {assert!(false); Gears::S},
    };

    let op = match iter.next().expect("must have a gear"){
        '>' => Operator::MAJOR,
        '<' => Operator::MINOR,
        _ => {assert!(false); Operator::MAJOR}
    };

    let mut value = 0;
    for c in iter{
        value *= 10;
        value += (c as u8 - b'0') as usize;
    }

    return WorkflowInstruction::Comparison(Comparison::new(g, op, value, extract_instruction_basic(split[1])));
}

fn extract_instruction_basic(part: &str) -> WorkflowBasic {
    if part == "A"{
        return WorkflowBasic::Accept;
    }
    if part == "R"{
        return WorkflowBasic::Reject;
    }
    return WorkflowBasic::Jump(part.to_owned());
}
