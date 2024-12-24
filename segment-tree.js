class SegTree {
  constructor(m, op, e) {
    this.n = Math.pow(2, Math.ceil(Math.log2(m))); // 葉の数を確定
    this.op = op; // 演算子
    this.e = e; // 単位元
    this.node = new Array(2 * this.n).fill(e); // 0-indexedで葉を初期化
  }

  update(a, x) {
    a += this.n;
    this.node[a] = x;
    while (a > 0) {
      a >>= 1;
      this.node[a] = this.op(this.node[2 * a], this.node[2 * a + 1]); // 親ノードに子ノードの演算結果を反映
    }
  }

  get(a) {
    a += this.n;
    return this.node[a];
  }

  query(a, b) {
    // 閉区間[a, b]の演算結果を取得
    let res = this.e;
    a += this.n;
    b += this.n + 1;

    while (a < b) {
      res = a % 2 === 1 ? this.op(res, this.node[a++]) : res;
      res = b % 2 === 1 ? this.op(res, this.node[--b]) : res;
      a >>= 1;
      b >>= 1;
    }

    return res;
  }
  show() {
    console.log(this.node.slice(this.n, 2 * this.n));
  }
}
