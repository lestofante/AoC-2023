struct Ticket{
    winning: Vec<i32>,
    extracted: Vec<i32>,
}

fn load() -> Vec<Ticket>{
    let input_file = include_str!("../../../data/input.txt");

    let mut tickets :Vec<Ticket> = vec!();
    tickets.push(Ticket{
        winning: vec!(),
        extracted: vec!(),
    });
    for line in input_file.lines() {
        let title: Vec<&str> = line.split(':').collect();
        assert!(title.len() == 2);
        let lists: Vec<&str> = title[1].split('|').collect();
        assert!(lists.len() == 2);
        let winning: Vec<i32> = lists[0].split_whitespace().map(|s| s.parse().expect("parse error")).collect();
        println!("winning {winning:?}");
        let extracted: Vec<i32> = lists[1].split_whitespace().map(|s| s.parse().expect("parse error")).collect();
        println!("extracted {extracted:?}");
        tickets.push(Ticket{
            winning,
            extracted,
        });
    }
    tickets
}

fn main() {
    let tickets = load();
    let mut my_tickets: Vec<usize> = vec!();
    for i in 1..tickets.len(){
        my_tickets.push(i);
    }

    let mut index = 0;
    while index < my_tickets.len(){
        let ticket_id = my_tickets[index];
        let mut won = 0;
        for ext in &tickets[ticket_id].extracted{
            if tickets[ticket_id].winning.contains(&ext){
                won += 1;
            }
        }
        println!("ticket {index} is id {ticket_id} and won {won} tickets");
        for i in 0..won{
            my_tickets.push(ticket_id+i+1);
        }
        println!("parital {won:?}");
        index += 1;
    }
    println!("won {}", my_tickets.len());
}
