use std::fs;
use std::collections::HashMap;
use rand::prelude::*;

fn random_number() -> f64 {
    let mut rng = rand::rng(); // In rand 0.9+, use rand::rng()
    
    // Generate a float between 0.0 and 1.0
    rng.random()
}
fn last_n_chars(s: &str, n: usize) -> String {
    s.chars().rev().take(n).collect::<Vec<_>>().into_iter().rev().collect()
}

fn predict_next(before: &HashMap<char, HashMap<String, usize>>, context: &str, temperature: f64) -> char {
    let mut max_count = 0;
    let mut predicted_char = '@';
    let cleaned_context = last_n_chars(context, before.values().next().unwrap_or(&HashMap::new()).keys().next().unwrap_or(&"_-_-_".to_string()).len());

    for (&ch, counts) in before {
        if let Some(&count) = counts.get(&cleaned_context) {
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
    for entry in entries {
        let entry = entry?;
        println!("starting training on {}", entry.file_name().to_string_lossy());
        let path = entry.path();
        
        // Filter to only print files (excluding subdirectories)
        if path.is_file() {
            let mut content = fs::read_to_string(&path)?.replace("\n", " ");
            content = remove_non_ascii(&content);
            content = content.replace(".", ". ");
            while content.contains("  ") {
                content = content.replace("  ", " ");
            }
            for i in 1..=5 {
                before_list.push(get_before_hash(&content, i));
            }
        }
    }
    println!("finished training");
    let mut text = String::from("which w");
    for _ in 0..500 {
        text.push(predict_next_with_fallback(&before_list, &text, 10.0));
    }
    println!("'{}'", text);
    Ok(())
}
