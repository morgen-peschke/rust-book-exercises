/// Given a list of integers, use a vector and return the median of the list
///
/// - Median :: when sorted, the value in the middle position
pub fn median(input: &[i32]) -> Option<i32> {
    if input.is_empty() {
        return None;
    }
    if input.iter().is_sorted() {
        let mid = input.len() / 2;
        input.get(mid).copied()
    } else {
        let mut local_copy = input.to_vec();
        local_copy.sort_unstable();
        let mid = local_copy.len() / 2;
        local_copy.get(mid).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degenerate_cases() {
        assert_eq!(median(&[]), None);
        assert_eq!(median(&[1]), Some(1));
    }

    #[test]
    fn odd_numbered_lengths() {
        assert_eq!(median(&[1, 2, 3]), Some(2));
        assert_eq!(median(&[3, 1, 2]), Some(2));
        assert_eq!(median(&[2, 2, 2]), Some(2));
    }

    #[test]
    fn even_numbered_lengths() {
        assert_eq!(median(&[1, 2, 3, 4]), Some(3));
        assert_eq!(median(&[3, 1, 2, 4]), Some(3));
        assert_eq!(median(&[4, 2, 2, 2]), Some(2));
    }
}
