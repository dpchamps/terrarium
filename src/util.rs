pub fn vec_to_int(bits: &[u8]) -> u8 {
    bits.into_iter()
        .filter(|&x| *x == 0 || *x == 1)
        .fold(0, |acc, &bit| (acc << 1) ^ bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_bits_to_num() {
        let result = vec_to_int(&vec![1, 1, 0, 1]);

        assert_eq!(result, 13);
    }

    #[test]
    fn handles_non_zero_nums() {
        let result = vec_to_int(&vec![10, 10, 0]);

        assert_eq!(result, 0);
    }
}
