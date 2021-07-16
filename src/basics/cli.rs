// CLI

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[0].clone();
    println!("Command: {}", command);
}
