use sqlite_rust::input_buffer::InputBuffer;
use sqlite_rust::meta_command::execute_query;
use sqlite_rust::page::Table;
use sqlite_rust::{Error, Result};

fn main() -> Result<()> {
    let mut table = Table::new();
    welcome_message();
    while true {
        let mut buffer = InputBuffer::new();
        match execute_query(&mut buffer, &mut table) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
    Ok(())
}

fn welcome_message() {
    println!("Welcome to SQLite-RUST!");
    println!(r"                                                                 ");
    println!(r"    _____ ____    __    _ __             ____  __  _____________ ");
    println!(r"   / ___// __ \  / /   (_) /____        / __ \/ / / / ___/_  __/ ");
    println!(r"   \__ \/ / / / / /   / / __/ _ \______/ /_/ / / / /\__ \ / /    ");
    println!(r"  ___/ / /_/ / / /___/ / /_/  __/_____/ _, _/ /_/ /___/ // /     ");
    println!(r" /____/\___\_\/_____/_/\__/\___/     /_/ |_|\____//____//_/      ");
    println!(r"                                                                 ");
    println!(r"                                                                 ");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main().unwrap();
    }
}
