use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use rand::Rng;
use terminal_client::Communication;

async fn get_stream_and_room() -> (OwnedReadHalf, OwnedWriteHalf, String) {
    let stream = Communication::connect_server().await.unwrap();

    let (read, writer) = stream.into_split();

    let mut rng = rand::rng();
    let num: i32 = rng.random_range(1..=1000);

    let room = format!("Room{num}");

    (read, writer, room)
}

enum JsonType {
    Message,
    Create,
    Join,
}

fn get_send_json(json_type: JsonType, room: &str, from: Option<&str>, msg: Option<&str>) -> String {
    match json_type {
        JsonType::Create => {
            serde_json::json!({
                "type": "CreateRoom",
                "room": room,
            })
            .to_string()
                + "\n"
        }
        JsonType::Join => {
            serde_json::json!({
                "type": "JoinRoom",
                "room": room,
            })
            .to_string()
                + "\n"
        }
        JsonType::Message => {
            assert!(from.is_some() && msg.is_some());

            serde_json::json!({
                "type": "Message",
                "room": room.trim(),
                "from": from.unwrap(),
                "text": msg.unwrap().trim()
            })
            .to_string()
                + "\n"
        }
    }
}

#[cfg(test)]
mod communication_tests {
    use super::*;
    use std::sync::Arc;
    use terminal_client::Messages;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_create_after_connect_successfully() {
        let (mut read, writer, room) = get_stream_and_room().await;

        let writer_lock = Arc::new(RwLock::new(writer));
        let create_json = get_send_json(JsonType::Create, &room, None, None);

        Communication::send_msg(create_json, Arc::clone(&writer_lock))
            .await
            .unwrap();

        let msg = Communication::read_msg(&mut read).await.unwrap();

        if let Messages::Created { room: created_room } = msg {
            assert_eq!(created_room, room);
        }
    }

    #[tokio::test]
    async fn fails_when_room_doesnt_exist() {
        let (mut read, writer, room) = get_stream_and_room().await;

        let writer_lock = Arc::new(RwLock::new(writer));

        let create_json = get_send_json(JsonType::Join, &room, None, None);

        Communication::send_msg(create_json, Arc::clone(&writer_lock))
            .await
            .unwrap();

        let msg = Communication::read_msg(&mut read).await.unwrap();

        if let Messages::Error { msg } = msg {
            assert!(msg.contains("does not exist"));
        }
    }

    #[tokio::test]
    async fn successfully_sends_and_receive_msgs() {
        // Creating 2 streams for 2 user connections
        let (_, writer_user1, room) = get_stream_and_room().await;
        let (mut read_user2, writer_user2, _) = get_stream_and_room().await;
        let user_1 = "User1";
        let user_1_msg = "Hey User2!";

        let writer_lock1 = Arc::new(RwLock::new(writer_user1));
        let writer_lock2 = Arc::new(RwLock::new(writer_user2));

        let create_json = get_send_json(JsonType::Create, &room, None, None);

        // create room
        Communication::send_msg(create_json, Arc::clone(&writer_lock1))
            .await
            .unwrap();

        // make the 2 users join the room
        let join_json = get_send_json(JsonType::Join, &room, None, None);
        let join_json_clone = join_json.clone();
        Communication::send_msg(join_json, Arc::clone(&writer_lock1))
            .await
            .unwrap();

        Communication::send_msg(join_json_clone, Arc::clone(&writer_lock2))
            .await
            .unwrap();

        // Ignore the join message received by the server for user2 to get the message from the server
        let _ = Communication::read_msg(&mut read_user2).await.unwrap();

        // send message from user_1 to the room
        let message_user_1 =
            get_send_json(JsonType::Message, &room, Some(user_1), Some(user_1_msg));

        println!("{message_user_1}");

        Communication::send_msg(message_user_1, Arc::clone(&writer_lock1))
            .await
            .unwrap();

        // check if the message received is by the correct user and the right text
        let msg_received = Communication::read_msg(&mut read_user2).await.unwrap();

        if let Messages::Message { from, text } = msg_received {
            assert_eq!(&from, user_1);
            assert_eq!(&text, user_1_msg);
        } else {
            panic!()
        }

        // gracefully closing the connection
        drop(writer_lock1);
        drop(writer_lock2);
    }
}
