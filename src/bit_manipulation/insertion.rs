// Solutions to Exercise 5.1


/// Given two 32-bit integers, insert one into the other starting at index j ending in index i
/// 
/// # Examples:
/// ```
/// use ctci::bit_manipulation::insertion::insertion;
/// assert_eq!(3, insertion(1, 2, 0, 0));
/// ```
pub fn insertion(m: i32, n:i32, i: i32, j: i32) -> i32{
    let m_size = get_size(m).unwrap_or(-1);
    let shifted_m = m << (j-m_size);
    let mask = (1 << ((j - i) + 1)) - 1;  // 111000
    let mask = mask << i;
    let shifted_m = mask & shifted_m;
    let result = n & !mask;  // Clear the nits to set
    let result = result | shifted_m;
    result
}
fn get_size(num: i32) -> Option<i32> {
    if num == 0 {return None;}
    let mut res = 1;
    let mut ans = -1;
    while num >= res{
        res *= 2;
        ans += 1;
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_size(){
      assert_eq!(get_size(0), None);
      assert_eq!(get_size(1), Some(0));
      assert_eq!(get_size(3), Some(1));
      assert_eq!(get_size(4), Some(2));
      assert_eq!(get_size(4), Some(2));
  }

  #[test]
  fn test_insertion(){
      let a = 20;  // 10100
      let b = 5;  //    101
      let start = 3;
      let end = 2;
      let expected = 24; // 11000
      assert_eq!(expected, insertion(b, a, end, start));
      assert_eq!(3, insertion(1, 2, 0, 0));
  }
}