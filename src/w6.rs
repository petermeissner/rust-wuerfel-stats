use rand::Rng;


fn w6() -> u8 {
  let x : u8 = rand::thread_rng().gen_range(1..7);
  return x;
}

fn main() {
  print!("{}", w6());
}
