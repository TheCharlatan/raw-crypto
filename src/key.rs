extern "C" {
  fn generate_key_pair(public: *mut u8, secret: *mut u8);
  fn check_public_key(public_key: *const u8) -> bool;
  fn secret_key_to_public_key(secret_key: *const u8, public_key: *mut u8) -> bool;
}

pub struct Key {}

impl Key {
  pub fn generate(public_key: &mut [u8; 32], private_key: &mut [u8; 32]) {
    unsafe { generate_key_pair(public_key.as_mut_ptr(), private_key.as_mut_ptr()) }
  }
  pub fn check_public_key(public_key: &[u8]) -> bool {
    unsafe { return check_public_key(public_key.as_ptr()) }
  }
  pub fn secret_to_public(secret_key: &[u8], public_key: &mut [u8]) -> bool {
    unsafe { return secret_key_to_public_key(secret_key.as_ptr(), public_key.as_mut_ptr()) }
  }
}
