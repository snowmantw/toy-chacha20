#![feature(slice_patterns)]     // to make passing multable slice elements possible.
#![feature(wrapping_int_impl)]

extern crate rayon;
use rayon::prelude::*;

// Wrapping for overflows during computing.
use std::num::Wrapping;

fn main() {
    let key = [1,2,3,4,5,6,7,8];
    let nounce = [9,8,7];
    let plaintext: Vec<Wrapping<u32>> = "0123456789ABCDEF".chars().map(|c| Wrapping(c as u32)).collect();    // 16 words, 4 bytes/32 bits per word

    let mut cc20 = ChaCha20::new(&key, &nounce, 1);
    cc20.scramble();

    let enctext: Vec<Wrapping<u32>> = cc20.state.iter().zip(plaintext.iter()).map(|k| {
        let (s, c) = k;
        return s ^ c;
    }).collect();
    println!("text: {:?}, enc text: {:?}", plaintext, enctext);
}

struct ChaCha20 {
    state: [Wrapping<u32>; 16]
}

impl ChaCha20 {

    // key: 8 * 4 bytes = 32bytes
    // nounce:  3 * 4 bytes = 12 bytes
    // counter: 4 bytes
    fn new(key: &[u32; 8], nounce: &[u32; 3], counter: u32) -> ChaCha20 {
        return ChaCha20 {
            state:  [Wrapping(0x65787061), Wrapping(0x6e642033), Wrapping(0x322d6279), Wrapping(0x7465206b),
                     Wrapping(key[0]), Wrapping(key[1]), Wrapping(key[2]), Wrapping(key[3]),
                     Wrapping(key[4]), Wrapping(key[5]), Wrapping(key[6]), Wrapping(key[3]),
                     Wrapping(counter), Wrapping(nounce[0]), Wrapping(nounce[1]), Wrapping(nounce[2])]
        };
    }

    fn scramble(&mut self) {
        // Vertical quarter rounds.
        let mut result_v = [self.state[0], self.state[4], self.state[8], self.state[12]
                           ,self.state[1], self.state[5], self.state[9], self.state[13]
                           ,self.state[2], self.state[6], self.state[10], self.state[14]
                           ,self.state[3], self.state[7], self.state[11], self.state[15]
                           ];

        result_v.par_chunks_mut(4).for_each(|slot| {
            quarter_round(slot);
        });

        self.set_band(&result_v, 0, 4, 8, 12);
        self.set_band(&result_v, 1, 5, 9, 13);
        self.set_band(&result_v, 2, 6, 10, 14);
        self.set_band(&result_v, 3, 7, 11, 15);

        let mut result_d = [self.state[0], self.state[5], self.state[10], self.state[15]
                           ,self.state[1], self.state[6], self.state[11], self.state[12]
                           ,self.state[2], self.state[7], self.state[8], self.state[13]
                           ,self.state[3], self.state[4], self.state[9], self.state[14]
                           ];

        result_d.par_chunks_mut(4).for_each(|slot| {
            quarter_round(slot);
        });

        self.set_band(&result_d, 0, 5, 10, 15);
        self.set_band(&result_d, 1, 6, 11, 12);
        self.set_band(&result_d, 2, 7, 8, 13);
        self.set_band(&result_d, 3, 4, 9, 14);
    }

    fn set_band(&mut self, result: &[Wrapping<u32>], a: usize, b: usize, c: usize, d: usize) {
        self.state[a] = result[0];
        self.state[b] = result[1];
        self.state[c] = result[2];
        self.state[d] = result[3];
    }
}

fn quarter_round(result: &mut [Wrapping<u32>]) {
    for _ in 0..10 { 
        result[0] += result[1];
        result[3] ^= result[0];
        result[3] = result[3].rotate_left(16);

        result[2] += result[3];
        result[1] ^= result[2];
        result[1] = result[1].rotate_left(12);

        result[0] += result[1];
        result[3] ^= result[0];
        result[3] = result[3].rotate_left(8);

        result[2] += result[3];
        result[1] ^= result[2];
        result[1] = result[1].rotate_left(7);
    }
}




