use std::{fs::File, io::Read};

pub fn problem2(mut file: File){
    let mut buf: String = "".to_string();
    file.read_to_string(&mut buf).unwrap();

    let mut totalScore: i128 = 0;
    buf.split("\n").map(|v| {
        let mut num = 0;
        if(v.starts_with("A")){
            num += 1; //Rock
        }else if(v.starts_with("B")){
            num += 2; //Paper
        }else{
            num += 3; //Scissors
        }

        let mut choice = 0;
        let mut score = 0;
        if(v.ends_with("X")){
            choice += 1;
            if(num == 1){
                score += 3;
            }
            if(num == 2){
                score += 1;
            }
            if(num == 3){
                score += 2;
            }
        }else if(v.ends_with("Y")){
            choice += 2;
            score += 3;
            score += num;
        }else{
            choice += 3;
            score += 6;
            if(num == 1){
                score += 2;
            }
            if(num == 2){
                score += 3;
            }
            if(num == 3){
                score += 1;
            }
        }

        totalScore += score;

        println!("{}: {}", v, score);
    }).for_each(drop);

    println!("{}", totalScore);

    return;
}

pub fn problem1(mut file: File){
    let mut buf: String = "".to_string();
    file.read_to_string(&mut buf).unwrap();

    let mut totalScore: i128 = 0;
    buf.split("\n").map(|v| {
        let mut num = 0;
        if(v.starts_with("A")){
            num += 1; //Rock
        }else if(v.starts_with("B")){
            num += 2; //Paper
        }else{
            num += 3; //Scissors
        }

        let mut score = 0;
        if(v.ends_with("X")){
            score += 1;
        }else if(v.ends_with("Y")){
            score += 2;
        }else{
            score += 3;
        }

        if(num == score){
            score += 3;
        }
        if(score == 1 && num == 3){
            score += 6;
        }
        if(score == 2 && num == 1){
            score += 6;
        }
        if(score == 3 && num == 2){
            score += 6;
        }

        totalScore += score;

        println!("{}: {}", v, score);
    }).for_each(drop);

    println!("{}", totalScore);

    return;
}