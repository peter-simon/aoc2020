use std::fs;

#[derive(Debug)]
struct Seat {
    row: u16,
    seat: u16,
}

impl Seat {
    fn new(row: u16, seat: u16) -> Seat {
        Seat { row, seat }
    }

    fn seat_id(&self) -> u32 {
        self.row as u32 * 8 + self.seat as u32
    }
}

fn decode(code: &str, one: &char, zero: &char) -> u16 {
    //println!("one: {}", one);
    //println!("zero: {}", zero);
    let mut out: u16 = 0;
    let len = code.len();
    for (i, c) in code.chars().enumerate() {
        //println!("c:{}", c);
        let v = match c {
            _ if c == *one => 1,
            _ if c == *zero => 0,
            _ => panic!(),
        };
        out += 2u16.pow((len - i - 1) as u32) * v;
    }
    //println!("decoded from: {}, value: {}", code, out);
    out
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_it = content.split("\r\n");

    let mut seats: Vec<Seat> = content_it
        .map(|x| Seat::new(decode(&x[0..7], &'B', &'F'), decode(&x[7..], &'R', &'L')))
        .collect();

    let max_id = seats.iter().map(|x| x.seat_id()).max().unwrap();

    seats.sort_by(|a, b| a.seat_id().cmp(&b.seat_id()));
    let sorted_ids = seats.iter().map(|x| x.seat_id());
    let mut seat_holes = seats
        .iter()
        .zip(seats.iter().skip(1))
        .filter(|(curr, next)| next.seat_id() - curr.seat_id() > 1);
    println!("{:?}", seat_holes.next().unwrap().1.seat_id()-1);
}
