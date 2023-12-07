use std::cmp::Ordering;

const SIZE:usize = 1000;

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, PartialOrd)]
enum Cards{
    INVALID,
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl Cards {
    fn from(c: char) -> Cards{
        match c {
            '2' => Cards::_2,
            '3' => Cards::_3,
            '4' => Cards::_4,
            '5' => Cards::_5,
            '6' => Cards::_6,
            '7' => Cards::_7,
            '8' => Cards::_8,
            '9' => Cards::_9,
            'T' => Cards::T,
            'J' => Cards::J,
            'Q' => Cards::Q,
            'K' => Cards::K,
            'A' => Cards::A,
            _ => Cards::INVALID,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Data{
    cards: [Cards; 5],
    bid: usize,
}

impl Default for Data {
    fn default() -> Self {
        Self { cards: [Cards::_2; 5], bid: 0 }
    }
}

fn parse(section: &str) -> [Data;SIZE] {
    let mut data: [Data;SIZE] = [Default::default();SIZE];

    let mut section = section.split_whitespace();
    for i in 0..data.len(){

        let mut cards_str = section.next().unwrap().chars();
        data[i].bid = section.next().unwrap().parse().unwrap();

        for x in 0..data[i].cards.len(){
            data[i].cards[x] = Cards::from(cards_str.next().unwrap());
        }
    }
    assert!(section.next().is_none(), "file should be empty");
    data
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, PartialOrd)]
enum Type{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, Debug)]
struct Pair{
    char: Cards,
    num: usize,
}

impl Default for Pair {
    fn default() -> Self {
        Self { char: Cards::INVALID, num: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hand{
    t: Type,
    d: Data,
}

impl Default for Hand{
    fn default() -> Self {
        Self { t: Type::HighCard, d: Default::default() }
    }
}

fn compare(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    if a.t > b.t{
        return Ordering::Greater;
    }
    if b.t > a.t{
        return Ordering::Less;
    }
    for i in 0..5{
        if a.d.cards[i] > b.d.cards[i] {
            return Ordering::Greater;
        }
        if b.d.cards[i] > a.d.cards[i] {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn main() {
    let input_file_sections = include_str!("../../../data/input.txt");
    let data = parse(input_file_sections);

    let mut hands: [Hand; SIZE] = [Default::default(); SIZE];

    for i in 0..data.len(){
        let d = &data[i];
        let mut pair: [Pair;5] = Default::default();
        let mut groups = 0;
        let mut max_pair = 0;
        let mut jolly = 0;
        for c in d.cards{
            for p in &mut pair{
                if c == Cards::J{
                    jolly+=1;
                    break;
                }
                if p.char == Cards::INVALID{
                    p.char = c;
                    p.num = 0;
                    groups += 1;
                    break;
                }
                if p.char == c{
                    p.num += 1;
                    if p.num > max_pair{
                        max_pair = p.num;
                    }
                    break;
                }
            }
        }
        max_pair += jolly;

        hands[i] = Hand{t: match groups {
            1 => Type::FiveOfAKind,
            2 => 
            if max_pair == 3 {
                Type::FourOfAKind
            }else{
                Type::FullHouse
            },
            
            3 => 
            if max_pair == 2{
                Type::ThreeOfAKind
            }else{
                Type::TwoPair
            },
            4 => Type::OnePair,
            0 => Type::FiveOfAKind, // all Jollly
            _ => Type::HighCard,
        }, d: *d};
    }

    hands.sort_by(|a, b|compare(a, b));

    let mut sum_win = 0;
    for i in 0..hands.len(){
        sum_win += hands[i].d.bid * (i+1);
    }
    println!("hands {hands:#?}");
    println!("sum_win {sum_win}");

}
