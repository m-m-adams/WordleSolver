use anyhow::Result;
use std::collections::HashSet;
use std::str;
type Word = [u8; 5];

fn word_from_str(s: &str) -> Result<Word> {
    let bytes = s.as_bytes();

    Ok(bytes[0..5].try_into()?)
}

#[derive(Debug, Clone)]
pub struct Wordle {
    _guess: usize,
    answer: Word,
    wordlist: Vec<Word>,
    state: WordleState,
}
#[derive(Debug, Clone)]
struct WordleState {
    known: [Option<u8>; 5],
    contains: HashSet<u8>,
    not_contains: HashSet<u8>,
}

impl Wordle {
    pub fn new(wordlist: Vec<String>, answer: &str) -> Wordle {
        Wordle {
            _guess: 0,
            answer: word_from_str(answer).expect("wrong answer length"),
            wordlist: wordlist
                .iter()
                .map(|word| word_from_str(word).expect("wrong answer length"))
                .collect(),
            state: WordleState {
                known: [None; 5],
                contains: vec![].into_iter().collect(),
                not_contains: vec![].into_iter().collect(),
            },
        }
    }
    pub fn guess(&mut self) -> Word {
        let starting = (self.wordlist.len() as f32).log2();
        let mut best_score = 0.;
        let mut best_guess = b"     ";
        for guess in &self.wordlist {
            let mut score = 0.;
            for pot_answer in &self.wordlist {
                let pot_state = self.evaluate(guess, pot_answer);
                let pot_words = self
                    .wordlist
                    .iter()
                    .cloned()
                    .filter(|w| Wordle::check_word(&pot_state, w))
                    .count();
                let i_gain = starting - (pot_words as f32).log2();
                score += i_gain;
            }

            if score > best_score {
                best_score = score;
                best_guess = guess;
            }
        }
        let pot_state = self.evaluate(best_guess, &self.answer);
        let remaining: Vec<Word> = self
            .wordlist
            .iter()
            .cloned()
            .filter(|w| Wordle::check_word(&pot_state, w))
            .collect();
        let remaining: Vec<String> = remaining
            .iter()
            .map(|a| str::from_utf8(a).expect("not a word").to_owned())
            .collect();
        for item in remaining {
            print!("{item} ")
        }
        println!("");
        println!("information gain is {best_score}");
        return *best_guess;
    }

    fn evaluate(&self, guess: &Word, answer: &Word) -> WordleState {
        let a = answer;
        debug_assert!(Wordle::check_word(&self.state, guess));
        let mut new_state = self.state.clone();

        for (i, l) in guess.iter().enumerate() {
            if *l == a[i] {
                new_state.known[i] = Some(*l);
            } else if a.contains(&l) {
                new_state.contains.insert(*l);
            } else {
                new_state.not_contains.insert(*l);
            }
        }

        new_state
    }

    //checks if a word is possible in a given state
    fn check_word(state: &WordleState, word: &Word) -> bool {
        for (i, letter) in word.iter().enumerate() {
            if state.not_contains.contains(&letter) {
                return false;
            }
            if let Some(l) = state.known[i] {
                if *letter != l {
                    return false;
                }
            }
        }
        for c in &state.contains {
            if !word.contains(c) {
                return false;
            }
        }
        return true;
    }
}
