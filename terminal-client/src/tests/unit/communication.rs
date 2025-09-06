#[cfg(test)]
mod communication_test {
    use terminal_client::Communication;

    #[test]
    fn test_build() {
        let test_user = "Test user".to_string();
        let com = Communication::build(test_user.clone());

        assert_eq!(com.user_name, test_user);
        assert_eq!(com.joined_room, None);
    }

    #[tokio::test]
    // #[ignore]
    async fn test_connect_server() {
        Communication::connect_server().await.unwrap();
    }
}
