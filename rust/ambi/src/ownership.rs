fn main() {
  let s = "owned data".to_string();
  {
    let t = s;
  }
  {
    let s = "owned data".to_string();
    let ref_s = &s;
    let t = s;
  }
}
