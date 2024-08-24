#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    // associated function, does not take 'self'
    fn new(id: u32, holder: String) -> Self {
        Account {
            id: id, // since the var name is the same, we could have also written 'id'
            holder: holder,
            balance: 0,
        }
    }
    fn show(&self){
        println!("{:#?}", &self);
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount:i32)->i32{
        self.balance +=amount;
        self.balance
    }
    fn withdraw(&mut self, amount:i32)->i32{
        self.balance -=amount;
        self.balance
    }    
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    // associated function, does not take 'self'
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
    fn show(&self){
        println!("{:#?}", &self);
    }

    fn add_account(&mut self, account:Account){
        self.accounts.push(account);
    }

    fn total_balace(&self) -> i32 {
        self.accounts.iter().map(|account|account.balance).sum()
    }

    fn summary(&self)->Vec<String>{
        self.accounts
        .iter()
        .map(|account|account.summary())
        .collect::<Vec<String>>()
    }
}

fn print_account(account:Account){
    println!("{:#?}", account);
}


pub fn structs_example() {
    println!("Exampels on working with structs");
    let mut bank = Bank::new();    
    let mut account = Account::new(1, "Said".to_string());
    bank.show();
    bank.show();
    account.show();
    account.deposit(1000);
    bank.add_account(account);
    bank.show();
    println!("{}", bank.total_balace());
    
}
