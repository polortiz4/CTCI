pub fn set_bit(num: i32, i: usize) -> i32{
    num | (1 << i)
}
pub fn get_bit(num: i32, i: usize) -> i32{
    if (num & (1 << i)) != 0 {1} else {0}
}
pub fn clear_bit(num: i32, i: usize) -> i32{
    num & !(1 << i)
}
pub fn update_bit(num: i32, i: usize, bit_is_1: bool) -> i32{
    let value = if bit_is_1 {1} else {0};
    let mask = !(1<<i);
    (mask & num) | (value << i)
}
pub fn clear_bits_ms_through_i(num: i32, i: usize) -> i32{
    let mask = (1 << i) - 1;  // 11111000
    num & mask
}

pub fn clear_bits_i_through_0(num: i32, i: usize) -> i32{
    let mask = (1 << (i + 1)) - 1;
    num & !mask
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fancy_clears(){
      let a = 10;
      assert_eq!(clear_bits_ms_through_i(a, 2), 2);
      assert_eq!(clear_bits_ms_through_i(a, 0), 0);
      assert_eq!(clear_bits_ms_through_i(a, 3), 2);

      assert_eq!(clear_bits_i_through_0(a, 2), 8);
      assert_eq!(clear_bits_i_through_0(a, 0), 10);
      assert_eq!(clear_bits_i_through_0(a, 1), 8);
      assert_eq!(clear_bits_i_through_0(a, 3), 0);
  }

  #[test]
  fn test_set(){
      let a = 2;
      assert_eq!(set_bit(a, 0), 3);
      assert_eq!(set_bit(a, 1), 2);
  }

  #[test]
  fn test_get(){
      let a = 2;
      assert_eq!(get_bit(a, 0), 0);
      assert_eq!(get_bit(a, 1), 1);
  }

  #[test]
  fn test_clear(){
      let a = 2;
      assert_eq!(clear_bit(a, 0), 2);
      assert_eq!(clear_bit(a, 1), 0);
  }

  #[test]
  fn test_update(){
      let a = 2;
      assert_eq!(update_bit(a, 0, false), 2);
      assert_eq!(update_bit(a, 0, true), 3);
      assert_eq!(update_bit(a, 1, false), 0);
      assert_eq!(update_bit(a, 1, true), 2);
  }
}