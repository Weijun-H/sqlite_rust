use sqlite_rust::input_buffer::InputBuffer;
use sqlite_rust::{Error, Result};

fn main() -> Result<()> {
    let mut buffer = InputBuffer::new();
    welcome_message();
    while true {
        buffer.print_prompt();
        buffer.read_input()?;
        if buffer.get_buffer() == ".exit" {
            break;
        } else {
            println!("Unrecognized command '{}'", buffer.get_buffer());
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
