use sqlite_rust::input_buffer::InputBuffer;
use sqlite_rust::meta_command::do_meta_command;
use sqlite_rust::meta_command::execute_statement;
use sqlite_rust::meta_command::parse_statement;
use sqlite_rust::meta_command::MetaCommandResult;
use sqlite_rust::{Error, Result};

fn main() -> Result<()> {
    let mut buffer = InputBuffer::new();
    let mut table = sqlite_rust::page::Table::new();
    welcome_message();
    while true {
        buffer.print_prompt();
        buffer.read_input()?;
        if buffer.get_buffer().starts_with(".") {
            match do_meta_command(&mut buffer) {
                Ok(MetaCommandResult::MetaCommandSuccess) => {
                    continue;
                }
                Ok(MetaCommandResult::MetaCommandUnrecognizedCommand) => {
                    println!("Unrecognized command '{}'", buffer.get_buffer());
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        match parse_statement(&mut buffer) {
            Ok(statement) => {
                execute_statement(&statement, &mut table)?;
            }
            Err(e) => {
                println!("Error preparing statement: {}", e);
                continue;
            }
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
