class UnionFind {
  constructor(n) {
    this.par = new Array(n).fill(-1);
    this.rank = new Array(n).fill(0);
    this.siz = new Array(n).fill(1);
    this.min_node = new Array(n).fill(0);

    for (let v = 0; v < n; v++) {
      this.min_node[v] = v;
    }
  }

  // 根を求めるメソッド
  root(x) {
    if (this.par[x] === -1) {
      return x;
    } else {
      this.par[x] = this.root(this.par[x]); // 経路圧縮
      return this.par[x];
    }
  }

  // x と y が同じグループに属するかどうか
  issame(x, y) {
    return this.root(x) === this.root(y);
  }

  // x を含むグループと y を含むグループを併合する
  unite(x, y) {
    let rx = this.root(x);
    let ry = this.root(y);
    if (rx === ry) return false;

    // union by rank
    if (this.rank[rx] < this.rank[ry]) {
      [rx, ry] = [ry, rx];
    }
    this.par[ry] = rx;
    if (this.rank[rx] === this.rank[ry]) {
      this.rank[rx]++;
    }
    this.siz[rx] += this.siz[ry];

    this.min_node[rx] = Math.min(this.min_node[rx], this.min_node[ry]);

    return true;
  }

  // x を含む根付き木のサイズを求める
  size(x) {
    return this.siz[this.root(x)];
  }

  get_min_node(x) {
    return this.min_node[this.root(x)];
  }
}
