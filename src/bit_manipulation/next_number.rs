// Solution to Exercise 5.4

/// Given an input, return the both the next smallest and next largest number
/// 
/// # Examples:
/// ```
/// use ctci::bit_manipulation::next_number::next_number;
/// 
/// let a = 10;  // 1010
/// assert_eq!(Some((9, 12)), next_number(a));
/// ```
pub fn next_number(input: u32) -> (Option<u32>, Option<u32>) {
    let next_small = flip_first_zero_forward(input);
    let next_large = flip_first_one_forward(input);
    (next_small, next_large)
}

fn flip_first_one_forward(input: u32) -> Option<u32> {
    let one_idx = get_first_one_from(input, 0)?;
    let zero_idx = get_first_zero_from(input, one_idx);

    let result = update_bit(input, zero_idx, true);
    Some(update_bit(result, one_idx, false))
}
fn flip_first_zero_forward(input: u32) -> Option<u32> {
    let zero_idx = get_first_zero_from(input, 0);
    let one_idx = get_first_one_from(input, zero_idx)?;

    let result = update_bit(input, zero_idx, true);
    Some(update_bit(result, one_idx, false))
}
fn get_first_from(input: u32, start_idx: usize, one: bool) -> Option<usize> {
    let bit_size = 32;
    let expected;
    if one {
        expected = 1
    } else {
        expected = 0
    }

    let mut idx = start_idx;
    while idx < bit_size {
        if get_bit(input, idx) == expected {
            break;
        }
        idx += 1;
    }
    if idx == bit_size {None} else {Some(idx)}
}
fn get_first_one_from(input: u32, start_idx: usize) -> Option<usize> {
    get_first_from(input, start_idx, true)
}
fn get_first_zero_from(input: u32, start_idx: usize) -> usize {
    get_first_from(input, start_idx, false).unwrap()
}
fn get_bit(num: u32, i: usize) -> u32 {
    if (num & (1 << i)) != 0 {
        1
    } else {
        0
    }
}
fn update_bit(num: u32, i: usize, bit_is_1: bool) -> u32{
    let value = if bit_is_1 {1} else {0};
    let mask = !(1<<i);
    (mask & num) | (value << i)
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_getter(){
        let a = 10;  // 1010
        assert_eq!(Some(1), get_first_one_from(a, 0));
        assert_eq!(Some(1), get_first_one_from(a, 1));
        assert_eq!(Some(3), get_first_one_from(a, 2));
        assert_eq!(Some(3), get_first_one_from(a, 3));
        assert_eq!(None, get_first_one_from(a, 4));

        assert_eq!(0, get_first_zero_from(a, 0));
        assert_eq!(2, get_first_zero_from(a, 1));
        assert_eq!(2, get_first_zero_from(a, 2));
        assert_eq!(4, get_first_zero_from(a, 3));
    }

    #[test]
    fn test_next_number(){
        let a = 10;  // 1010
        let b = 0;  // 0
        let c = 7;  // 111
        assert_eq!((Some(9), Some(12)), next_number(a));
        assert_eq!((None, None), next_number(b));
        assert_eq!((None, Some(14)), next_number(c));
    }
}