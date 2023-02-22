use std::collections::HashMap;

// Eq requires that you derive PartialEq on the type.
// Eqトレイトを使用する時は、PartialEqをderiveする必要があります。
#[derive(PartialEq, Eq, Hash)]
struct Account {
    username: String,
    password: String,
}

struct AccountInfo {
    name: String,
    email: String,
}

type Accounts = HashMap<Account, AccountInfo>;

fn try_logon(accounts: &Accounts, username: &str, password: &str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username: String::from(username),
        password: String::from(password),
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed!"),
    }
}

fn main() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman".to_string(),
        password: "password123".to_string(),
    };

    let account_info = AccountInfo {
        name: "John Everyman".to_string(),
        email: "j.everyman@email.com".to_string(),
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "psasword123");

    try_logon(&accounts, "j.everyman", "password123");
}
