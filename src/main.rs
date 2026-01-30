use clap::{Parser, Subcommand};
use colored::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "isaw")]
#[command(about = "Generate letter combinations and search for words/sentences", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate all permutations of given letters
    Permutations {
        /// Letters to permute (e.g., "abc")
        letters: String,
        
        /// Minimum length of combinations
        #[arg(short, long, default_value = "1")]
        min: usize,
        
        /// Maximum length of combinations (defaults to letter count)
        #[arg(short = 'x', long)]
        max: Option<usize>,
        
        /// Search for specific word/sentence in results
        #[arg(short, long)]
        search: Option<String>,
        
        /// Case insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },
    
    /// Generate all combinations (order doesn't matter)
    Combinations {
        /// Letters to combine (e.g., "abc")
        letters: String,
        
        /// Length of combinations
        #[arg(short, long, default_value = "2")]
        length: usize,
        
        /// Search for specific pattern in results
        #[arg(short, long)]
        search: Option<String>,
        
        /// Case insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },
    
    /// Generate all possible words from letters (like Scrabble)
    Words {
        /// Available letters (e.g., "abcde")
        letters: String,
        
        /// Minimum word length
        #[arg(short, long, default_value = "2")]
        min: usize,
        
        /// Maximum word length (defaults to letter count)
        #[arg(short = 'x', long)]
        max: Option<usize>,
        
        /// Search for specific pattern
        #[arg(short, long)]
        search: Option<String>,
        
        /// Only show unique combinations
        #[arg(short, long)]
        unique: bool,
    },
    
    /// Search through custom alphabet combinations
    Search {
        /// Pattern to search for
        pattern: String,
        
        /// Letters/alphabet to use (defaults to a-z)
        #[arg(short, long)]
        letters: Option<String>,
        
        /// Length of combinations to search
        #[arg(short = 'n', long, default_value = "3")]
        length: usize,
        
        /// Case insensitive search
        #[arg(short, long)]
        ignore_case: bool,
        
        /// Use regex pattern
        #[arg(short, long)]
        regex: bool,
    },
    
    /// Count total combinations possible
    Count {
        /// Letters available
        letters: String,
        
        /// Minimum length
        #[arg(short, long, default_value = "1")]
        min: usize,
        
        /// Maximum length
        #[arg(short = 'x', long)]
        max: Option<usize>,
        
        /// Count combinations instead of permutations
        #[arg(short, long)]
        combinations: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Permutations { letters, min, max, search, ignore_case } => {
            generate_permutations(&letters, min, max, search, ignore_case);
        }
        Commands::Combinations { letters, length, search, ignore_case } => {
            generate_combinations(&letters, length, search, ignore_case);
        }
        Commands::Words { letters, min, max, search, unique } => {
            generate_words(&letters, min, max, search, unique);
        }
        Commands::Search { pattern, letters, length, ignore_case, regex } => {
            search_combinations(&pattern, letters, length, ignore_case, regex);
        }
        Commands::Count { letters, min, max, combinations } => {
            count_combinations(&letters, min, max, combinations);
        }
    }
}

fn generate_permutations(letters: &str, min: usize, max: Option<usize>, search: Option<String>, ignore_case: bool) {
    let chars: Vec<char> = letters.chars().collect();
    let max_len = max.unwrap_or(chars.len());
    
    println!("{}", format!("📝 Generating permutations of '{}' (length {} to {})", letters, min, max_len).cyan().bold());
    println!("{}", "─".repeat(50).dimmed());
    
    let mut count = 0;
    let mut matches = 0;
    
    for len in min..=max_len {
        for perm in chars.iter().permutations(len) {
            let word: String = perm.into_iter().collect();
            count += 1;
            
            if let Some(ref search_term) = search {
                let (word_check, term_check) = if ignore_case {
                    (word.to_lowercase(), search_term.to_lowercase())
                } else {
                    (word.clone(), search_term.clone())
                };
                
                if word_check.contains(&term_check) {
                    matches += 1;
                    print_highlighted(&word, search_term, ignore_case);
                }
            } else {
                println!("  {}", word);
            }
        }
    }
    
    println!("{}", "─".repeat(50).dimmed());
    if search.is_some() {
        println!("{}", format!("✨ Found {} matches out of {} permutations", matches, count).green().bold());
    } else {
        println!("{}", format!("✨ Generated {} permutations", count).green().bold());
    }
}

fn generate_combinations(letters: &str, length: usize, search: Option<String>, ignore_case: bool) {
    let chars: Vec<char> = letters.chars().collect();
    
    println!("{}", format!("🔤 Generating combinations of '{}' (length {})", letters, length).cyan().bold());
    println!("{}", "─".repeat(50).dimmed());
    
    let mut count = 0;
    let mut matches = 0;
    
    for combo in chars.iter().combinations(length) {
        let word: String = combo.into_iter().collect();
        count += 1;
        
        if let Some(ref search_term) = search {
            let (word_check, term_check) = if ignore_case {
                (word.to_lowercase(), search_term.to_lowercase())
            } else {
                (word.clone(), search_term.clone())
            };
            
            if word_check.contains(&term_check) {
                matches += 1;
                print_highlighted(&word, search_term, ignore_case);
            }
        } else {
            println!("  {}", word);
        }
    }
    
    println!("{}", "─".repeat(50).dimmed());
    if search.is_some() {
        println!("{}", format!("✨ Found {} matches out of {} combinations", matches, count).green().bold());
    } else {
        println!("{}", format!("✨ Generated {} combinations", count).green().bold());
    }
}

fn generate_words(letters: &str, min: usize, max: Option<usize>, search: Option<String>, unique: bool) {
    let chars: Vec<char> = letters.chars().collect();
    let max_len = max.unwrap_or(chars.len());
    
    println!("{}", format!("📖 Generating word combinations from '{}' (length {} to {})", letters, min, max_len).cyan().bold());
    println!("{}", "─".repeat(50).dimmed());
    
    let mut seen: HashSet<String> = HashSet::new();
    let mut count = 0;
    let mut matches = 0;
    
    for len in min..=max_len {
        for perm in chars.iter().permutations(len) {
            let word: String = perm.into_iter().collect();
            
            if unique {
                if seen.contains(&word) {
                    continue;
                }
                seen.insert(word.clone());
            }
            
            count += 1;
            
            if let Some(ref search_term) = search {
                if word.to_lowercase().contains(&search_term.to_lowercase()) {
                    matches += 1;
                    print_highlighted(&word, search_term, true);
                }
            } else {
                println!("  {}", word);
            }
        }
    }
    
    println!("{}", "─".repeat(50).dimmed());
    if search.is_some() {
        println!("{}", format!("✨ Found {} matches out of {} words", matches, count).green().bold());
    } else {
        println!("{}", format!("✨ Generated {} words", count).green().bold());
    }
}

fn search_combinations(pattern: &str, letters: Option<String>, length: usize, ignore_case: bool, regex: bool) {
    let alphabet = letters.unwrap_or_else(|| "abcdefghijklmnopqrstuvwxyz".to_string());
    let chars: Vec<char> = alphabet.chars().collect();
    
    println!("{}", format!("🔍 Searching for '{}' in {}-letter combinations", pattern, length).cyan().bold());
    println!("{}", format!("   Using alphabet: {}", alphabet).dimmed());
    println!("{}", "─".repeat(50).dimmed());
    
    let results: Vec<String> = if regex {
        let re = regex_lite::Regex::new(pattern).unwrap_or_else(|e| {
            eprintln!("{}", format!("Invalid regex: {}", e).red());
            std::process::exit(1);
        });
        
        generate_all_combinations(&chars, length)
            .into_par_iter()
            .filter(|word| re.is_match(word))
            .collect()
    } else {
        let search_pattern = if ignore_case { pattern.to_lowercase() } else { pattern.to_string() };
        
        generate_all_combinations(&chars, length)
            .into_par_iter()
            .filter(|word| {
                let w = if ignore_case { word.to_lowercase() } else { word.clone() };
                w.contains(&search_pattern)
            })
            .collect()
    };
    
    for word in &results {
        print_highlighted(word, pattern, ignore_case);
    }
    
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", format!("✨ Found {} matches", results.len()).green().bold());
}

fn generate_all_combinations(chars: &[char], length: usize) -> Vec<String> {
    if length == 0 {
        return vec![String::new()];
    }
    
    let mut results = Vec::new();
    
    fn helper(chars: &[char], current: String, length: usize, results: &mut Vec<String>) {
        if current.len() == length {
            results.push(current);
            return;
        }
        
        for &c in chars {
            let mut next = current.clone();
            next.push(c);
            helper(chars, next, length, results);
        }
    }
    
    helper(chars, String::new(), length, &mut results);
    results
}

fn count_combinations(letters: &str, min: usize, max: Option<usize>, combinations: bool) {
    let n = letters.chars().count();
    let max_len = max.unwrap_or(n);
    
    println!("{}", format!("📊 Counting {} for '{}'", 
        if combinations { "combinations" } else { "permutations" },
        letters
    ).cyan().bold());
    println!("{}", "─".repeat(50).dimmed());
    
    let mut total: u128 = 0;
    
    for k in min..=max_len {
        let count = if combinations {
            // C(n, k) = n! / (k! * (n-k)!)
            binomial(n as u128, k as u128)
        } else {
            // P(n, k) = n! / (n-k)!
            permutation(n as u128, k as u128)
        };
        
        println!("  Length {}: {}", format!("{}", k).yellow(), format!("{}", count).white().bold());
        total += count;
    }
    
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", format!("✨ Total: {}", total).green().bold());
}

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn binomial(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn permutation(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    factorial(n) / factorial(n - k)
}

fn print_highlighted(word: &str, pattern: &str, ignore_case: bool) {
    if ignore_case {
        let lower_word = word.to_lowercase();
        let lower_pattern = pattern.to_lowercase();
        
        if let Some(pos) = lower_word.find(&lower_pattern) {
            let before = &word[..pos];
            let matched = &word[pos..pos + pattern.len()];
            let after = &word[pos + pattern.len()..];
            println!("  {}{}{}", before, matched.yellow().bold(), after);
        } else {
            println!("  {}", word);
        }
    } else if let Some(pos) = word.find(pattern) {
        let before = &word[..pos];
        let matched = &word[pos..pos + pattern.len()];
        let after = &word[pos + pattern.len()..];
        println!("  {}{}{}", before, matched.yellow().bold(), after);
    } else {
        println!("  {}", word);
    }
}
