fn main() {
    let text = String::from("Well my my hello there my fiend");
    rearrange(text);
}

fn rearrange(text: String) -> String {
    text
}

#[test]
fn string_is_returned() {
    let text = String::from("Well my my hello there my fiend");
    let result = rearrange(text);
    assert_eq!(result, "Well my my hello there my fiend");
}
