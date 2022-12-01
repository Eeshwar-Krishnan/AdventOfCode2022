use std::fs::File;
use std::cmp;
use std::io::prelude::*;

pub fn problem1(mut file: File){
    let mut buf: String = "".to_string();
    file.read_to_string(&mut buf).unwrap();
    let mut tmp = 0;
    let mut max = 0;
    buf.split("\n").map(|val| {match(val.parse::<i32>()){Ok(v) => {tmp += v;} Err(v) => {max=cmp::max(max, tmp); tmp=0;}}}).for_each(drop);
    println!("{}", max);
}

pub fn problem2(mut file: File){
    let mut buf: String = "".to_string();
    file.read_to_string(&mut buf).unwrap();
    let mut tmp = 0;
    let mut ind = 0;
    let mut maxes = vec![0];
    buf.split("\n").map(|val| {match(val.parse::<i32>()){Ok(v) => {tmp += v;} Err(v) => {maxes.push(tmp); tmp=0;}}}).for_each(drop);
    maxes.sort(); maxes.reverse();
    maxes[0..3].iter().for_each(|v| {println!("{}", v)});
    println!("{}", maxes[0..3].iter().sum::<i32>());
}