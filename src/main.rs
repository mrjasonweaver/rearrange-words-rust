// Rearrange the words in text such that all words are rearranged in an increasing order of their lengths.
// If two words have the same length, arrange them in their original order.
// https://leetcode.com/problems/rearrange-words-in-a-sentence/description/

fn main() {
    let text = String::from("Well hello to you my friend");
    rearrange(text);
}

fn upper_first_char(s: &str) -> String {
    let mut chars = s.chars();
    let c = chars.next().unwrap().to_uppercase().to_string();
    let s: String = chars.collect();
    c + &s
}

fn rearrange(text: String) -> String {
    let words: Vec<_> = text.split(' ').collect();
    let mut words = words.into_iter().map(|w| w.to_lowercase()).collect::<Vec<_>>();
    words.sort_by_key(|w| w.len());
    words[0] = upper_first_char(&words[0]);
    words.join(" ")
}

#[test]
fn rearrange_text_version_one() {
    let text = String::from("Leetcode is cool");
    let text = rearrange(text);
    assert_eq!(text, "Is cool leetcode");
}

#[test]
fn rearrange_text_version_two() {
    let text = String::from("Keep calm and code on");
    let text = rearrange(text);
    assert_eq!(text, "On and keep calm code");
}

#[test]
fn rearrange_text_version_three() {
    let text = String::from("To be or not to be");
    let text = rearrange(text);
    assert_eq!(text, "To be or to be not");
}
