// Date: 2021-09-13
// Use Tokio read a text file

use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("src/data/input.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let mut lines = contents.lines();
    println!("File contents: {}", lines.next().unwrap());
    Ok(())
}
