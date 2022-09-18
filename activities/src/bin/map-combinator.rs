fn maybe_num() -> Option<i32> {}

fn maybe_word() -> Option<String> {}

fn main() {
    // let plus_one = match maybe_num() {
    //     Some(num) => Some(num + 1),
    //     None => None,
    // };

    // only applies if there is a value, otherwise map does nothing
    let plus_one = maybe_num().map(|num| num + 1);

    let word_length = maybe_word().map(|word| word.len()).map(|len| len * 2);
}
