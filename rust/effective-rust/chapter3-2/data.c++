class BankAccount {
  public:
    BankAccount(): balance_(0) {}

    int64_t balance() const {
      if (balance_ < 0) {
        std::cerr << "**. Oh no, gone overdrawn: " << balance_ << "!**\n";
        std::abort();
      }
      return balance_;
    }
    void deposit(uint32_t amount) {
      balance_ += amount;
    }
    bool withdraw(uinit32_t amount) {
      if (balance_ < amount) {
        return false;
      }
      std::this_thread::sleep_for(std::chrono::milliseconds(500));

      balance_ -= amount;
      return true;
    }

    private:
    int64_t balance_;
};
