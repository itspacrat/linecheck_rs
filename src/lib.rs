pub fn is_square_int(data: usize) -> bool {
    if data > 0 {
        let sq = ((data as f64).sqrt()).trunc() as usize;
        if sq * sq == data {
            true
        } else {
            false
        }
    } else {
        // no square of 0 exists
        false
    }
}

pub fn is_square_iter<T: Iterator>(data: T) -> bool {
    let int_data = {
        let mut count = 0;
        for (i,_) in data.enumerate() {
            count = i;
        }
        // account for indexing starting at 0 & return
        count+1
    };
    is_square_int(int_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    // is_square_iter tests
    #[test]
    fn check_bad_square_length() {
        let result = is_square_iter(vec![1,2,3,4,5].iter());
        assert_eq!(result, false);
    }
    #[test]
    fn check_good_square_length() {
        let result = is_square_iter(vec![1,2,3,4,5,6,7,8,9].iter());
        assert_eq!(result, true);
    }

}