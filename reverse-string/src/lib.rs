pub fn reverse(input: &str) -> String {
    let mut char_array: Vec<char> = Vec::new();

    for c in input.chars() {
        char_array.push(c);
    }

    let mut reversed_char_array: Vec<char> = Vec::new();

    for (i, _) in char_array.iter().enumerate() {
        let index = char_array.len() - i - 1;
        reversed_char_array.push(char_array[index]);
    }

    let reversed_string: String = reversed_char_array.into_iter().collect();

    reversed_string
}
