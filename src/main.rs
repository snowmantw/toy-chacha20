#![feature(slice_patterns)]     // to make passing multable slice elements possible.
#![feature(wrapping_int_impl)]

// Wrapping for overflows during computing.
use std::num::Wrapping;

fn main() {
    let key = [1,2,3,4,5,6,7,8];
    let nounce = [9,8,7];
    let mut cc20 = ChaCha20::new(&key, &nounce, 1);
    if let Some(n) = cc20.next() {
        println!("{}", n);
    }
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
        for _ in 0..10 { 
            // Vertical quarter rounds.
            let qresult = quarter_round(self.state[0], self.state[4], self.state[8], self.state[12]);
            self.set_state(&qresult, 0, 4, 8, 12);

            let qresult = quarter_round(self.state[1], self.state[5], self.state[9], self.state[13]);
            self.set_state(&qresult, 1, 5, 9, 13);

            let qresult = quarter_round(self.state[2], self.state[6], self.state[10], self.state[14]);
            self.set_state(&qresult, 2, 6, 10, 14);

            let qresult = quarter_round(self.state[3], self.state[7], self.state[11], self.state[15]);
            self.set_state(&qresult, 3, 7, 11, 15);

            // Diagonal quarter rounds.
            let qresult = quarter_round(self.state[0], self.state[5], self.state[10], self.state[15]);
            self.set_state(&qresult, 0, 5, 10, 15);

            let qresult = quarter_round(self.state[1], self.state[6], self.state[11], self.state[12]);
            self.set_state(&qresult, 1, 6, 11, 12);

            let qresult = quarter_round(self.state[2], self.state[7], self.state[8], self.state[13]);
            self.set_state(&qresult, 2, 7, 8, 13);

            let qresult = quarter_round(self.state[3], self.state[4], self.state[9], self.state[14]);
            self.set_state(&qresult, 3, 4, 9, 14);
        }
    }

    fn set_state(&mut self, qres: &[Wrapping<u32>], a: usize, b: usize, c: usize, d: usize) {
        let rstate = &mut self.state;
        rstate[a] = qres[0];
        rstate[b] = qres[1];
        rstate[c] = qres[2];
        rstate[d] = qres[3];
    }

}

impl Iterator for ChaCha20 {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.scramble();
        return Some(2);
    }
}

fn quarter_round(_a: Wrapping<u32>, _b: Wrapping<u32>, _c: Wrapping<u32>, _d: Wrapping<u32>) -> [Wrapping<u32>; 4] {
    
    let mut a = _a.clone();
    let mut b = _b.clone();
    let mut c = _c.clone();
    let mut d = _d.clone();

    a += b;
    d ^= a;
    d = d.rotate_left(16);

    c += d;
    b ^= c;
    b = b.rotate_left(12);

    a += b;
    d ^= a;
    d = d.rotate_left(8);

    c += d;
    b ^= c;
    b = b.rotate_left(7);

    return [a, b, c, d];
}
