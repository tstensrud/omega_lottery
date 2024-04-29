use std::io::{self, Write};


mod ticket_operations;

fn main() {
    let mut running: bool = true;
    let max_numbers: i8 = 42;
    let mut count = 0;
    
    while running {
        read_and_handle();
        println!("{}", count);
        count += 1;
        if count > 3 {
            running = false;
        }

    }

    let mut row_of_numbers = ticket_operations::new_row(max_numbers);
    for (i, num) in row_of_numbers.iter().enumerate() {
        if i == row_of_numbers.len() - 1 {
            print!(" {}", num);    
        }
        else {
            print!("{} -", num);
        }
    }
}

fn read_and_handle() {
    print!("Write something: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut buffer: String = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => { println!("You said: {}", buffer.trim()); }
        Err(e)=> { println!("{e}") },
    }
}

