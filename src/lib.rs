/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
  let code_vec = Vec::from(code);
  if code_vec.iter().fold(0, |acc, &x| {
    if x as char != ' ' {
      return acc + 1;
    }
    acc
  }) <= 1
  {
    return false;
  }
  let result = code_vec
    .into_iter()
    .rev()
    .filter(|&x| x as char != ' ')
    .enumerate()
    .map(|(index, digit_as_u8)| {
      if digit_as_u8 > 57 || digit_as_u8 < 48 {
        return Err(0);
      }
      let digit = (digit_as_u8 as char).to_string().parse::<u32>().unwrap();
      if index % 2 != 0 {
        let doubled_digit = digit * 2;
        if doubled_digit > 9 {
          return Ok(doubled_digit - 9);
        }
        return Ok(doubled_digit);
      } else {
        return Ok(digit);
      }
    })
    .collect::<Result<Vec<u32>, u32>>();
  match result {
    Ok(vec) => {
      if vec.iter().sum::<u32>() % 10 == 0 {
        return true;
      } else {
        return false;
      }
    }
    Err(_) => return false,
  }
}
