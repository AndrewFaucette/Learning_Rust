use std::io;
//use std::cmp::Ordering;

fn main() {
    let mut accounts = Vec::new();
    let mut passwords = Vec::new();

    loop {
        println!("        1. Access a password
        2. Add a password
        3. Remove a password
        4. Exit
        Input a number for the desired function: "
        );

        let mut menu_input = String::new();

        io::stdin().read_line(&mut menu_input).expect("Failed to read line");

        match menu_input.trim() {
            "1" => access_password(&accounts, &passwords),
            "2" => add_password(&mut accounts, &mut passwords),
            "3" => remove_password(&mut accounts, &mut passwords),
            "4" => break,
            _ => println!("Invalid input, please try again."),
        }
    }
}

fn access_password(accounts: &Vec<String>, passwords: &Vec<String>) {
    println!("Accessing a password...");
    for (index, account) in accounts.iter().enumerate() {
        println!("{}: {}", index + 1, account);
    }
    println!("Input the number of the account: ");
    let mut account_input = String::new();
    io::stdin().read_line(&mut account_input).expect("Failed to read line");
    let account_input: u32 = account_input.trim().parse().expect("Please type a number!");
    let index = account_input as usize - 1;
    match (accounts.get(index), passwords.get(index)) {
        (Some(account), Some(password)) => {
            println!("The password for {} is: {}", account, password);
        }
        _ => println!("Invalid account number."),
    }
}

fn add_password(accounts: &mut Vec<String>, passwords: &mut Vec<String>) {
    println!("Adding a password...");
    println!("Input the number of the account: ");
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).expect("Failed to read line");
    accounts.push(new_name.trim().to_string());
    println!("Input the password: ");
    let mut new_password = String::new();
    io::stdin().read_line(&mut new_password).expect("Failed to read line");
    passwords.push(new_password.trim().to_string());
}

fn remove_password(accounts: &mut Vec<String>, passwords: &mut Vec<String>) {
    println!("Removing a password...");
    for (index, account) in accounts.iter().enumerate() {
        println!("{}: {}", index + 1, account);
    }
    println!("Input the number of the account: ");
    let mut account_input = String::new();
    io::stdin().read_line(&mut account_input).expect("Failed to read line");
    let account_input: u32 = account_input.trim().parse().expect("Please type a number!");
    let index = account_input as usize - 1;
    accounts.remove(index);
    passwords.remove(index);
    println!("Password has been removed.");
}