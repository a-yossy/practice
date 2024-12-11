class PriorityQueue {
  constructor() {
    this.heap = [];
  }

  // 要素を追加する (valueは [cost, node] の形式)
  push(value) {
    this.heap.push(value);
    this._heapifyUp(this.heap.length - 1);
  }

  // 最小要素を取り出す
  pop() {
    if (this.heap.length === 0) return undefined;
    const top = this.heap[0];
    const end = this.heap.pop();
    if (this.heap.length > 0) {
      this.heap[0] = end;
      this._heapifyDown(0);
    }
    return top;
  }

  peek() {
    return this.heap.length === 0 ? undefined : this.heap[0];
  }

  size() {
    return this.heap.length;
  }

  _heapifyUp(idx) {
    while (idx > 0) {
      const parentIdx = (idx - 1) >> 1;
      // 比較はdistanceで行うため、[0]を参照
      if (this.heap[parentIdx][0] <= this.heap[idx][0]) break;
      [this.heap[parentIdx], this.heap[idx]] = [this.heap[idx], this.heap[parentIdx]];
      idx = parentIdx;
    }
  }

  _heapifyDown(idx) {
    const length = this.heap.length;
    while (true) {
      const left = (idx << 1) + 1;
      const right = (idx << 1) + 2;
      let smallest = idx;

      if (left < length && this.heap[left][0] < this.heap[smallest][0]) {
        smallest = left;
      }
      if (right < length && this.heap[right][0] < this.heap[smallest][0]) {
        smallest = right;
      }

      if (smallest === idx) break;

      [this.heap[idx], this.heap[smallest]] = [this.heap[smallest], this.heap[idx]];
      idx = smallest;
    }
  }
}
