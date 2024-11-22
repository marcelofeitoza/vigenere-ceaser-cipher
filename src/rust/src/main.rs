use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::path::Path;

use csv::Writer;
use rand::Rng;

fn main() {
    let input_dir = Path::new("../../data/input");
    let output_dir = Path::new("../../data");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let ciphers = read_lines(&input_dir.join("criptogramas.txt").to_string_lossy());
    let dictionary_words = read_lines(&input_dir.join("dicionario.txt").to_string_lossy());
    let dictionary: HashSet<String> = dictionary_words.into_iter().collect();

    let mut wtr_caesar =
        Writer::from_path(output_dir.join("caesar.csv")).expect("Unable to create CSV file");
    let mut wtr_substitution =
        Writer::from_path(output_dir.join("substitution.csv")).expect("Unable to create CSV file");
    let mut wtr_vigenere =
        Writer::from_path(output_dir.join("vigenere.csv")).expect("Unable to create CSV file");

    // Write headers
    for wtr in [&mut wtr_caesar, &mut wtr_substitution, &mut wtr_vigenere].iter_mut() {
        wtr.write_record(["Cipher Text", "Key", "Decrypted Text", "Score"])
            .expect("Unable to write header");
    }

    // Process each cipher with all methods
    for cipher in &ciphers {
        decrypt_caesar_cipher(cipher, &mut wtr_caesar);
        decrypt_substitution_cipher(cipher, &mut wtr_substitution);
        decrypt_vigenere_cipher(cipher, &mut wtr_vigenere);
    }

    // Flush all writers
    for wtr in [&mut wtr_caesar, &mut wtr_substitution, &mut wtr_vigenere].iter_mut() {
        wtr.flush().expect("Unable to flush CSV writer");
    }
}

// Caesar Cipher Decryption
fn decrypt_caesar_cipher(cipher: &str, wtr: &mut Writer<std::fs::File>) {
    for shift in 1..26 {
        let decrypted = caesar_decrypt(cipher, shift);
        let score = calculate_ngram_score(&decrypted);
        wtr.write_record(&[
            cipher.to_string(),
            shift.to_string(),
            decrypted,
            score.to_string(),
        ])
        .expect("Unable to write record");
    }
}

fn caesar_decrypt(text: &str, shift: u8) -> String {
    let mut decrypted = String::new();
    for c in text.chars() {
        if c.is_ascii_uppercase() {
            let c_index = c as u8 - b'A';
            let decrypted_c = ((26 + c_index - shift) % 26) + b'A';
            decrypted.push(decrypted_c as char);
        } else {
            decrypted.push(c);
        }
    }
    decrypted
}

// Substitution Cipher Decryption
fn decrypt_substitution_cipher(cipher_text: &str, wtr: &mut Writer<std::fs::File>) {
    let cipher_freq = letter_frequencies(cipher_text);
    let mut current_mapping = initial_mapping(&cipher_freq);
    let mut current_decrypted = apply_mapping(cipher_text, &current_mapping);
    let mut current_score = calculate_ngram_score(&current_decrypted);

    let max_iterations = 10000;
    let mut iterations = 0;
    let mut temperature = 100.0;
    let cooling_rate = 0.0005;

    while iterations < max_iterations && temperature > 0.1 {
        let neighbor = generate_neighbor(&current_mapping);
        let decrypted = apply_mapping(cipher_text, &neighbor);
        let score = calculate_ngram_score(&decrypted);
        let delta = score - current_score;

        // Write the current mapping and decrypted text to CSV
        let key = mapping_to_string(&neighbor);
        wtr.write_record(&[
            cipher_text.to_string(),
            key,
            decrypted.clone(),
            score.to_string(),
        ])
        .expect("Unable to write record");

        if delta < 0.0 || rand::random::<f64>() < (-delta / temperature).exp() {
            current_mapping = neighbor;
            current_decrypted = decrypted;
            current_score = score;
        }
        iterations += 1;
        temperature *= 1.0 - cooling_rate;
    }
}

// Vigenère Cipher Decryption
fn number_to_key(num: usize, length: usize) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut key = String::with_capacity(length);
    let mut remaining = num;

    for _ in 0..length {
        let idx = remaining % 26;
        key.push(alphabet.chars().nth(idx).unwrap());
        remaining /= 26;
    }

    key
}

fn decrypt_vigenere_cipher(cipher_text: &str, wtr: &mut Writer<std::fs::File>) {
    let max_key_length = 8;
    let mut keys_tried = 0;
    let max_keys = 100000;

    for key_length in 1..=max_key_length {
        let total_keys = 26_usize.pow(key_length as u32);
        let max_keys_for_length = (max_keys / (max_key_length as usize)).min(total_keys);

        for key_num in 0..max_keys_for_length {
            let key = number_to_key(key_num, key_length);
            let decrypted = vigenere_decrypt(cipher_text, &key);
            let score = calculate_ngram_score(&decrypted);

            wtr.write_record(&[
                cipher_text.to_string(),
                key.clone(),
                decrypted.clone(),
                score.to_string(),
            ])
            .expect("Unable to write record");

            keys_tried += 1;
        }
    }
}

// Helper Functions

fn letter_frequencies(text: &str) -> Vec<(char, usize)> {
    let mut freq = [0usize; 26];
    for c in text.chars() {
        if c.is_ascii_uppercase() {
            freq[(c as u8 - b'A') as usize] += 1;
        }
    }
    let mut freq_vec: Vec<(char, usize)> =
        ('A'..='Z').zip(freq.iter()).map(|(c, &f)| (c, f)).collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending by frequency
    freq_vec
}

const PORTUGUESE_FREQUENCIES: [char; 26] = [
    'A', 'E', 'O', 'S', 'R', 'I', 'N', 'D', 'M', 'U', 'T', 'C', 'L', 'P', 'V', 'G', 'H', 'Q', 'B',
    'F', 'Z', 'J', 'X', 'K', 'W', 'Y',
];

fn initial_mapping(cipher_freq: &[(char, usize)]) -> [char; 26] {
    let mut mapping = ['A'; 26];
    for (i, &(c, _)) in cipher_freq.iter().enumerate() {
        mapping[(c as u8 - b'A') as usize] = PORTUGUESE_FREQUENCIES[i];
    }
    mapping
}

fn apply_mapping(text: &str, mapping: &[char; 26]) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_uppercase() {
            let index = (c as u8 - b'A') as usize;
            result.push(mapping[index]);
        } else {
            result.push(c);
        }
    }
    result
}

fn generate_neighbor(mapping: &[char; 26]) -> [char; 26] {
    let mut new_mapping = *mapping;
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..26);
    let j = rng.gen_range(0..26);
    new_mapping.swap(i, j);
    new_mapping
}

fn mapping_to_string(mapping: &[char; 26]) -> String {
    mapping.iter().collect()
}

fn vigenere_decrypt(text: &str, key: &str) -> String {
    let mut decrypted = String::new();
    let key_bytes = key.as_bytes();
    let key_length = key.len();
    for (i, c) in text.chars().enumerate() {
        if c.is_ascii_uppercase() {
            let c_index = c as u8 - b'A';
            let k = key_bytes[i % key_length] - b'A';
            let decrypted_c = ((26 + c_index - k) % 26) + b'A';
            decrypted.push(decrypted_c as char);
        } else {
            decrypted.push(c);
        }
    }
    decrypted
}

// N-gram Scoring
fn calculate_ngram_score(text: &str) -> f64 {
    let ngram_probs = get_bigram_probabilities();
    let mut score = 0.0;
    let text = text.to_uppercase();
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() - 1 {
        if chars[i].is_ascii_uppercase() && chars[i + 1].is_ascii_uppercase() {
            let bigram = format!("{}{}", chars[i], chars[i + 1]);
            if let Some(&prob) = ngram_probs.get(&bigram) {
                score += prob.ln();
            } else {
                // Assign a small probability to unseen bigrams
                score += -15.0; // Adjust as needed
            }
        }
    }
    -score // Return negative log-likelihood
}

fn get_bigram_probabilities() -> HashMap<String, f64> {
    // Simplified bigram frequencies for Portuguese
    // In practice, use a comprehensive bigram frequency table
    let bigrams = vec![
        ("DE", 2.98),
        ("AO", 1.62),
        ("OS", 1.57),
        ("DA", 1.51),
        ("DO", 1.48),
        ("EM", 1.45),
        ("SE", 1.31),
        ("ES", 1.28),
        ("OR", 1.26),
        ("RA", 1.22),
        // Add more bigrams as needed
    ];
    let total: f64 = bigrams.iter().map(|&(_, freq)| freq).sum();
    let mut probs = HashMap::new();
    for &(bigram, freq) in &bigrams {
        probs.insert(bigram.to_string(), freq / total);
    }
    probs
}

// Common Functions
fn read_lines(filename: &str) -> Vec<String> {
    let file = std::fs::File::open(filename).expect("Unable to open file");
    let buf_reader = std::io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
