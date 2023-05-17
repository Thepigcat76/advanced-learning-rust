use std::io::{self, BufRead};

fn main() {
    let mut friends: Vec<String> = vec!["uwu".to_string(), "owo".to_string(), "#w#".to_string()];
    loop {
        println!("Enter a new name:");
        let input = scan();
        if input.is_empty() {
            break; // Break the loop if an empty input is provided
        }
        friends.push(input);
        for friend in &friends {
            println!("{}", friend);
        }
    }
}

fn scan() -> String {
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed_input = input.trim().to_owned();
    return trimmed_input;
}