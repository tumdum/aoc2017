use std::io::{BufRead, BufReader};

fn valid(p: &Vec<String>) -> bool {
    let unique : std::collections::HashSet<&String> =
        p.iter().collect();
    unique.len() == p.len()
}

fn is_anagram(l: &str, r: &str) -> bool {

    let mut lc = l.chars().collect::<Vec<char>>();
    let mut rc = r.chars().collect::<Vec<char>>();
    lc.sort();
    rc.sort();
    lc == rc
}

fn no_anagrams(p: &Vec<String>) -> bool {
    for (il,l) in p.iter().enumerate() {
        for (ir,r) in p.iter().enumerate() {
            if il == ir {
                continue;
            }
            if is_anagram(l, r) {
                return false;
            }
        }
    }
    true
}

fn valid_b(p: &Vec<String>) -> bool {
    no_anagrams(p) && valid(p)
}

fn main() {
    let passphrases : Vec<String> = BufReader::new(std::io::stdin()).lines().map(|l| l.unwrap()).collect();
    let passphrases : Vec<Vec<String>> = passphrases.iter().map(|l| l.split(" ").map(|x| x.to_owned()).collect()).collect();
    let a = passphrases.iter().filter(|r| valid(r)).count();
    println!("{}", a);
    let b = passphrases.iter().filter(|r| valid(r) && no_anagrams(r)).count();
    println!("{}", b);
}

#[test]
fn valid_test() {
    assert!(valid(&vec!["aa".to_owned(), "bb".to_owned(), "cc".to_owned(), "dd".to_owned(), "ee".to_owned()]));
    assert!(!valid(&vec!["aa".to_owned(), "bb".to_owned(), "cc".to_owned(), "dd".to_owned(), "aa".to_owned()]));
}

#[test]
fn valid_b_test() {
    assert!(valid_b(&vec!["abcde".to_owned(), "fghij".to_owned()]));
}
