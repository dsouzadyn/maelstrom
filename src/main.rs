use maelstrom::MaelstromMessage;
use std::io;
fn main() -> io::Result<()> {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                let mut message = MaelstromMessage::parse(trimmed_input);
                message.eval();
                println!("{message}");
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }

    Ok(())
}
