// Implement a struct to represent a bank account with fields for account number, balance, and owner, and create methods to deposit and withdraw money.

#[allow(dead_code)]

struct Account {
    number: u64,
    owner: String,
    balance: u64,
}

impl Account {
    fn deposit(&mut self, amount: u64) {
        if amount == 0 {
            println!("Mentally healthy people never make a deposite of $0.");
            println!("We prey for you.");
            return;
        }

        self.balance += amount;
        println!("You deposited ${} and you are current balance became ${}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: u64) {
        if amount > self.balance {
            println!("Do not cheat the system! System is not your boyfriend/girlfriend.");
            println!("Be a good human.");
            return;
        }

        self.balance -= amount;
        println!("You made a withdraw of ${}, which leaves you with ${}.", amount, self.balance);
    }
}

fn main() {
    let mut harse: Account = Account {
        number: 1,
        owner: "Harse".to_string(),
        balance: 1000
    };

    harse.deposit(0);
    harse.withdraw(1200);
}
