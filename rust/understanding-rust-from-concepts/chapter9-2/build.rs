fn main() {
  cc::Build::new()
    .file("src/point.c")
    .compile("point");
}
