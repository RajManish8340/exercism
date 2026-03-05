pub fn reverse(input: &str) -> String {
    if input == "" {
        return "".to_string();
    }

    // if input == "".to_string() {
    //     return "".to_string();
    // }
    let mut x: Vec<char> = input.chars().collect::<Vec<char>>();
    // let mut bytes = input.bytes().collect::<Vec<_>>();
    let mut left_index = 0;
    let mut right_index = x.len() - 1;

    while left_index < right_index {
        x.swap(left_index, right_index);
        // bytes.swap(left_index, right_index);

        left_index += 1;
        right_index -= 1;
    }

    x.iter().collect()
    // bytes.iter().collect::<Vec<_>>()
}
