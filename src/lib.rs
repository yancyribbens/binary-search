use core::cmp::Ordering::*;

fn binary_search(s: &[u32], x: &u32) -> Result<usize, usize> {
    let mut size = s.len();
    let mut left = 0;
    let mut right = size;

    while left < right {
        let mid = left + size / 2;
        let val = s.get(mid).unwrap();
        let cmp = val.cmp(&x); 

        match cmp {
            Less => left = mid + 1,
            Greater => right = mid,
            Equal => return Ok(mid),
        };

        size = right - left;
    }

    Err(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        assert_eq!(binary_search(&s, &13), Ok(9));
        assert_eq!(binary_search(&s, &4), Err(7));
        assert_eq!(binary_search(&s, &100), Err(13));
    }
}
