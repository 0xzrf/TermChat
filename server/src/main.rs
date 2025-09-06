use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Error},
    net::{TcpListener, TcpStream},
    sync::{Mutex, RwLock, mpsc},
};
use uuid::Uuid;

type ClientId = Uuid;
type Tx = mpsc::UnboundedSender<ServerMessage>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientCommand {
    JoinRoom {
        room: String,
    },
    CreateRoom {
        room: String,
    },
    Message {
        room: String,
        text: String,
        from: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
enum ServerMessage {
    Joined {
        room: String,
    },
    Message {
        room: String,
        from: String,
        text: String,
    },
    Error {
        msg: String,
    },
    Created {
        room: String,
    },
}

struct Room {
    members: HashMap<ClientId, Tx>,
}

struct ServerState {
    rooms: HashMap<String, Arc<Mutex<Room>>>,
}

type SharedState = Arc<RwLock<ServerState>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut rooms = HashMap::new();
    rooms.insert(
        "lobby".to_string(),
        Arc::new(Mutex::new(Room {
            members: HashMap::new(),
        })),
    );
    let state = Arc::new(RwLock::new(ServerState { rooms }));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?;
        println!("A new incoming ");
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, state).await {
                eprintln!("client error: {e:?}");
            }
        });
    }
}

async fn handle_client(stream: TcpStream, state: SharedState) -> Result<(), Error> {
    let id = Uuid::new_v4();
    let (reader, mut writer) = stream.into_split();

    // channel to send messages to this client
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    // spawn writer task
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(line) = serde_json::to_string(&msg) {
                println!("Received message: {line}");
                let _ = writer.write_all(line.as_bytes()).await;
                let _ = writer.write_all(b"\n").await;
            }
        }
    });

    // read commands
    let mut reader = BufReader::new(reader).lines();

    while let Some(line) = reader.next_line().await? {
        let cmd: ClientCommand = match serde_json::from_str(&line) {
            Ok(c) => c,
            Err(_) => {
                println!("Couldn't call line: {line}");
                let _ = tx.send(ServerMessage::Error {
                    msg: "invalid command: {}".into(),
                });
                continue;
            }
        };

        match cmd {
            ClientCommand::CreateRoom { room } => {
                let mut state_guard = state.write().await;

                if state_guard.rooms.get(&room).is_some() {
                    let _ = tx.send(ServerMessage::Error {
                        msg: format!("Room: {room} already exist"),
                    });
                    break;
                }
                state_guard.rooms.insert(
                    room.clone(),
                    Arc::new(Mutex::new(Room {
                        members: HashMap::new(),
                    })),
                );
                let _ = tx.send(ServerMessage::Created { room });
            }
            ClientCommand::JoinRoom { room } => {
                println!("Joining room: {room}");
                let room_arc = {
                    let state_guard = state.write().await;

                    if state_guard.rooms.get(&room).is_none() {
                        let _ = tx.send(ServerMessage::Error {
                            msg: format!("Room: {} does not exist", room.trim()),
                        });
                        break;
                    }

                    state_guard.rooms.get(&room).unwrap().clone()
                };
                let mut room_guard = room_arc.lock().await;
                room_guard.members.insert(id, tx.clone());
                let _ = tx.send(ServerMessage::Joined { room });
            }
            ClientCommand::Message { room, text, from } => {
                println!("Sending message to room: {room}");
                let maybe_room = {
                    let state_guard = state.read().await;
                    state_guard.rooms.get(&room).cloned()
                };
                if let Some(room_arc) = maybe_room {
                    let room_guard = room_arc.lock().await;
                    for (other_id, member_tx) in room_guard.members.iter() {
                        if *other_id != id {
                            let _ = member_tx.send(ServerMessage::Message {
                                room: room.clone(),
                                from: from.to_string(),
                                text: text.clone(),
                            });
                        }
                    }
                } else {
                    println!("Room doesn't exist");
                    let _ = tx.send(ServerMessage::Error {
                        msg: format!("room {room} does not exist"),
                    });
                }
            }
        }
    }

    Ok(())
}
