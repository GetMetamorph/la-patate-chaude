use crate::challenge::Challenge;
use crate::message_struct::{RecoverSecretIn, RecoverSecretOut};
use std::collections::HashMap;

pub struct Recover {
    pub input: RecoverSecretIn,
}

impl Challenge for Recover {
    type Input = RecoverSecretIn;
    type Output = RecoverSecretOut;

    fn name() -> String {
        "RecoverSecret".to_string()
    }
    fn new(input: Self::Input) -> Self {
        Recover { input }
    }

    fn solve(&self) -> Self::Output {
        let tab = create_element_tuples(self.input.letters.clone(), self.input.tuple_size.clone());
        let res = recover_secret(tab);
        Self::Output {
            secret: res
        }
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let mut last_seen: HashMap<char, usize> = HashMap::new();
        for (i, car) in answer.secret.chars().enumerate() {
            let tuple_index =
                create_element_tuples(self.input.letters.clone(), self.input.tuple_size.clone())
                    .iter()
                    .position(|tuple| tuple.contains(car))
                    .unwrap();
            let car_index =
                create_element_tuples(self.input.letters.clone(), self.input.tuple_size.clone())
                    [tuple_index]
                    .chars()
                    .position(|c| c == car)
                    .unwrap();
            if let Some(prev_index) = last_seen.get(&car) {
                if tuple_index != *prev_index || car_index != (*prev_index + 1) {
                    return false;
                }
            }
            last_seen.insert(car, tuple_index);
        }

        has_unique_chars(answer.secret.clone())
    }
}

fn create_element_tuples(letters: String, element_tuple_sizes: Vec<usize>) -> Vec<String> {
    let mut i = 0;
    let mut tuples = Vec::new();
    for size in element_tuple_sizes {
        tuples.push(letters[i..i + size].to_owned());
        i += size;
    }
    tuples
}

fn has_unique_chars(s: String) -> bool {
    let mut seen_chars = [false; 256];
    for c in s.chars() {
        let c_val = c as usize;
        if seen_chars[c_val] {
            return false;
        }
        seen_chars[c_val] = true;
    }
    true
}

fn recover_secret(tab: Vec<String>) -> String {
    let mut res = Vec::new();

    // Iterate over tuples in `tab`
    for tuple in tab {
        // Iterate over characters in the tuple
        for c in tuple.chars() {
            // Find the index of `c` in the tuple
            let idx = tuple.find(c).unwrap();

            // Check if the character should be inserted
            if idx == 0 || res.contains(&tuple.chars().nth(idx - 1).unwrap()) {
                if !res.contains(&c) {
                    // Insert `c` at the beginning of `res`
                    res.insert(0, c);
                }

                // Check if the character after `c` should be removed
                if idx + 1 < tuple.len() {
                    let a = res.iter().position(|&x| x == tuple.chars().nth(idx + 1).unwrap_or_default());
                    let b = res.iter().position(|&x| x == c).unwrap();
                    if let Some(x) = a {
                        if x < b {
                            res.remove(x);
                        }
                    }
                }
            }
        }
    }

    // Concatenate the characters in `res`
    res.iter().collect()
}

