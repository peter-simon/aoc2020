use std::{vec};
use day1::{LevelRegister, NumT, update_cache}; 
use itertools::Itertools;

extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Day1").version("1.0")
        .arg(Arg::with_name("file")
            .long("file")
            .short("f")
            .default_value("input.txt"))
        .arg(Arg::with_name("magicnum")
            .long("num")
            .short("n")
            .default_value("2020"))
        .arg(Arg::with_name("members")
            .long("member")
            .short("m")
            .default_value("2")).get_matches();

    let KEY_NUM = clap::value_t!(matches.value_of("magicnum"), NumT).unwrap();
    let LEVELS = clap::value_t!(matches.value_of("members"), u8).unwrap();
    let filename = matches.value_of("file").unwrap();

    let text = day1::read_from_file(&filename);
    let text_it = text.split_whitespace();

    let all_nums_iter: Vec<i64> = text_it.map(|x|x.parse::<NumT>().unwrap()).collect();

    let mut register: Vec<LevelRegister> = vec![LevelRegister::new(); LEVELS as usize - 1];

    for num in all_nums_iter.iter() {
        let tmp_register = &mut register;
        update_cache(num, tmp_register); // .> does not work mutable borrow in a loop :()
        let found = day1::evaluate(KEY_NUM, num, &tmp_register.last().unwrap());
        if let Some(y) = found {
            for x in y {
                let prod = x.iter().fold(1, |prod, i| prod * *i);
                println!("Numbers: {:?}, result: {}", x, prod);
                //return;
            }
        }
    }
}

fn main2() {
    let text = day1::read_from_file("input.txt");
    let text_it = text.split_whitespace();

    let all_nums_iter= text_it.map(|x|x.parse::<NumT>().unwrap());
    
    let LEVELS: u8 = 5;
    let KEY_NUM = 2020i64;
    for c in all_nums_iter.combinations(LEVELS as usize - 1) {
        if c.iter().sum::<i64>() == KEY_NUM {
            let prod = c.iter().fold(1, |prod, i| prod * *i);
            println!("{:?}: {}", c, prod)
        }
    }
}