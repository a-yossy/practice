macro_rules! complain {
    (msg : $msg:expr) => {
        println!("Complaint filed: {}", $msg);
    };
    (user : $userid:tt , msg : $msg:expr) => {
        println!("Complaint from user {}: {}", $userid, $msg);
    };
}

fn main() {
    complain!(user: "jimb", msg: "the AI lab's chatbots keep picking on me");
}
