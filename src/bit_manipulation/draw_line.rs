// Solution to Exercise 5.8

type Byte = u8;

/// Given a matrix in the form of an array of matrices and a width, draw a horizontal line from (x1, y) to (x2, y)
/// 
/// # Examples:
/// ```
/// use ctci::bit_manipulation::draw_line::draw_line;
/// 
/// // 0 0 0 0 0 0 0 0
/// // 0 0 0 0 0 0 0 0
/// // |
/// // v
/// // 0 0 0 0 0 0 0 0
/// // 0 0 0 1 1 1 0 0
/// 
/// let a = vec![0b0000_0000, 0b0000_0000];
/// let expected = vec![0b0000_0000, 0b0001_1100];
/// assert_eq!(expected, draw_line(a, 8, 3, 5, 1));
/// ```
pub fn draw_line(screen: Vec<Byte>, width: u32, x1: u32, x2: u32, y: u32) -> Vec<Byte>{
    assert_eq!(width % 8, 0);
    let row_y_idx = y * width/8;
    let start_idx = row_y_idx + x1/8;
    let end_idx = row_y_idx + x2/8;
    let x1_offset = x1 % 8;
    let x2_offset = x2 % 8;

    let mut result = screen.to_vec();

    // Set the first and last bytes
    let x1_mask = (1 << (8 - x1_offset)) - 1;
    let x2_mask = 0b1111_1111 << (8 - (x2_offset + 1));
    
    if start_idx == end_idx {
        result[end_idx as usize] = screen[end_idx as usize] | (x2_mask & x1_mask);
    }
    else{
        result[start_idx as usize] = screen[start_idx as usize] | x1_mask;
        result[end_idx as usize] = screen[end_idx as usize] | x2_mask;
    }

    for idx in (start_idx + 1)..end_idx{
        result[idx as usize] = 0b1111_1111;
    }

    result
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn draw_line_test(){
        let a = vec![0b0000_0000, 0b0000_0000];
        let expected = vec![0b0000_0000, 0b0001_1100];
        
        assert_eq!(expected, draw_line(a, 8, 3, 5, 1));
        
        let b = vec![0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000];
        let b_expected = vec![0b0011_1111, 0b1111_1111, 0b1111_1100, 0b0000_0000, 0b0000_0000, 0b0000_0000];
        assert_eq!(b_expected, draw_line(b, 24, 2, 21, 0));
    }
}