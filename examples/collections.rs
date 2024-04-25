use std::collections::HashMap;

fn main() {

}

fn borrow(i : &i32) {
    println!("{:?}", i);
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::collections::HashMap;
    use crate::{borrow, words_stats};

    #[test]
    fn vec_intro() {
        // new empty vector
        let v: Vec<i32> = Vec::new();

        // new vector with values
        let mut v = vec![1, 2, 3];

        // adding values to vector
        v.push(4);
        v.push(5);

        // getting elements
        let n: &i32 = &v[0]; // borrow the value; panic when index out of bound
        let n: Option<&i32> = v.get(2); // none if index out of bound

        // removing values
        let e = v.pop();
    }

    #[test]
    fn vec_borrowing_values() {
        let mut v = vec![1, 2, 3];

        let first = &v[0];

        v.push(4);

        // println!("{}", *first); // uncomment to see the compile error
    }

    #[test]
    fn vec_immutable_iter() {
        let v = vec![1, 2, 3];

        for e in &v { // need to use reference (otherwise the v value is moved)
            println!("{}", *e); // need to explicitly derefer the value
        }
    }

    fn vec_mutable_iter() {
        let mut v = vec![1, 2, 3];

        for e in &mut v { // mutable borrow
            *e += 2;
        }

        assert_eq!(vec![2, 4, 6], v);
    }


    #[test]
    fn hash_map_intro() {
        let mut map = HashMap::new();

        map.insert(String::from("world"), 1);
        map.insert(String::from("rust"), 2);

        for (text, number) in &map {
            println!("{} has {} occurrences", text, number);
        }

        map.remove("world");
    }

    #[test]
    fn hash_map_words_stats() {
        let text = "Litwo! Ojczyzno moja! ty jesteś jak zdrowie. Litwo!";

        let stats = words_stats(text);
        assert_eq!(*stats.get("ojczyzno").unwrap(), 1);
        assert_eq!(*stats.get("litwo").unwrap(), 2);
    }

}


#[allow(dead_code)]
fn hash_map_words_stats_poem() {
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
            // map.entry(unified_word).and_modify(|counter| *counter += 1).or_insert(1);
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