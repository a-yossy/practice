class SortedSet {
  constructor() {
    this.tree = [SortedSet.newChild(-1)];
    (this.root = this.size = 0), (this.reuse = []);
  }

  add(x) {
    let v = this.has(x);
    if (v >= 0) return false;
    SortedSet.addmain(this, x, ~v);
    return true;
  }

  static addmain(self, x, v) {
    let tree = self.tree,
      p = tree[v].parent;
    (tree[v].black = 0), (tree[v].value = x);
    self.size++;
    if (self.reuse.length) {
      let l = self.reuse.pop(),
        r = self.reuse.pop();
      (tree[v].left = l), (tree[v].right = r);
      SortedSet.reset(tree, l);
      SortedSet.reset(tree, r);
      tree[l].parent = tree[r].parent = v;
    } else {
      let l = tree.push(SortedSet.newChild(v), SortedSet.newChild(v)) - 2;
      (tree[v].left = l), (tree[v].right = l + 1);
    }
    if (p === -1) return (tree[v].black = 1);
    let V = v,
      P = p;
    while (P !== -1) {
      if (tree[P].left === V) tree[P].lSize++;
      (V = P), (P = tree[P].parent);
    }
    while (tree[p].black === 0) {
      let pp = tree[p].parent,
        u;
      if (tree[pp].left === p) u = tree[pp].right;
      else u = tree[pp].left;
      if (tree[u].black) {
        let ppp = tree[pp].parent;
        if (tree[pp].left === p)
          if (tree[p].left === v) {
            let pr = tree[p].right;
            (tree[p].right = pp), (tree[p].parent = ppp), (tree[p].black = 1);
            (tree[pp].left = pr),
              (tree[pp].lSize -= tree[p].lSize + 1),
              (tree[pp].parent = p),
              (tree[pp].black = 0);
            tree[pr].parent = pp;
            if (ppp === -1) self.root = p;
            else if (tree[ppp].left === pp) tree[ppp].left = p;
            else tree[ppp].right = p;
          } else {
            let vl = tree[v].left,
              vr = tree[v].right;
            (tree[v].left = p),
              (tree[v].right = pp),
              (tree[v].lSize += tree[p].lSize + 1),
              (tree[v].parent = ppp),
              (tree[v].black = 1);
            (tree[p].right = vl), (tree[p].parent = v);
            (tree[pp].left = vr),
              (tree[pp].lSize -= tree[v].lSize + 1),
              (tree[pp].parent = v),
              (tree[pp].black = 0);
            (tree[vl].parent = p), (tree[vr].parent = pp);
            if (ppp === -1) self.root = v;
            else if (tree[ppp].left === pp) tree[ppp].left = v;
            else tree[ppp].right = v;
          }
        else if (tree[p].left === v) {
          let vl = tree[v].left,
            vr = tree[v].right;
          (tree[p].left = vr),
            (tree[p].lSize -= tree[v].lSize + 1),
            (tree[p].parent = v);
          (tree[v].left = pp),
            (tree[v].right = p),
            (tree[v].lSize += tree[pp].lSize + 1),
            (tree[v].parent = ppp),
            (tree[v].black = 1);
          (tree[pp].right = vl), (tree[pp].parent = v), (tree[pp].black = 0);
          (tree[vl].parent = pp), (tree[vr].parent = p);
          if (ppp === -1) self.root = v;
          else if (tree[ppp].left === pp) tree[ppp].left = v;
          else tree[ppp].right = v;
        } else {
          let pl = tree[p].left;
          (tree[p].left = pp),
            (tree[p].lSize += tree[pp].lSize + 1),
            (tree[p].parent = ppp),
            (tree[p].black = 1);
          (tree[pp].right = pl), (tree[pp].parent = p), (tree[pp].black = 0);
          tree[pl].parent = pp;
          if (ppp === -1) self.root = p;
          else if (tree[ppp].left === pp) tree[ppp].left = p;
          else tree[ppp].right = p;
        }
        break;
      }
      (tree[p].black = tree[u].black = 1), (tree[pp].black = 0);
      (v = pp), (p = tree[pp].parent);
      if (p === -1) return (tree[v].black = 1);
    }
  }

  delete(x) {
    let v = this.has(x);
    if (v < 0) return false;
    SortedSet.deletemain(this, v);
    return true;
  }

  static deletemain(self, v) {
    let tree = self.tree,
      l = tree[v].left,
      r = tree[v].right;
    while (tree[l].left !== tree[r].left) {
      if (tree[l].left !== -1) {
        while (tree[l].right !== -1) l = tree[l].right;
        l = tree[l].parent;
        tree[v].value = tree[l].value;
        v = l;
      } else {
        while (tree[r].left !== -1) r = tree[r].left;
        r = tree[r].parent;
        tree[v].value = tree[r].value;
        v = r;
      }
      (l = tree[v].left), (r = tree[v].right);
    }
    self.size--;
    let p = tree[v].parent,
      V = v,
      P = p,
      b = tree[v].black;
    while (P !== -1) {
      if (tree[P].left === V) tree[P].lSize--;
      (V = P), (P = tree[P].parent);
    }
    self.reuse.push(l, r);
    (tree[v].value = undefined),
      (tree[v].left = tree[v].right = -1),
      (tree[v].lSize = 0),
      (tree[v].black = 1);
    if (b === 0 || p === -1) return;
    if (tree[p].left === v) SortedSet.leftShortage(self, tree, v, p);
    else SortedSet.rightShortage(self, tree, v, p);
  }

  static leftShortage(self, tree, v, p) {
    let s = tree[p].right,
      sl = tree[s].left,
      sr = tree[s].right,
      pp = tree[p].parent;
    if (tree[s].black === 0) {
      (tree[p].right = sl), (tree[p].parent = s), (tree[p].black = 0);
      tree[sl].parent = p;
      (tree[s].left = p),
        (tree[s].lSize += tree[p].lSize + 1),
        (tree[s].parent = pp),
        (tree[s].black = 1);
      if (pp === -1) self.root = s;
      else if (tree[pp].left === p) tree[pp].left = s;
      else tree[pp].right = s;
      (pp = s), (s = sl), (sl = tree[s].left), (sr = tree[s].right);
    }
    if (tree[sr].black === 0) {
      (tree[sl].parent = p), (tree[sr].black = 1);
      (tree[s].left = p),
        (tree[s].lSize += tree[p].lSize + 1),
        (tree[s].parent = pp),
        (tree[s].black = tree[p].black);
      (tree[p].right = sl), (tree[p].parent = s), (tree[p].black = 1);
      if (pp === -1) (self.root = s), (tree[s].black = 1);
      else if (tree[pp].left === p) tree[pp].left = s;
      else tree[pp].right = s;
      return;
    }
    if (tree[sl].black === 0) {
      let sll = tree[sl].left,
        slr = tree[sl].right;
      (tree[sll].parent = p), (tree[slr].parent = s);
      (tree[s].left = slr),
        (tree[s].lSize -= tree[sl].lSize + 1),
        (tree[s].parent = sl);
      (tree[sl].left = p),
        (tree[sl].right = s),
        (tree[sl].lSize += tree[p].lSize + 1),
        (tree[sl].parent = pp),
        (tree[sl].black = tree[p].black);
      (tree[p].right = sll), (tree[p].parent = sl), (tree[p].black = 1);
      if (pp === -1) (self.root = sl), (tree[sl].black = 1);
      else if (tree[pp].left === p) tree[pp].left = sl;
      else tree[pp].right = sl;
      return;
    }
    tree[s].black = 0;
    if (tree[p].black === 0) return (tree[p].black = 1);
    if (pp === -1) return;
    if (tree[pp].left === p) SortedSet.leftShortage(self, tree, p, pp);
    else SortedSet.rightShortage(self, tree, p, pp);
  }

  static rightShortage(self, tree, v, p) {
    let s = tree[p].left,
      sl = tree[s].left,
      sr = tree[s].right,
      pp = tree[p].parent;
    if (tree[s].black === 0) {
      (tree[p].left = sr),
        (tree[p].lSize -= tree[s].lSize + 1),
        (tree[p].parent = s),
        (tree[p].black = 0);
      tree[sr].parent = p;
      (tree[s].right = p), (tree[s].parent = pp), (tree[s].black = 1);
      if (pp === -1) self.root = s;
      else if (tree[pp].left === p) tree[pp].left = s;
      else tree[pp].right = s;
      (pp = s), (s = sr), (sl = tree[s].left), (sr = tree[s].right);
    }
    if (tree[sl].black === 0) {
      (tree[sr].parent = p), (tree[sl].black = 1);
      (tree[s].right = p),
        (tree[s].parent = pp),
        (tree[s].black = tree[p].black);
      (tree[p].left = sr),
        (tree[p].lSize -= tree[s].lSize + 1),
        (tree[p].parent = s),
        (tree[p].black = 1);
      if (pp === -1) (self.root = s), (tree[s].black = 1);
      else if (tree[pp].left === p) tree[pp].left = s;
      else tree[pp].right = s;
      return;
    }
    if (tree[sr].black === 0) {
      let srl = tree[sr].left,
        srr = tree[sr].right;
      (tree[srr].parent = p), (tree[srl].parent = s);
      (tree[s].right = srl), (tree[s].parent = sr);
      (tree[sr].left = s),
        (tree[sr].right = p),
        (tree[sr].lSize += tree[s].lSize + 1),
        (tree[sr].parent = pp),
        (tree[sr].black = tree[p].black);
      (tree[p].left = srr), (tree[p].parent = sr), (tree[p].black = 1);
      if (pp === -1) (self.root = sr), (tree[sr].black = 1);
      else if (tree[pp].left === p) tree[pp].left = sr;
      else tree[pp].right = sr;
      return;
    }
    tree[s].black = 0;
    if (tree[p].black === 0) return (tree[p].black = 1);
    if (pp === -1) return;
    if (tree[pp].left === p) SortedSet.leftShortage(self, tree, p, pp);
    else SortedSet.rightShortage(self, tree, p, pp);
  }

  xor(x) {
    let v = this.has(x);
    if (v < 0) {
      SortedSet.addmain(this, x, ~v);
      return false;
    }
    SortedSet.deletemain(this, v);
    return true;
  }

  has(x) {
    let v = this.root,
      tree = this.tree;
    while (tree[v].left !== -1) {
      if (tree[v].value === x) return v;
      if (tree[v].value < x) v = tree[v].right;
      else v = tree[v].left;
    }
    return ~v;
  }

  lowerBound(x) {
    let res = 0,
      v = this.root,
      tree = this.tree,
      V = -1;
    while (tree[v].left !== -1) {
      if (x < tree[v].value) (V = v), (v = tree[v].left);
      else if (x === tree[v].value) return [res + tree[v].lSize, v];
      else (res += tree[v].lSize + 1), (v = tree[v].right);
    }
    return [res, V];
  }

  upperBound(x) {
    let res = 0,
      v = this.root,
      tree = this.tree,
      V = -1;
    while (tree[v].left !== -1) {
      if (x === tree[v].value) {
        (res += tree[v].lSize + 1), (v = tree[tree[v].right].left);
        if (v === -1) return [res, V];
        while (tree[v].left !== -1) v = tree[v].left;
        return [res, tree[v].parent];
      }
      if (x < tree[v].value) (V = v), (v = tree[v].left);
      else (res += tree[v].lSize + 1), (v = tree[v].right);
    }
    return [res, V];
  }

  next(id) {
    let tree = this.tree,
      c = tree[id].right;
    if (tree[c].left === -1) {
      let p = tree[id].parent;
      while (p !== -1) {
        if (tree[p].left === id) return p;
        (id = p), (p = tree[p].parent);
      }
      return -1;
    } else {
      while (tree[c].left !== -1) c = tree[c].left;
      return tree[c].parent;
    }
  }

  back(id) {
    let tree = this.tree,
      c = tree[id].left;
    if (tree[c].left === -1) {
      let p = tree[id].parent;
      while (p !== -1) {
        if (tree[p].right === id) return p;
        (id = p), (p = tree[p].parent);
      }
      return -1;
    } else {
      while (tree[c].right !== -1) c = tree[c].right;
      return tree[c].parent;
    }
  }

  get first() {
    let v = this.root,
      tree = this.tree;
    while (tree[v].left !== -1) v = tree[v].left;
    return tree[v].parent;
  }
  get last() {
    let v = this.root,
      tree = this.tree;
    while (tree[v].right !== -1) v = tree[v].right;
    return tree[v].parent;
  }

  get show() {
    let v = [this.root],
      tree = this.tree;
    while (v.length) {
      let v2 = [],
        s = [];
      for (let i of v) {
        if (tree[i].left === -1) s.push(-1);
        else {
          s.push(tree[i].value + (tree[i].black ? "b" : ""));
          v2.push(tree[i].left, tree[i].right);
        }
      }
      console.log(s);
      v = v2;
    }
  }

  static newChild(p) {
    return {
      value: undefined,
      left: -1,
      right: -1,
      lSize: 0,
      parent: p,
      black: 1,
    };
  }

  static reset(tree, v) {
    tree[v].value = undefined;
    tree[v].left = tree[v].right = -1;
    (tree[v].lSize = 0), (tree[v].black = 1);
  }

  get(k) {
    let v = this.root;
    let tree = this.tree;

    while (tree[v].left !== -1) {
      if (k === tree[v].lSize) {
        return tree[v].value;
      } else if (k < tree[v].lSize) {
        v = tree[v].left;
      } else {
        k -= tree[v].lSize + 1;
        v = tree[v].right;
      }
    }

    return tree[v].value;
  }

  getSize() {
    return this.size;
  }

  toArray() {
    const result = [];
    this._inOrderTraversal(this.root, result);
    return result;
  }

  _inOrderTraversal(node, result) {
    if (node === -1) return;
    this._inOrderTraversal(this.tree[node].left, result);
    if (this.tree[node].value !== undefined) {
      result.push(this.tree[node].value);
    }
    this._inOrderTraversal(this.tree[node].right, result);
  }

  *[Symbol.iterator]() {
    let current = this.first;
    while (current !== -1) {
      yield this.tree[current].value;
      current = this.next(current);
    }
  }
}
