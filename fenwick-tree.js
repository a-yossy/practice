class FenwickTree {
  /**
   * フィンウィック木を初期化します。
   * @param {number} size - 配列のサイズ
   */
  constructor(size) {
    this._n = size;
    this.data = new Array(size).fill(0); // 0ベースの配列
  }

  /**
   * 指定したインデックスに値を追加します。
   * @param {number} p - 0ベースのインデックス
   * @param {number} x - 追加する値
   */
  add(p, x) {
    if (p < 0 || p >= this._n) {
      throw new Error(`Index out of bounds: p=${p}, size=${this._n}`);
    }
    p += 1; // 1ベースに変換
    while (p <= this._n) {
      this.data[p - 1] += x; // 0ベースに戻してアクセス
      p += p & -p;
    }
  }

  /**
   * 区間 [l, r) の累積和を取得します。
   * @param {number} l - 0ベースの開始インデックス
   * @param {number} r - 0ベースの終了インデックス（右端は含まない）
   * @returns {number} 区間の累積和
   */
  sum(l, r) {
    if (l < 0 || r > this._n || l > r) {
      throw new Error(`Invalid range: l=${l}, r=${r}, size=${this._n}`);
    }
    return this._sum(r) - this._sum(l);
  }

  /**
   * [0, r) の累積和を取得します。
   * @param {number} r - 0ベースの終了インデックス（右端は含まない）
   * @returns {number} 累積和
   */
  _sum(r) {
    let s = 0;
    while (r > 0) {
      s += this.data[r - 1];
      r -= r & -r;
    }
    return s;
  }
}
