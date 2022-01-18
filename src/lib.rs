use anyhow::Result;
use std::{collections::HashMap, str};
type Word = [u8; 5];

fn word_from_str(s: &str) -> Result<Word> {
    let bytes = s.as_bytes();

    Ok(bytes[0..5].try_into()?)
}

#[derive(Debug, Clone)]
pub struct Wordle {
    guess: usize,
    answer: Word,
    guesses: Vec<Word>,
    wordlist: Vec<Word>,
    pub state: WordleState,
}

#[derive(Debug, Clone)]
pub struct WordleState {
    pub known: [Option<u8>; 5],
    pub contains: Vec<u8>,
    pub not_contains: Vec<u8>,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Wr {
    Green,
    Yellow,
    Grey,
}

type WordleResult = [Wr; 5];

impl Wordle {
    pub fn new(wordlist: Vec<String>, answer: &str) -> Wordle {
        Wordle {
            guess: 0,
            answer: word_from_str(answer).expect("wrong answer length"),
            wordlist: wordlist
                .iter()
                .map(|word| word_from_str(word).expect("wrong answer length"))
                .collect(),
            guesses: vec![],
            state: WordleState {
                known: [None; 5],
                contains: vec![],
                not_contains: vec![],
            },
        }
    }

    pub fn solve(&mut self) -> Result<(String, usize)> {
        let first_guess = word_from_str("raise")?;
        let (state, _) = self.evaluate(&first_guess, &self.answer);
        self.update_state(state);
        self.guess = 1;
        let guess = loop {
            let guess = self.guess();
            let (state, res) = self.evaluate(&guess, &self.answer);
            self.update_state(state);
            // let g = str::from_utf8(&guess).expect("not a word");
            // println!("{g} gives {res:?}");
            // for word in &self.wordlist {
            //     let w = str::from_utf8(word).expect("not a word");
            //     print!("{w:?}, ");
            // }
            // println!("");

            self.guess += 1;
            self.guesses.push(guess);
            if self.wordlist.len() == 1 {
                break self.wordlist[0];
            }
        };

        let answer = str::from_utf8(&guess)?;

        Ok((answer.to_string(), self.guess))
    }

    pub fn update_state(&mut self, answer: WordleState) {
        self.state = answer;
        let remaining: Vec<Word> = self
            .wordlist
            .iter()
            .cloned()
            .filter(|w| Wordle::check_word(&self.state, w))
            .collect();
        self.wordlist = remaining;
    }

    pub fn guess(&mut self) -> Word {
        let starting = (self.wordlist.len() as f32).log2();
        let mut best_score = 0.;
        let mut best_guess = b"     ";
        for guess in &self.wordlist {
            if self.guesses.contains(guess) {
                continue;
            }
            let mut score = 0.;
            let mut result_lengths: HashMap<WordleResult, usize> = HashMap::new();
            for pot_answer in &self.wordlist {
                let (pot_state, result) = self.evaluate(guess, pot_answer);
                if !result_lengths.contains_key(&result) {
                    let pot_words = self
                        .wordlist
                        .iter()
                        .cloned()
                        .filter(|w| Wordle::check_word(&pot_state, w))
                        .count();
                    result_lengths.insert(result.clone(), pot_words);
                }

                let i_gain = starting - (result_lengths[&result] as f32).log2();
                score += i_gain;
            }

            if score > best_score {
                best_score = score;
                best_guess = guess;
            }
        }
        let _score = best_score / self.wordlist.len() as f32;
        let _guess = str::from_utf8(best_guess).expect("not a word");
        return *best_guess;
    }

    fn evaluate(&self, guess: &Word, answer: &Word) -> (WordleState, WordleResult) {
        debug_assert!(Wordle::check_word(&self.state, guess));
        let mut wr: WordleResult = [Wr::Green; 5];
        let mut new_state = self.state.clone();

        for (i, c) in guess.iter().enumerate() {
            if *c == answer[i] {
                wr[i] = Wr::Green;
                new_state.known[i] = Some(*c);
            } else if answer.contains(&c) {
                wr[i] = Wr::Yellow;
                new_state.contains.push(*c);
            } else {
                wr[i] = Wr::Grey;
                new_state.not_contains.push(*c);
            }
        }

        (new_state, wr)
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
