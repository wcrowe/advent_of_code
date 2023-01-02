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
        score = Win  + b;
    } else if a == Rock && b == Paper {
        score = Lose + b;
    } else if a == Paper && b == Rock {
        score = Win + b;
    } else if a == Paper && b == Scissors {
        score = Lose + b;
    } else if a == Scissors && b == Paper {
        score = Win + b;
    } else if a == Scissors && b == Rock {
        score = Lose+ b;
    }
    score
}

fn main() {
    let draw1 = "A,Y";
    let val1 =draw1.split(",").nth(0).unwrap().trim();
    let val2 =draw1.split(",").nth(1).unwrap().trim();
   let num1 = i32.parse(val1).unwrap();
   let num2 = i32.parse(val2).unwrap();
   //d let score = calculate(val1, val2);
    println!("{} vs {}" , val1, val2.trim());


}
