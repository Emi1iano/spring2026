fn main() {
    
}
#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        return BankAccount {balance: initial_balance};
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount < 0 as f64 {return}
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount < 0 as f64 {return}
        if self.balance - amount < 0 as f64 {return}
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let bank = BankAccount::new(100 as f64);
        assert_eq!(bank.balance(), 100 as f64)
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut bank = BankAccount::new(100 as f64);
        assert_eq!(bank.balance(), 100 as f64);
        bank.deposit(25 as f64);
        assert_eq!(bank.balance(), 125 as f64);
        bank.deposit(-50 as f64);
        assert_eq!(bank.balance(), 125 as f64);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut bank = BankAccount::new(100 as f64);
        assert_eq!(bank.balance(), 100 as f64);
        bank.withdraw(25 as f64);
        assert_eq!(bank.balance(), 75 as f64);
        bank.withdraw(100 as f64);
        assert_eq!(bank.balance(), 75 as f64);
    }

    // Add more tests here
}