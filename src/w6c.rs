use rand::Rng;

/// Return random number from 6 si&ded dice (Wuürfel)
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
  
  let mut c1 = 0; 
  let mut c2 = 0;
  let mut c3 = 0;
  let mut c4 = 0;
  let mut c5 = 0;
  let mut c6 = 0;
  
  for _i in 1..=100 {
    // role a dice
    let n = w6();
    println!("{} ", n);
    match n {
      1 => c1 = c1 + 1,
      2 => c2 = c2 + 1,
      3 => c3 = c3 + 1,
      4 => c4 = c4 + 1,
      5 => c5 = c5 + 1,
      6 => c6 = c6 + 1,
      _ => break,
    }
  }
  
  println!("
  1 = {c1}
  2 = {c2}
  3 = {c3}
  4 = {c4}
  5 = {c5}
  6 = {c6}
  ")
}
