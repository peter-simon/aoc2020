use std::fs;
use std::io::prelude::*;

struct RingBuffRead<T> {
    data: Vec<T>,
}

struct Stepper {
    pos: usize,
    size: usize
}

impl Stepper {
    fn new(size: usize) -> Stepper {            
        println!("size: {}", size);
        Stepper {pos: 0, size}
    }

    fn step(&mut self, n: &usize) -> usize {
        self.pos += n;
        while self.pos >= self.size {
            self.pos -= self.size;
        }
        return self.pos
    }
}

impl<T> RingBuffRead<T> {
    fn new(data: Vec<T>) -> RingBuffRead<T> {
        RingBuffRead {data}
    }
}

fn is_tree_write(c: &mut char) -> bool {
    if c == &'#' {
        *c = 'X';
        return true;
    }
    else {
        *c = 'O';
        return false;
    }
}

fn is_tree(c: &char) -> bool {
    c == &'#'
}

fn save_route(picture: &Vec<RingBuffRead<char>>) {
    let mut f = fs::OpenOptions::new().write(true)
                             .create_new(true)
                             .open("route.txt").unwrap();

    for p in picture {
        let line = String::from(p.data.iter().map(|x| x.to_string()).collect::<String>());
        write!(f, "{}\n", &line).unwrap();
    }

}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let content_it = content.split("\r\n");

    let right = 1 as usize;
    let down = 2 as usize;

    let mut picture: Vec<RingBuffRead<char>> = Vec::new();

    for line in content_it {
        picture.push(RingBuffRead::new(line.chars().collect()))
    }

    let mut stepper = Stepper::new(picture[0].data.len());
    let mut trees = 0u32;
    let mut row_skip = 0u32;
    
    for p in &mut picture[1..] {
        if down > row_skip as usize + 1 {
            row_skip += 1;
            continue;
        } else {
            row_skip = 0
        }
        let pos = stepper.step(&right);
        println!("{}",pos);
        let c = p.data.get_mut(pos).unwrap();
        trees += is_tree_write(c) as u32;
    }
    
    save_route(&picture);
    println!("trees: {}", trees);
}
