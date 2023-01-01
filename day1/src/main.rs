use std::fmt::format;

// Date: 2021-09-13
// Use Tokio read a text file
#[allow(unused_imports, unused_variables, dead_code, unused_mut)]
use tokio::{fs::File, io::AsyncReadExt};

// struct Elf
#[derive(Debug, Clone)]
struct Elf {
    name: String,
    calories: i32,
}

#[derive(Debug, Clone)]
struct Elves(Vec<Elf>);
impl Elves {
    fn new() -> Elves {
        Elves(Vec::new())
    }
    fn add(&mut self, elf: Elf) {
        self.0.push(elf);
    }
    fn get(&self, index: usize) -> &Elf {
        &self.0[index]
    }
    fn len(&self) -> usize {
        self.0.len()
    }

    // #[must_use]
    // pub fn is_empty(&self) -> bool {
    //     self.len() == 0
    // }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("src/data/input.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let lines = contents.lines();
    // loop over lines
    let mut elves = Elves::new();
    let mut calories_total = 0;
    let mut elf_number = 1;
    let elf = Elf {
        name: format!("Elf {}", elf_number.to_string()).to_string(),
        calories: 0,
    };
    for line in lines {
        if line.trim().len() == 0 {
            let elf = Elf {
                name: format!("Elf {}", elf_number.to_string()).to_string(),
                calories: calories_total,
            };
            calories_total = 0;
            continue;
        }

        if line.trim().len() > 0 {
            calories_total += line.trim().parse::<i32>().unwrap();
        }
        // println!("Line: {}, elf {}, elf calories : {}", line.trim(), elf_number, elf.calories);


    }
println!("Elf: {:?}", elves);
    Ok(())
}

