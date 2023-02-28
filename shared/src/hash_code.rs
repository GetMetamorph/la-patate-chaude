use crate::challenge::Challenge;
use crate::message_struct::{MD5HashCashInput, MD5HashCashOutput};

pub struct Hash {
    pub hash_in: MD5HashCashInput
}


impl Challenge for Hash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(hash_in: Self::Input) -> Self {
        Hash { hash_in }
    }

    fn solve(&self) -> Self::Output {
        let mut answer = Self::Output {
            seed: 0,
            hashcode: "".to_string(),
        };

        for seed in 0..=u64::MAX {
            let input = format!("{seed:0>16X}{}", self.hash_in.message);
            let hashcode = format!("{:0>16X}", md5::compute(&input));
            let num_hashcode = u128::from_str_radix(&hashcode, 16).unwrap();

            let zeros = num_hashcode.leading_zeros();
            if zeros >= self.hash_in.complexity {
                answer = Self::Output { seed, hashcode };
                break;
            }
        }
        answer
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let seed = answer.seed;
        let input = format!("{seed:0>16X}{}", self.hash_in.message);
        let hashcode = format!("{:X}", md5::compute(&input));
        count_bits_to_zero(&hashcode) >= self.hash_in.complexity && answer.hashcode == hashcode
    }
}

fn count_bits_to_zero(hex_string: &str) -> u32 {
    let hex_value = u128::from_str_radix(hex_string, 16).unwrap();
    hex_value.leading_zeros()
}