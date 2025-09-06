#[cfg(test)]
mod helper_tests {

    use terminal_client::helper_prelude::io::*;
    use terminal_client::print_minimal_welcome;

    // Testing if the print helper function successfully run
    #[test]
    fn test_print_right() {
        print_right("Hello world");
    }

    #[test]
    fn test_print_center() {
        print_center("Hello world");
    }
    #[test]
    fn test_print_welcome() {
        print_minimal_welcome();
    }
}
