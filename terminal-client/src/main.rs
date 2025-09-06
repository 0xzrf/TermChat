use terminal_client::run;

mod tests;
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
    }
}
