class MutexLock {
  public:
    MutexLock(Mutex* mu): mu_(mut) { mu_->lock(); }
    ~MutexLock() { mu_->unlock(); }
    private:
      Mutex* mu_;
}


class ThreadSafeInt {
  public:
    ThreadSafeInt(int v): value_(v) {}

    void add(int delta) {
      mu_.lock();
      value_ += delta;
      if (value_ > MAX_INT) {
        return;
      }
      mu_.unlock();
    }

    void add_with_modification(int delta) {
      MutexLock with_lock(&mu_);
      value_ += delta;
      if (value_ > MAX_INT) {
        return;
      }
    }
}
