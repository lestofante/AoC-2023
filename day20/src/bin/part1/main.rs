use std::collections::{VecDeque, HashMap};

#[derive(Debug, Clone)]
struct Box{
    m: Module,
    c: Vec<ModuleName>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ModuleName{
    str: String
}

#[derive(Debug, Clone)]
struct Conjunction{
    inputs: HashMap<ModuleName, Pulse>
}

#[derive(Debug, Clone)]
enum ModuleType{
    FlipFlop(Pulse),
    Conjunction(Conjunction),
    Broadcast,
    Button,
}

#[derive(Debug, Clone)]
struct Module{
    name: ModuleName,
    typ: ModuleType,
}

impl Box {
    fn pulse(&mut self, input: &Signal) -> Vec<Signal> {
        match &mut self.m.typ {
            ModuleType::FlipFlop(state) => {
                if input.p == Pulse::Low{
                    state.invert();
                    return self.c.iter().map(|out_m| Signal{p:state.clone(), dst: out_m.clone(), src: self.m.name.clone()}).collect();
                }else{
                    vec![]
                }
            },
            ModuleType::Conjunction(input_module) => {
                input_module.inputs.insert(input.src.clone(), input.p.clone());

                for i in input_module.inputs.values(){
                    if *i != Pulse::High{
                        return self.c.iter().map(|out_m| Signal{p:Pulse::High, dst: out_m.clone(), src: self.m.name.clone()}).collect();
                    }
                }
                return self.c.iter().map(|m| Signal{p:Pulse::Low, dst: m.clone(), src: self.m.name.clone()}).collect();
            },
            ModuleType::Broadcast => self.c.iter().map(|out_m| Signal{p:input.p.clone(), dst: out_m.clone(), src: self.m.name.clone()}).collect(),
            ModuleType::Button => self.c.iter().map(|out_m| Signal{p:Pulse::Low, dst: out_m.clone(), src: self.m.name.clone()}).collect(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse{
    High,
    Low
}
impl Pulse {
    fn invert(&mut self) {
        *self = match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

struct Signal{
    p: Pulse,
    dst: ModuleName,
    src: ModuleName,
}

fn load() -> HashMap<ModuleName, Box> {
    let mut collect: Vec<Box> = DATA.lines().map(|line| {

            let mut iter = line.split("->");
            let name = iter.next().expect("must have a name").trim();
            let (t, name) = match name.chars().next().expect("name must have a char") {
                'b' => (ModuleType::Broadcast, name[0..].to_owned()),
                '%' => (ModuleType::FlipFlop(Pulse::Low), name[1..].to_owned()),
                '&' => (ModuleType::Conjunction(Conjunction{inputs: HashMap::new()}), name[1..].to_owned()),
                _ => {assert!(false); (ModuleType::Broadcast, "".to_owned())},
            };
            let out = iter.next().expect("must have output").split(",").map(|v|{
                ModuleName{str:v.trim().to_owned()}
            }).collect();
            Box{m: Module{name: ModuleName{str:name}, typ: t}, c: out}
        }
    ).collect();

    let clone_collect = collect.clone();
    //todo: link all Conjunction to its list of input
    for v in &mut collect {
        match &mut v.m.typ {
            ModuleType::Conjunction(input_list) => {
                for out in &clone_collect {
                    if out.c.contains(&v.m.name){
                        input_list.inputs.insert(out.m.name.clone(), Pulse::Low);
                    }
                }
            },
            _  => {},
        }
    }

    let mut map: HashMap<ModuleName, Box> = HashMap::new();
    for v in collect {
        println!("loaded {:?} of type {:?}", v.m.name, v);
        map.insert(v.m.name.clone(), v);
    }
    map
}

const DATA: &str = include_str!("../../../data/input.txt");
fn main() {
    let mut map: HashMap<ModuleName, Box> = load();
    let mut io: VecDeque<Signal> = VecDeque::new();
    let pulse = Pulse::Low;
    let button = ModuleName{str:"button".to_string()};
    let broadcaster = ModuleName{str:"broadcaster".to_string()};
    let mut s_low = 0;
    let mut s_high = 0;
    for i in 0..1000{
        io.push_back(Signal { p: pulse.clone(), dst: broadcaster.clone(), src: button.clone() });
        println!("\nButton press: {i}");
        while let Some(current_signal) = io.pop_front() {
            //println!("{:?} -{:?}->{:?}", current_signal.src.str, current_signal.p, current_signal.dst.str);
            match current_signal.p {
                Pulse::High => s_high+=1,
                Pulse::Low => s_low+=1,
            }
            if let Some(m) = map.get_mut(&current_signal.dst){
                let ris = m.pulse(&current_signal);
                io.extend(ris);
            }
        }
    }
    println!("s_low {s_low} s_high {s_high} = {}", s_low*s_high);
}
