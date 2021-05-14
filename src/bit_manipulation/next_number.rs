// Solution to Exercise 5.4

/// Given an input, return the both the next smallest and next largest number
/// 
/// # Examples:
/// ```
/// use ctci::bit_manipulation::next_number::next_number;
/// 
/// let a = 10;  // 1010
/// assert_eq!((Some(9), Some(12)), next_number(a));
/// ```
pub fn next_number(input: u32) -> (Option<u32>, Option<u32>) {
    let next_small = flip_first_zero_forward(input);
    let next_large = flip_first_one_forward(input);
    (next_small, next_large)
}

fn flip_first_one_forward(input: u32) -> Option<u32> {
    let one_idx = get_first_one_from(input, 0)?;
    let zero_idx = get_first_zero_from(input, one_idx);

    let mut result = update_bit(input, zero_idx, true);
    result = update_bit(result, one_idx, false);
    Some(shift_ones_left(result, zero_idx))
}
fn flip_first_zero_forward(input: u32) -> Option<u32> {
    let zero_idx = get_first_zero_from(input, 0);
    let one_idx = get_first_one_from(input, zero_idx)?;
    // if one_idx - 1 > -1 && one_idx - 1 !=

    let mut result = update_bit(input, zero_idx, true);
    result = update_bit(result, one_idx, false);
    Some(shift_ones_right(result, one_idx))
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
fn shift_ones_left(input: u32, idx: usize) -> u32{
    let mut copy = input;
    let mut n_ones = 0;
    for _ in 0..idx {
        let new_copy = copy >> 1;
        if new_copy*2 + 1 == copy {
            n_ones += 1;
        }
        copy = new_copy;
    }
    copy = copy << idx;
    copy += (1 << n_ones) - 1;
    copy
}
fn shift_ones_right(input: u32, idx: usize) -> u32{
    let mut copy = input;
    let mut n_ones = 0;
    for _ in 0..idx {
        let new_copy = copy >> 1;
        if new_copy*2 + 1 == copy {
            n_ones += 1;
        }
        copy = new_copy;
    }
    copy = copy << idx;
    copy += ((1 << n_ones) - 1) << (idx - n_ones);
    copy
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_shifter(){
        let a = 22;  // 10110
        let _expected = 19;  // 10011
        let _expected2 = 21;  // 10101
        assert_eq!(22, shift_ones_left(a, 1));
        assert_eq!(21, shift_ones_left(a, 2));
        assert_eq!(19, shift_ones_left(a, 3));
        assert_eq!(19, shift_ones_left(a, 4));
        assert_eq!(7, shift_ones_left(a, 5));

        assert_eq!(22, shift_ones_right(a, 1));
        assert_eq!(22, shift_ones_right(a, 2));
        assert_eq!(22, shift_ones_right(a, 3));
        assert_eq!(28, shift_ones_right(a, 4));
        assert_eq!(28, shift_ones_right(a, 5));
        assert_eq!(56, shift_ones_right(a, 6));
    }
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
        let d = 22;  // 10110
        let d_expected = (Some(21), Some(25));  // (10101, 11001)
        let e = 23;  // 10111
        let e_expected = (Some(15), Some(27));  // (01111, 11011)
        let f = 167;  // 10100111
        let f_expected = (Some(158), Some(171));  // (10011110, 10101011)
        
        let ctci_example = 13948;  // 11011001111100
        let ctci_expected = (Some(13946), Some(13967));  // (11011001111010, 11011010001111)
        
        let b = 0;  // 0
        let c = 7;  // 111
        assert_eq!((Some(9), Some(12)), next_number(a));
        assert_eq!((None, None), next_number(b));
        assert_eq!((None, Some(11)), next_number(c));
        assert_eq!(d_expected, next_number(d));
        assert_eq!(e_expected, next_number(e));
        assert_eq!(f_expected, next_number(f));
        assert_eq!(ctci_expected, next_number(ctci_example));
    }
}