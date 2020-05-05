pub fn pig_latin(input: &str) -> String {
    let mut indices = input.char_indices();
    let first_char = indices
        .next()
        .expect("Can't convert empty string to pig latin")
        .1;
    let rest = indices.as_str();
    format!("{}{}ay", rest, first_char)
}
