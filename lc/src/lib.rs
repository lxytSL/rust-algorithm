mod can_be_typed_words;
mod compare_version;
mod does_alice_win;
mod find_closest;
mod get_no_zero_integers;
mod largest_perimeter;
mod make_the_integer_zero;
mod max_frequency_elements;
mod min_score_triangulation;
mod minimum_teachings;
mod minimum_total;
mod number_containers;
mod people_aware_of_secret;
mod replace_non_coprimes;
mod sort_vowels;
mod spread_sheet;
mod task_manager;
mod triangle_number;
mod triangular_sum;
mod min_time;

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
