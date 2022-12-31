// Date: 2021-09-13
// Use Tokio read a text file

use tokio::{fs::File, io::AsyncReadExt};

// struct Elf
#[derive(Debug)]
struct Elf {
    name: String,
    calories: i32,
} 
struct Elves {
    elves: Vec<Elf>,
}
impl Elves {
    fn new() -> Elves {
        Elves { elves: Vec::new() }
    }
    fn add(&mut self, elf: Elf) {
        self.elves.push(elf);
    }
    fn get(&self, index: usize) -> &Elf {
        &self.elves[index]
    }
    fn len(&self) -> usize {
        self.elves.len()
    }
}
    
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("src/data/input.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let mut lines = contents.lines();
    // loop over lines
    for line in lines {
        println!("Line: {}", line.trim());

    }

    Ok(())
}
