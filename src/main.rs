fn read_wordlist() -> Vec<String> {
    std::fs::read_to_string("./corpus.txt")
        .expect("cannot open file")
        .split(",")
        .map(|s| s.trim().to_owned())
        .collect()

}

fn get_guess_and_results() -> (String, String) {
    let stdin = std::io::stdin();
    let mut guess = String::new();
    let mut result = String::new();

    println!("Enter guess:");
    stdin.read_line(&mut guess).unwrap();
    println!("\nEnter Result (x = not present, p = present in wrong position, c = correct position):");
    stdin.read_line(&mut result).unwrap();

    (guess, result)
}

fn main() {
    let mut words = read_wordlist();

    while words.len() > 1 {
        let (guess, result) = get_guess_and_results();
        let mut seen_before = Vec::new();
        for (result_char, (pos, guess_char)) in result.chars().zip(guess.chars().enumerate()) {
            match result_char {
                'x' => {
                    if seen_before.contains(&guess_char) {
                        words.retain(|w| w.matches(guess_char).count() == 1);
                    } else {
                        words.retain(|w| !w.contains(guess_char));
                    }
                },
                'p' => {
                    seen_before.push(guess_char);
                    words.retain(|w| w.contains(guess_char) && w.chars().nth(pos).unwrap() != guess_char);
                },
                'c' => {
                    seen_before.push(guess_char);
                    words.retain(|w| w.chars().nth(pos).unwrap() == guess_char);
                },
                _ => {}
            }
        }

        println!("Potential words: {:?}\n", words);
    }

    if words.len() == 1 {
        println!("Answer: {}", words[0]);
    } else {
        println!("Nothing left...\nYou're on your own, bucko!");
    }
}
