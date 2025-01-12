fn main() {
    let interesting_words = String::from("Well we all know a lot about this and that");
    /*
     * Create list of Vowel letters
     * IF vowel
     *  add -hay to the end of the word, apple becomes apple-hay
     * ELSE
     *  first letter is moved to the end of the word along with ay, first becomes irst-fay
     */

    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();

    // Loop through our strirng and check if the first char is a vowel and do the appropiate
    // transformation
    for word in interesting_words.split_whitespace() {
        if let Some(first_char) = word.chars().next() {
            let first_char = first_char.to_lowercase().next().unwrap();
            let mut pig_word = String::new();

            if vowels.contains(&first_char) {
                pig_word = format!("{word}-hay ");
            } else {
                let mut word_copy = word.to_string();
                let first_letter = word_copy.remove(0);
                pig_word = format!("{word_copy}-{first_letter}ay ");
            }

            result.push_str(&pig_word);
        }
    }

    println!("Latin result? {}", result);
}
