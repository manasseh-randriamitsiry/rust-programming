fn main(){
    let x = 6;
    let y = &x;
    // cannot make operations with y because it a reference
    println!("x = {}, y = {}", x, y);

    // to solve it : use a mutable reference
    let mut z = 10;
    let r = &mut z;
    *r  += 100;
    println!("z = {}", z);

    // typo: cano only have 1 mutable reference
    // and unlimites immutable references
    // having like *r += 15 will fail

    let mut account = BankAccount{
        owner: String::from("John"),
        balance: 100.0,
    };

    // immutable borrow
    account.checking_balance();
    account.withdraw(50.0);
    account.checking_balance();
    account.withdraw(20.0);
    account.checking_balance();

}

pub struct BankAccount {
    owner: String,
    balance: f64,
}


impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}", amount, self.owner);
        self.balance -= amount;
    }

    fn checking_balance(&self) {
        println!("The balance of {} is {}", self.owner, self.balance);
    }
}