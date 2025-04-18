pub fn print_bitboard(bitboard: u64) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let mask = 1u64 << ((rank * 8) + file);

            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_bitboard_works() {
        print_bitboard(1);
        println!();
        print_bitboard(2);
        println!();
        print_bitboard(268435456);
        println!();
        print_bitboard(216172782113784000);
    }
}
