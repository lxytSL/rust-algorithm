mod find_closest;
mod get_no_zero_integers;
mod make_the_integer_zero;
mod minimum_teachings;
mod people_aware_of_secret;
mod sort_vowels;
mod does_alice_win;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
