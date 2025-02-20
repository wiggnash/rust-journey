fn main() {
    let mut string = String::from("hello world!");
    let result = return_first_word(&string);
    println!("index of the space : {}", result);

    // if we do string.clear() here , then result will also becomes invalid
    string.clear();
    // println!("index of the space : {}", result);
    // value of result is 5 here also
}

fn return_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[0..i];
        }
    }

    &s[..]
}
