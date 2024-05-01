use std::{fmt::LowerExp, io::{self, Write}};
use druid::{AppLauncher, WindowDesc};

mod ticket_operations;
mod user_operations;

fn main() {
    let running: bool = true;    
    let mut users: Vec<user_operations::User> = vec![];
    
    while running {
        
        println!("[N]ew user. [D]raw row. [P]rint users.");
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
    
        let mut buffer: String = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => { 
                match buffer.to_lowercase().trim() {
                    "d" => {
                        let row: [u8;8] = ticket_operations::new_row(40);
                        for number in row {
                            print!("{} -", number);
                        }
                        println!("");
                    },
                    "n" => {
                        let mut name: String = String::new();
                        let mut age: String = String::new();
                        let mut email: String = String::new();
                        
                        println!("Name: ");
                        io::stdin().read_line(&mut name);
                        println!("Age: ");
                        io::stdin().read_line(&mut age);
                        println!("Email: ");
                        io::stdin().read_line(&mut email);
                        users.push(user_operations::new_user(name, email, age));
                        println!("New user created");
                    },
                    "p" => {
                        if users.len() > 0 {
                            for user in &users {
                            
                                println!("Name: {}", user.get_name());
                                println!("Age: {}", user.get_age());
                                println!("E-mail: {}", user.get_email());
                                println!("");
                            }    
                        }
                        else {
                            println!("No users registered");
                        }
                    }
                    _ => println!("Unknown command"),
                }
             }
            Err(e)=> { println!("{e}") },
        }

    }
    }
