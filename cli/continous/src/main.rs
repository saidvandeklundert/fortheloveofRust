use std::io::{stdin, Read};
use std::io::stdout;
use crossterm::{
    cursor::{self, Hide},
    event::{poll, read, Event, KeyCode},
    terminal,
    ExecutableCommand,
};

#[derive(Debug)]
struct EthernetPort {
    speed: String,
    index: String,
    mtu: String,
    alias: String,
    //preemphasis: Option<String>,
    admin_status: String,
    oper_status: String,
    lanes: String,
    autoneg: String,
    description: String,
}

fn main() -> crossterm::Result<()> {
    // Enable raw mode to read individual key presses
    terminal::enable_raw_mode()?;
    //cursor::Hide(cursor::HideCursor)?;
    // Clear the screen
    stdout().execute(Hide)?;
    print!("{}", terminal::Clear(terminal::ClearType::All));

    loop {

        print!("{}", cursor::MoveTo(0, 0));


        print!("This is some state output from a networking device.\n");
        print!("The state is continously refreshed.\n");
        print!("Press q to quit the program\n");

        // Check for user input
        if poll(std::time::Duration::from_millis(1200))? {
            match read()? {
                Event::Key(event) => {
                    if event.code == KeyCode::Char('q') {
                        // Exit the loop if 'q' is pressed
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    // Restore the terminal to its original state
    terminal::disable_raw_mode()?;
    Ok(())
}