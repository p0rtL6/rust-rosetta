// Implements http://rosettacode.org/wiki/Loops/N_plus_one_half

use std::iter;

fn main() {
    for i in (1us..11) {
        print!("{}", i);
        if i == 10 {
            break;
        }
        print!(", ");
    }
}
