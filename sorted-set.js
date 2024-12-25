class SortedSet {
  static BUCKET_RATIO = 16;
  static SPLIT_RATIO = 24;

  constructor(iterable = []) {
    const list = Array.from(new Set(iterable)).sort((x, y) => x - y);
    this.size = list.length;

    const num_bucket = Math.ceil(Math.sqrt(this.size / SortedSet.BUCKET_RATIO));
    this.bucketList = Array.from({ length: num_bucket }, (_, i) =>
      list.slice(Math.floor((this.size * i) / num_bucket), Math.floor((this.size * (i + 1)) / num_bucket))
    );
  }

  *[Symbol.iterator]() {
    for (const i of this.bucketList) {
      for (const j of i) {
        yield j;
      }
    }
  }

  *reversed() {
    for (let i = this.bucketList.length - 1; i >= 0; i--) {
      for (let j = this.bucketList[i].length - 1; j >= 0; j--) {
        yield this.bucketList[i][j];
      }
    }
  }

  equals(other) {
    return Array.from(this).toString() === Array.from(other).toString();
  }

  get length() {
    return this.size;
  }

  structure() {
    console.log("SortedSet" + JSON.stringify(this.bucketList));
    return;
  }

  debug() {
    console.log(Array.from(this).toString());
    return;
  }

  /**
   * xが属するべきバケットとその位置を返す
   * @param {T} x
   * @returns {[Array<T>, number, number]} [bucket, bucketIdx, i]
   */
  _position(x) {
    let bucketIdx, bucket;
    for ([bucketIdx, bucket] of this.bucketList.entries()) {
      if (x <= bucket[bucket.length - 1]) break;
    }
    return [bucket, bucketIdx, this._lowerBound(bucket, x)];
  }

  /**
   *
   * @param {*} x
   * @returns {boolean} xが含まれている時 true、含まれていない時 false
   */
  has(x) {
    if (this.size === 0) return false;
    const [bucket, _, i] = this._position(x);
    return i !== bucket.length && bucket[i] === x;
  }

  add(x) {
    if (this.size === 0) {
      this.bucketList = [[x]];
      this.size = 1;
      return true;
    }
    const [bucket, bucketIdx, i] = this._position(x);
    if (i !== bucket.length && bucket[i] === x) return false;
    bucket.splice(i, 0, x);
    this.size++;
    if (bucket.length > this.bucketList.length * SortedSet.SPLIT_RATIO) {
      const mid = bucket.length >> 1;
      this.bucketList.splice(bucketIdx, 1, bucket.slice(0, mid), bucket.slice(mid));
    }
    return true;
  }

  /**
   * @param {Array<T>} bucket
   * @param {number} bucketIdx
   * @param {number} i
   * @returns {T} ans
   */
  _pop(bucket, bucketIdx, i) {
    const ans = bucket.splice(i, 1)[0];
    this.size--;
    if (!bucket.length) this.bucketList.splice(bucketIdx, 1);
    return ans;
  }

  remove(x) {
    if (this.size === 0) return false;
    const [bucket, bucketIdx, i] = this._position(x);
    if (i === bucket.length || bucket[i] !== x) return false;
    this._pop(bucket, bucketIdx, i);
    return true;
  }

  // less than x。xより小さい最大の要素を返す。存在しない場合は undefined を返す
  lt(x) {
    for (let i = this.bucketList.length - 1; i >= 0; i--) {
      const bucket = this.bucketList[i];
      if (bucket[0] < x) return bucket[this._lowerBound(bucket, x) - 1];
    }
    return undefined;
  }

  // less than or equal to x。x以下の最大の要素を返す。存在しない場合は undefined を返す
  le(x) {
    for (let i = this.bucketList.length - 1; i >= 0; i--) {
      const bucket = this.bucketList[i];
      if (bucket[0] <= x) return bucket[this._upperBound(bucket, x) - 1];
    }
    return undefined;
  }

  // greater than x。xより大きい最小の要素を返す。存在しない場合は undefined を返す
  gt(x) {
    for (let i = 0; i < this.bucketList.length; i++) {
      const bucket = this.bucketList[i];
      if (bucket[bucket.length - 1] > x) return bucket[this._upperBound(bucket, x)];
    }
    return undefined;
  }

  // greater than or equal to x。x以上の最小の要素を返す。存在しない場合は undefined を返す
  ge(x) {
    for (let i = 0; i < this.bucketList.length; i++) {
      const bucket = this.bucketList[i];
      if (bucket[bucket.length - 1] >= x) return bucket[this._lowerBound(bucket, x)];
    }
    return undefined;
  }

  // k番目の要素を返す
  get(k) {
    if (k < 0) {
      for (let i = this.bucketList.length - 1; i >= 0; i--) {
        k += this.bucketList[i].length;
        if (k >= 0) return this.bucketList[i][k];
      }
    } else {
      for (let i = 0; i < this.bucketList.length; i++) {
        if (k < this.bucketList[i].length) return this.bucketList[i][k];
        k -= this.bucketList[i].length;
      }
    }
    return undefined;
  }

  // k番目の要素を削除して返す。動的にインデックスが変わるので注意
  pop(k = -1) {
    if (k < 0) {
      for (let i = this.bucketList.length - 1; i >= 0; i--) {
        k += this.bucketList[i].length;
        if (k >= 0) return this._pop(this.bucketList[i], i, k);
      }
    } else {
      for (let i = 0; i < this.bucketList.length; i++) {
        if (k < this.bucketList[i].length) return this._pop(this.bucketList[i], i, k);
        k -= this.bucketList[i].length;
      }
    }
    return undefined;
  }

  // xより小さい要素の個数を返す。xのインデックスと同じ
  index(x) {
    let ans = 0;
    for (const bucket of this.bucketList) {
      if (bucket[bucket.length - 1] >= x) return ans + this._lowerBound(bucket, x);
      ans += bucket.length;
    }
    return ans;
  }

  // x以下の要素の個数を返す。xのインデックス+1
  index_right(x) {
    let ans = 0;
    for (const bucket of this.bucketList) {
      if (bucket[bucket.length - 1] > x) return ans + this._upperBound(bucket, x);
      ans += bucket.length;
    }
    return ans;
  }

  _lowerBound(arr, x) {
    let ng = -1;
    let ok = arr.length;
    while (ok - ng > 1) {
      const mid = (ok + ng) >> 1;
      if (arr[mid] < x) ng = mid;
      else ok = mid;
    }
    return ok;
  }

  _upperBound(arr, x) {
    let ng = -1;
    let ok = arr.length;
    while (ok - ng > 1) {
      const mid = (ok + ng) >> 1;
      if (arr[mid] <= x) ng = mid;
      else ok = mid;
    }
    return ok;
  }

  min() {
    if (this.size === 0) return undefined;
    return this.bucketList[0][0];
  }

  max() {
    if (this.size === 0) return undefined;
    const lastBucket = this.bucketList[this.bucketList.length - 1];
    return lastBucket[lastBucket.length - 1];
  }

  clear() {
    this.bucketList = [];
    this.size = 0;
  }

  // 和集合（this ∪ other）
  union(other) {
    const res = new SortedSet(this);
    for (const x of other) res.add(x);
    return res;
  }

  // 積集合（this ∩ other）
  intersection(other) {
    const res = new SortedSet();
    for (const x of this) if (other.has(x)) res.add(x);
    return res;
  }

  // 差集合（this - other）
  difference(other) {
    const res = new SortedSet();
    for (const x of this) if (!other.has(x)) res.add(x);
    return res;
  }

  // 対象差（this ⊕ other = (this - other) ∪ (other - this)）
  symmetricDifference(other) {
    const res = new SortedSet();
    for (const x of this) if (!other.has(x)) res.add(x);
    for (const x of other) if (!this.has(x)) res.add(x);
    return res;
  }

  // 部分集合判定
  isSubset(other) {
    if (this.size > other.size) return false;
    for (const x of this) if (!other.has(x)) return false;
    return true;
  }

  // 真部分集合判定
  isProperSubset(other) {
    return this.size < other.size && this.isSubset(other);
  }

  // 上位集合判定
  isSuperset(other) {
    return other.isSubset(this);
  }
}
/*
  使い方
  this.bucketList: バケットのリスト。バケットはソートされている。
  this.size: 要素数
  add(x): xを追加してTrueを返す。ただしxがすでにs内にある場合、xは追加せずにFalseを返す。
  remove(x): xを削除してTrueを返す。ただしxがs内にない場合、何もせずにFalseを返す。
  lt(x): xより小さい最大の要素を返す。もし存在しないなら undefined を返す。
  le(x): x以下の最大の要素を返す。もし存在しないなら undefined を返す。
  gt(x): xより大きい最小の要素を返す。もし存在しないなら undefined を返す。
  ge(x): x以上の最小の要素を返す。もし存在しないなら undefined を返す。
  index(x): xより小さい要素の数を返す。
  index_right(x): x以下の要素の数を返す。
  */
