use std::collections::HashMap;

fn main() {
    hash_map_ex3();

}

#[allow(dead_code)]
fn hash_map_ex1() {
    let mut map = HashMap::new();

    map.insert(String::from("world"), 1);
    map.insert(String::from("rust"), 2);

    for (text, number) in &map {
        println!("{} has {} occurences", text, number);
    }
}

#[allow(dead_code)]
fn hash_map_ex2() {
    let text = "Litwo! Ojczyzno moja! ty jesteś jak zdrowie:";

    let stats = words_stats(text);
    println!("{:?}", stats);
}

#[allow(dead_code)]
fn hash_map_ex3() {
    let response = reqwest::blocking::get("https://wolnelektury.pl/media/book/txt/pan-tadeusz.txt").expect("Cannot get poem from a given URL");
    let poem = response.text().unwrap();

    let stats = words_stats(&poem);
    let sorted_stats = sort_stats(&stats);
    println!("{:?}", &sorted_stats[..20]);
}

fn words_stats(s: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    for word in s.split_whitespace() {
        let unified_word = word.trim_matches(|c| char::is_ascii_punctuation(&c)).to_lowercase();
        if unified_word.len() > 4 {
            let count = map.entry(unified_word).or_insert(0);
            *count += 1;
        }
    }
    map
}

fn sort_stats(stats : &HashMap<String, i32>) -> Vec<(&str, i32)> {
    let mut sorted_stats : Vec<(&str, i32)> = Vec::new();
    for (word, count) in stats.iter() {
        sorted_stats.push((word, *count));
    }

    sorted_stats.sort_by(|(_, c1), (_, c2)| c2.partial_cmp(c1).expect("FAILED"));

    sorted_stats
}