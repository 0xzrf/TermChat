pub fn print_minimal_welcome() {
    const BOLD: &str = "\x1b[1m";
    const CYAN: &str = "\x1b[36m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    println!("{}{}🎯 TermChat{} - Ready to connect!", BOLD, CYAN, RESET);
    println!(
        "{}Type /help for commands or /join <room> to get started{}",
        GREEN, RESET
    );
    println!(
        "{}─────────────────────────────────────────────────{}",
        CYAN, RESET
    );
    println!();
    print_help();
}

pub fn print_help() {
    const BOLD: &str = "\x1b[1m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";
    const BLUE: &str = "\x1b[34m";

    println!("{BOLD}{BLUE}📋 Quick Commands:{RESET}");
    println!("{YELLOW}   /join <room>     - Join a chat room");
    println!("   /create <room>   - Create a new room");
    println!("   /set_user        - Set the username(default to terminal user){RESET}");
    // println!("   /list            - List available rooms");
    println!("   /leave           - Leave current room");
    println!("   /help            - Show all commands");
    println!("   /quit            - Exit the application{RESET}");
}
