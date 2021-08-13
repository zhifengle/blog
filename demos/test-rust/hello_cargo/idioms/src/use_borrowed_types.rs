// &str 由于 &String  同理 &[T]  &Vec<T>   &T   &Box<T>

fn three_vowels(word: &str) -> bool {
    let mut count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
                if count >= 3 {
                    return true;
                }
            }
            _ => count = 0,
        }
    }
    false
}
