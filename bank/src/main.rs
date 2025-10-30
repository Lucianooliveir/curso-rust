#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        if self.balance - amount < 0 {
            println!("Falta saldo na conta! Pobre");
            self.balance
        } else {
            self.balance -= amount;
            self.balance
        }
    }

    fn summary(&self) -> String {
        format!("Holder: {}\nBalance: {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let account: Account = Account::new(1, String::from("eu"));
    let mut bank: Bank = Bank::new();

    bank.add_account(account);
    bank.accounts[0].deposit(50);
    bank.accounts[0].withdraw(30);
    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
