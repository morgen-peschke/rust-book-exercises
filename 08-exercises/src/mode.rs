use std::collections::HashMap;

/// Given a list of integers, use a vector and return the mode of the list
///
/// - Mode :: the value that occurs most often (A hash map will be helpful here)
pub fn mode(input: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for e in input.iter() {
        counts.entry(*e).and_modify(|c| *c += 1).or_insert(1);
    }
    if let Some((max, count)) = counts.iter().max_by(|(_, a), (_, b)| a.cmp(b)) {
        let potentials: Vec<(&i32, &usize)> = counts.iter().filter(|(_, c)| *c == count).collect();

        if potentials.len() == 1 {
            Some(*max)
        } else {
            potentials.iter().map(|(e, _)| **e).min()
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degenerate_cases() {
        assert_eq!(mode(&[]), None);
        assert_eq!(mode(&[1]), Some(1));
    }

    #[test]
    fn odd_numbered_lengths() {
        assert_eq!(mode(&[1, 2, 3]), Some(1));
        assert_eq!(mode(&[3, 1, 2]), Some(1));
        assert_eq!(mode(&[3, 2, 2]), Some(2));
        assert_eq!(mode(&[2, 1, 2]), Some(2));
        assert_eq!(mode(&[2, 2, 2]), Some(2));
    }

    #[test]
    fn even_numbered_lengths() {
        assert_eq!(mode(&[1, 2, 3, 4]), Some(1));
        assert_eq!(mode(&[3, 1, 2, 4]), Some(1));
        assert_eq!(mode(&[3, 2, 2, 4]), Some(2));
        assert_eq!(mode(&[2, 1, 2, 4]), Some(2));
        assert_eq!(mode(&[3, 1, 2, 2]), Some(2));
        assert_eq!(mode(&[4, 2, 2, 2]), Some(2));
    }
}
