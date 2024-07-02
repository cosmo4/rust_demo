use std::env;
use std::fs::File;

use std::io::{self, BufRead, BufReader, Write};


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();


    if args.len() != 2 {
        eprintln!("Usage: text_analyzer <file_path>");
        std::process::exit(1);
    }


    let file_path = &args[1];


    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file '{}': {}", file_path, error);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);


    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;


    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        char_count += line.chars().count();
        word_count += line.split_whitespace().count();
    }


    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);


    let mut input = String::new();
    print!("Would you like to count a word or a character? (word/char): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();


    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    match input.as_str() {
        "word" => {
            let mut word = String::new();
            print!("Enter the word to count: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut word)?;
            let word = word.trim();

            let mut word_occurrences = 0;
            for line in reader.lines() {
                let line = line?;
                word_occurrences += line.split_whitespace().filter(|&w| w == word).count();
            }

            println!("The word '{}' appears {} times in the text.", word, word_occurrences);
        }
        "char" => {
            let mut character = String::new();
            print!("Enter the character to count: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut character)?;
            let character = character.trim().chars().next().unwrap();

            let mut char_occurrences = 0;

            for line in reader.lines() {
                let line = line?;
                
                char_occurrences += line.chars().filter(|&c| c == character).count();
            }

            println!("The character '{}' appears {} times in the text.", character, char_occurrences);

        }
        _ => {

            eprintln!("Invalid choice. Please enter 'word' or 'char'.");
        }
    }

    Ok(())
}
