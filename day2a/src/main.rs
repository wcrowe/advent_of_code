#[allow(unused_imports, unused_variables, dead_code, unused_mut)]


const Rock: usize =1;
const Paper: usize =2;
const Scissors: usize =3;

const Win: usize =6;
const Lose: usize =0;
const Draw: usize =3;
const A: usize = Rock;
// const B = 2;
// const C = 3;
// const X = 1;
const Y : usize = Paper;
const Z : usize = Scissors;

fn calculate(a: usize, b: usize) -> usize {
    let mut score = 0;
    if a == b {
        score = Draw + b;
    } else if a == Rock && b == Scissors {
        score = Lose  + b;
    } else if a == Rock && b == Paper {
        score = Win + b;
    } else if a == Paper && b == Rock {
        score = Lose + b;
    } else if a == Paper && b == Scissors {
        score = Win + b;
    } else if a == Scissors && b == Paper {
        score = Lose + b;
    } else if a == Scissors && b == Rock {
        score = Win+ b;
    }
    score
}
fn letter_to_number(letter: &str) -> usize {
    let mut result = 0;
    if letter == "A" || letter == "X"{
        result = Rock;
    } else if letter == "B" || letter == "Y"{
        result = Paper;
    } else if letter == "C" || letter == "Z"{
        result = Scissors;
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
    let val1 =draw1.split(",").nth(0).unwrap().trim();
    let val2 =draw1.split(",").nth(1).unwrap().trim();
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
