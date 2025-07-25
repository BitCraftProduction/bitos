// BitOS is initialized
//Current level: 0

#[derive(Debug)]
enum Command{
    Shutdown,
    Reboot,
    Print(&'static str)
}

fn handle_command(cmd:Command){
    match  cmd {
        Command::Shutdown => println!("System is shutting down..."),
        Command::Reboot => println!("System is rebooting..."),
        Command::Print(msg)=>println!("Message: {msg}")
    }
}

fn main(){
    let cmd = Command::Print("Hello World!");
    handle_command(cmd);
}