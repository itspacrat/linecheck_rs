pub fn count_iter<T: Iterator>(data: T) -> usize {
    let mut count = 0;
        for (i, _) in data.enumerate() {
            count = i;
        }
        // account for indexing starting at 0 & return
        count+1
}

/// takes in an integer radicand, and an integer index, and checks if the
/// radicand is a perfect nth-root of that index.
pub fn is_perfect_power(radicand: f64, index: f64) -> bool {
    if  radicand > 0.0 {
        if radicand.powf(1.0 / index).trunc().powf(index) == radicand {
            true
        } else {
            false
        }
    } else {
        // no square of 0
        false
    }
}

/// checks if a
pub fn is_square(data: f64) -> bool {
    is_perfect_power(data as f64, 2.0)
}

pub fn is_cube(data: f64) -> bool {
    is_perfect_power(data as f64, 3.0)
}

pub fn is_square_iter<T: Iterator>(data: T) -> bool {
    is_square(count_iter(data) as f64)
}
pub fn is_cube_iter<T: Iterator>(data: T) -> bool {
    is_cube(count_iter(data) as f64)
}
#[cfg(test)]
mod tests {
    use super::*;

    // is_square_iter tests
    #[test]
    fn check_bad_square_length() {
        let result = is_square_iter(vec![1, 2, 3, 4, 5].iter());
        assert_eq!(result, false);
    }
    #[test]
    fn check_good_square_length() {
        let result = is_square_iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9].iter());
        assert_eq!(result, true);
    }
    #[test]
    fn check_bad_cube_length() {
        let result = is_cube_iter(vec![1, 2, 3, 4, 5].iter());
        assert_eq!(result, false);
    }
    #[test]
    fn check_good_cube_length() {
        let result = is_cube_iter(vec![1, 2, 3, 4, 5, 6, 7, 8].iter());
        assert_eq!(result, true);
    }
}
