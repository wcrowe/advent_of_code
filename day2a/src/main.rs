#[allow(unused_imports, unused_variables, dead_code, unused_mut)]
#[derive(Debug, Clone)]

enum Shapes {
    Rock = 0u8,
    Paper = 1u8,
    Scissors = 2u8,
}
enum Result {
    Win = 6u8,
    Lose = 1u8,
    Draw = 3u8,
}
enum Opponent {
    A = shapes::Rock,
    B = shapes::Paper,
    C = shapes::Scissors,
}
enum Mine {
    X = shapes::Rock,
    Y = shapes::Paper,
    Z = shapes::Scissors,
}

fn main() {
    let draw1 = (opponent::A, mine::Y);
    // let draw2 = (B,X);
    // let draw3 = (C,Z);
    // match draw1 {
    //     (draw1.0 > draw1.1) => println!("Win"),
    //     (draw1.0 < draw1.1) => println!("Loss"),
    //     (draw1.0 = draw1.1) => println!("Draw"),
    // }
    println!("Hello, world!");
}
