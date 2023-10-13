use ::rand::distributions::Alphanumeric;
use ::rand::{thread_rng, Rng};

pub fn generate_random_txt(size: usize) -> String {
  let rand_txt: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(size)
    .map(char::from)
    .collect();
  return rand_txt;
}
