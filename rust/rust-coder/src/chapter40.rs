pub fn main() {
  let result = "120".parse();
  assert!(matches!(result, Ok(120)));
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), 120);

  let result = "xxx".parse();
  assert!(matches!(result, Err(_)));
  assert!(result.is_err());
  if let Err(ref err) = result {
    eprintln!("{}", err);
  }
  assert_eq!(result.unwrap_or(-1), -1);
}
