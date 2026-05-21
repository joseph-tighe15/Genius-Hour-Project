use std::{fs};
use std::collections::HashMap;
use rand::prelude::*;
use console::{Term, Key};
use std::io::{self, Write};

fn clear_terminal() {
    // \x1B[2J: Clears the screen
    // \x1B[1;1H: Moves cursor to row 1, column 1
    print!("\x1B[2J\x1B[1;1H");
    println!("");
}
fn last_word(s: &str) -> &str {
    let words: Vec<&str>;
    if s.is_empty() || s.ends_with(" ") {
        words = Vec::new();
    } else {
        words = s.split_whitespace().collect();
    }
    words.last().unwrap_or(&"")
}
fn random_number() -> f64 {
    let mut rng = rand::rng(); // In rand 0.9+, use rand::rng()
    
    // Generate a float between 0.0 and 1.0
    rng.random()
}
fn last_n_chars<'a>(s: &'a str, n: usize) -> &'a str {
    if n == 0 {
        return "";
    }

    let mut start = s.len();
    let mut chars = s.char_indices().rev();
    for _ in 0..n {
        if let Some((idx, _)) = chars.next() {
            start = idx;
        } else {
            start = 0;
            break;
        }
    }
    &s[start..]
}

fn predict_next(before: &HashMap<char, HashMap<String, usize>>, context: &str, temperature: f64) -> char {
    let mut max_count = 0;
    let mut predicted_char = '@';
    let cleaned_context = last_n_chars(context, before.values().next().unwrap_or(&HashMap::new()).keys().next().unwrap_or(&"_-_-_".to_string()).len());

    for (&ch, counts) in before {
        if let Some(&count) = counts.get(cleaned_context) {
            if count as f64+random_number()*temperature > max_count as f64 && !(ch == ' ' && context.ends_with(" ")) {
                max_count = count;
                predicted_char = ch;
            }
        }
    }

    predicted_char
}
fn predict_next_with_fallback(before: &[HashMap<char, HashMap<String, usize>>], context: &str, temperature: f64) -> char {
    for i in (0..before.len()).rev() {
        if let Some(predicted_char) = before.get(i).map(|b| predict_next(b, context, temperature)) && predicted_char != '@' {
            return predicted_char;
        }
    }
    ' '
}
fn predict_next_word(before: &[HashMap<char, HashMap<String, usize>>], raw: &str, temperature: f64) -> String {
    let mut output = String::new();
    let mut context = raw.replace("\n", " ");
    context = remove_non_ascii(&context);
    context.retain(|c| !c.is_ascii_punctuation());
    context = context.to_lowercase();
    while context.contains("  ") {
        context = context.replace("  ", " ");
    }
    for _ in 0..500 {
        // Implementation for predicting the next word
        let next_char = predict_next_with_fallback(before, &(context.to_string() + &output), temperature);
        output.push(next_char);
        if next_char == ' ' {
            break
        }
    }
    output
}
fn predict_next_paragraph(before: &[HashMap<char, HashMap<String, usize>>], raw: &str, temperature: f64) -> String {
    let mut output = String::new();
    let mut context = raw.replace("\n", " ");
    context = remove_non_ascii(&context);
    context.retain(|c| !c.is_ascii_punctuation());
    context = context.to_lowercase();
    while context.contains("  ") {
        context = context.replace("  ", " ");
    }
    for _ in 0..500 {
        // Implementation for predicting the next word
        let next_char = predict_next_with_fallback(before, &(context.to_string() + &output), temperature);
        output.push(next_char);
    }
    output
}
fn remove_non_ascii(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii()).collect()
}
fn get_before_hash(content: &str, search_before: usize) -> HashMap<char, HashMap<String, usize>> {
    let mut before = HashMap::new();
    let chars: Vec<char> = content.chars().collect();
for i in search_before..chars.len() {
    let context: String = chars[i-search_before..i].iter().collect();
    let ch = chars[i];
    before.entry(ch)
          .or_insert_with(HashMap::new)
          .entry(context)
          .and_modify(|e| *e += 1)
          .or_insert(1);
}
    before
}
fn main() -> std::io::Result<()> {
    let entries = fs::read_dir("../text/")?;
    let mut before_list = Vec::new();
    println!("training stage started");
    for entry_res in entries {
        let entry = match entry_res {
            Ok(e) => e,
            Err(err) => {
                eprintln!("skipping unreadable directory entry: {}", err);
                continue;
            }
        };
        println!("starting training on {}", entry.file_name().to_string_lossy());
        let path = entry.path();
        
        // Filter to only process files (excluding subdirectories)
        if path.is_file() {
            let raw = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("skipping file {}: {}", path.display(), err);
                    continue;
                }
            };
            let mut content = raw.replace("\n", " ");
            content = remove_non_ascii(&content);
            content.retain(|c| !c.is_ascii_punctuation());
            content = content.to_lowercase();
            while content.contains("  ") {
                content = content.replace("  ", " ");
            }
            for i in 1..=5 {
                before_list.push(get_before_hash(&content, i));
            }
        }
    }
    println!("finished training");
    println!("before_list entries: {}", before_list.len());/*
    let mut text = String::from("which w");
    for _ in 0..500 {
        text.push(predict_next_with_fallback(&before_list, &text, 10.0));
    }
    println!("'{}'", text);*/
    let mut text = ": ".to_string();
    let stdout = Term::buffered_stdout();
    println!("press R or N");
    let mut real_mode : bool = false;
    if let Ok(key) = stdout.read_char() {
        if key == 'R' || key == 'r' {
            real_mode = true;
        }
    }
    print!("\x1b[?25l"); // Hide
    io::stdout().flush().unwrap();
    loop {
        let next;
        clear_terminal();
        if real_mode {
            next = predict_next_word(&before_list, &text, 0.0);
            let mut next2 = predict_next_word(&before_list, &text, 15.0);
            let mut next3 = predict_next_word(&before_list, &text, 50.0);
            for _ in 0..3 {
                if next2 == next {
                    next2 = predict_next_word(&before_list, &text, 15.0);
                }
            }
            for _ in 0..3 {
                if next3 == next || next3 == next2 {
                    next3 = predict_next_word(&before_list, &text, 50.0);
                }
            }
            print!("{}{}", last_word(&text), next);
            if next != next2 {
                print!("  {}{}", last_word(&text), next2);
            }
            if next3 != next2 && next3 != next {
                print!("  {}{}", last_word(&text), next3);
            } 
            println!("");
        } else {
            next = predict_next_paragraph(&before_list, &text, 15.0);
            
        println!("  {}{}", last_word(&text), next);
        }
        println!("{}█", text);


        match stdout.read_key()? {
            Key::Backspace => {text.pop();},
            Key::Tab => {
                text = format!("{}{}", text, next).to_string();
            },
            Key::Char(c) => {text.push(c);},
            Key::Escape => {
                println!("Exiting...");
                break;
            }
            _ => println!("Detected another key"),
        }
    }
    Ok(())
}
