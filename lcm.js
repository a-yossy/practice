const gcd = (a, b) => {
  if (a < b) {
    return gcd(b, a);
  }
  while (b !== 0) {
    const remainder = a % b;
    a = b;
    b = remainder;
  }

  return a;
};
const lcm = (a, b) => {
  return (a * b) / gcd(a, b);
};
