#[allow(unused_imports, unused_variables, dead_code, unused_mut)]


const ROCK: usize =1;
const PAPER: usize =2;
const SCISSORS: usize =3;

const WIN: usize =6;
const LOSE: usize =0;
const DRAW: usize =3;
const A: usize = ROCK;
// const B = 2;
// const C = 3;
// const X = 1;
const Y : usize = PAPER;
const Z : usize = SCISSORS;

fn calculate(a: usize, b: usize) -> usize {
    let mut score = 0;
    if a == b {
        score = DRAW + b;
    } else if a == ROCK && b == SCISSORS {
        score = LOSE  + b;
    } else if a == ROCK && b == PAPER {
        score = WIN + b;
    } else if a == PAPER && b == ROCK {
        score = LOSE + b;
    } else if a == PAPER && b == SCISSORS {
        score = WIN + b;
    } else if a == SCISSORS && b == PAPER {
        score = LOSE + b;
    } else if a == SCISSORS && b == ROCK {
        score = WIN+ b;
    }
    score
}
fn letter_to_number(letter: &str) -> usize {
    let mut result = 0;
    if letter == "A" || letter == "X"{
        result = ROCK;
    } else if letter == "B" || letter == "Y"{
        result = PAPER;
    } else if letter == "C" || letter == "Z"{
        result = SCISSORS;
    }
    result

}

fn convert_to_string(score: usize) -> String {
    let mut result = String::new();
    if score == 6 {
        result = "Win".to_string();
    } else if score == 0 {
        result = "Lose".to_string();
    } else if score == 3 {
        result = "Draw".to_string();
    }
    result
}

fn main() {
    let draw1 = "A,X";
    let val1 =draw1.split(',').next().unwrap().trim();
    let val2 =draw1.split(',').nth(1).unwrap().trim();
   let num1 = letter_to_number(val1);
   let num2 = letter_to_number(val2);
    let score = calculate(num1, num2);
    println!("score = {}", score);
    // println!("{} vs {}" , val1, val2.trim());
    // println!("{} vs {}" , num1, num2);
    // println!("{} vs {}" , convert_to_string(num1), convert_to_string(num2));
    // println!("{} vs {}" , convert_to_string(score), score);
    // let draw2 = "B,Z";
    // let val1 =draw2.split(",").nth(0).unwrap().trim();
    // let val2 =draw2.split(",").nth(1).unwrap().trim();

    // println!("{} vs {}" , val1, val2.trim());


}
