use rand::Rng;

/// Return random number from 6 sided dice (Wuürfel)
/// 
/// # Examples 
/// 
/// ```
/// // get a single number
/// let w = w6();
/// 
/// // print
/// print!("One role of the dice: {}", w6());
/// ```
/// 
fn w6() -> u8 {
  let x : u8 = rand::thread_rng().gen_range(1..7);
  return x;
}

fn main() {
  print!("{}", w6());
}
