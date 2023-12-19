use std::collections::HashMap;

const DATA: &str = include_str!("../../../data/input.txt");

#[derive(PartialEq, Eq)]
enum ParsingStatus{
    ExtractingCommands,
    ExtractingInputs,
}

fn main() {
    let mut map = HashMap::<String, Workflow>::new();
    let mut inputs: Vec<GearInput> = vec!();
    let mut status = ParsingStatus::ExtractingCommands;
    for line in  DATA.lines(){
        if line == ""{
            status = ParsingStatus::ExtractingInputs;
            continue;
        }
        if status == ParsingStatus::ExtractingCommands{
            let ris = extract_workflow(line);
            map.insert(ris.name.clone(), ris);
        }else{
            let ris = extract_input(line);
            inputs.push(ris);
        }
    };

    for r in &inputs{
        println!("{r:?}");
    }

    for r in map.values(){
        println!("{r:?}");
    }

    let mut sum = 0;
    for r in &inputs{
        if execute(&r, map.get("in").expect("must exist"), &map) {
            for i in 0..r.data.len(){
                sum += r.data[i];
            }
        }
    }
    println!("sum is {sum:?}")
}

fn execute(r: &GearInput, point: &Workflow, map: &HashMap<String, Workflow>) -> bool{
    for p in &point.instructions{
        match p {
            WorkflowInstruction::Comparison(c) => match(c.operatoion){
                Operator::MAJOR => if r.get(&c.gear) > c.value{
                    return execute_basic(&c.jump, r, map);
                },
                Operator::MINOR => if r.get(&c.gear) < c.value{
                    return execute_basic(&c.jump, r, map);
                },
            },

            WorkflowInstruction::Basic(b) => return execute_basic(b, r, map),
        }
    }
    //shpould never run out of instructions
    assert!(false);
    return false;
}

fn execute_basic(b: &WorkflowBasic, r: &GearInput, map: &HashMap<String, Workflow>) -> bool {
    match b{
        WorkflowBasic::Jump(a) => return execute(r, map.get(a).expect("must exist"), map),
        WorkflowBasic::Accept => return true,
        WorkflowBasic::Reject => return false,
    }
}

#[derive(Debug)]
struct GearInput{
    data:[usize;4],
}
impl GearInput {
    fn new() -> GearInput {
        GearInput{data:[0;4]}
    }

    fn set(&mut self, i: Gears, sum: usize) {
        self.data[i as usize] = sum;
    }

    fn get(&self, g: &Gears) -> usize {
        self.data[g.clone() as usize]
    }
}

fn extract_input(line: &str) -> GearInput {
    assert!(line.len() > 2);
    let line = &line[1..line.len()-1];

    // the order is always {x=XXXX,m=XXXX,a=XXXX,s=XXXX}
    // important: this is also the order in enum Gears
    let line = line.split(",");
    let mut ris = GearInput::new();
    
    for (i, l) in line.enumerate(){
        let mut sum = 0;
        
        //skip X=
        for c in l.chars().skip(2){
            sum *= 10;
            sum += c as usize - b'0' as usize;
        }
        ris.set(Gears::from(i), sum);
    }
    ris
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
