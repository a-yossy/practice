use std::{collections::HashMap, hash::Hash, sync::Mutex};

fn main() {
    pub struct BankAccount {
        balance: std::sync::Mutex<i64>,
    }

    impl BankAccount {
        pub fn new() -> Self {
            BankAccount {
                balance: std::sync::Mutex::new(0),
            }
        }
        pub fn balance(&self) -> i64 {
            let balance = *self.balance.lock().unwrap();
            if balance < 0 {
                panic!("** Oh no, gone overdrawn: {}", balance);
            }
            balance
        }
        pub fn deposit(&self, amount: i64) {
            *self.balance.lock().unwrap() += amount;
        }
        pub fn withdraw(&self, amount: i64) -> bool {
            let mut balance = self.balance.lock().unwrap();
            if *balance < amount {
                return false;
            }
            *balance -= amount;
            true
        }
    }

    pub fn pay_in(account: &BankAccount) {
        loop {
            if account.balance() < 200 {
                println!("[A] Running low, deposit 400");
                account.deposit(400);
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    pub fn take_out(account: &BankAccount) {
        loop {
            if account.withdraw(100) {
                println!("[B] Withdrew 100, balance now {}", account.balance());
            } else {
                println!("[B] Failed to withdraw 100");
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    {
        let mut account = BankAccount::new();
        let _payer = std::thread::spawn(move || pay_in(&mut account));
    }

    {
        let account = std::sync::Arc::new(BankAccount::new());
        account.deposit(1000);

        let account2 = account.clone();
        let _taker = std::thread::spawn(move || take_out(&account2));

        let account3 = account.clone();
        let _payer = std::thread::spawn(move || pay_in(&account3));
    }

    struct Player {
        name: String,
    }
    #[derive(Clone)]
    struct GameId(usize);
    struct Game {
        id: GameId,
    }

    impl Game {
        fn add_player(&mut self, username: &str) -> bool {
            println!("Player {} added to game {}", username, self.id.0);
            true
        }

        fn remove_player(&mut self, username: &str) {
            println!("Player {} removed from game {}", username, self.id.0);
        }

        fn has_player(&self, username: &str) -> bool {
            println!("Checking if player {} is in game {}", username, self.id.0);
            true
        }
    }

    struct GameState {
        players: HashMap<String, Player>,
        games: HashMap<GameId, Game>,
    }

    struct GameServer {
        players: Mutex<HashMap<String, Player>>,
        games: Mutex<HashMap<GameId, Game>>,
    }

    impl GameServer {
        fn add_and_join(&self, username: &str, info: Player) -> Option<GameId> {
            {
                let mut players = self.players.lock().unwrap();
                players.insert(username.to_owned(), info);
            }
            {
                let mut games = self.games.lock().unwrap();
                for (id, game) in games.iter_mut() {
                    if game.add_player(username) {
                        return Some(id.clone());
                    }
                }
            }
            None
        }

        fn ban_player(&self, username: &str) {
            {
                let mut games = self.games.lock().unwrap();
                games
                    .iter_mut()
                    .filter(|(_id, g)| g.has_player(username))
                    .for_each(|(_id, g)| g.remove_player(username));
            }

            {
                let mut players = self.players.lock().unwrap();
                players.remove(username);
            }
        }
    }
}
