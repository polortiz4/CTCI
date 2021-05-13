// Solution to Exercise 5.3

/// Given an integer, find the largest number of ones in a row that could be written if a single bit is flipped
/// 
/// # Example:
/// ```
/// use ctci::bit_manipulation::flip_bit_to_win::flip_bit_to_win;
/// 
/// let inp = 10;  // 1010
/// assert_eq!(3, flip_bit_to_win(inp));
/// ```
pub fn flip_bit_to_win(input: u32) -> usize {
    let vectorized = ones_in_a_row(input);
    let mut max = 0;
    let mut add_one = 0;
    if vectorized.len() == 1 {return vectorized[0];}
    for (after, bef) in (vectorized[1..]).iter().zip((vectorized[..vectorized.len() - 1]).iter()){
        if bef + after > max {
            max = bef+after;
            add_one = 1;
        }
    }
    max + add_one
}
// Takes a i32 and returns a vector with the number of consecutive ones
// Example: 11011001 -> [1, 0, 2, 2]
fn ones_in_a_row(input: u32) -> Vec<usize> {
    if input == 0 {return vec![0]}
    let mut curr = input;
    let mut result = Vec::new();
    while curr != 0 {
        let mut n_ones = 0;
        while (curr >> 1)*2 + 1 == curr {
            curr = curr >> 1;
            n_ones += 1;
        }
        curr = curr >> 1;
        result.push(n_ones);
    }
    result
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn ones_in_a_row_test(){
        let inp = 10;  // 1010
        let inp2 = 11234;  // 10101111100010
        assert_eq!(vec![0, 1, 1], ones_in_a_row(inp));
        assert_eq!(vec![0, 1, 0, 0, 5, 1, 1], ones_in_a_row(inp2));
    }

    #[test]
    fn flip_bit_to_win_test(){
        let inp2 = 11234;  // 10101111100010
        assert_eq!(7, flip_bit_to_win(inp2));
        assert_eq!(0, flip_bit_to_win(0));
        assert_eq!(1, flip_bit_to_win(1));
        assert_eq!(2, flip_bit_to_win(2));
        assert_eq!(2, flip_bit_to_win(3));
    }
}