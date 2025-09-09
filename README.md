# Terminal Chat App

A simple Rust-based chat system with a server and terminal clients. Multiple users can connect, create rooms, and chat in real time.

---

## 📦 Requirements
- [Rust](https://www.rust-lang.org/tools/install) (for `cargo run`)
- The terminal client alias/command `cr` should be available (update this if you’re using a different client runner)

---

## 🚀 Setup & Usage

### 1. Clone the repository
```bash
git clone https://github.com/0xzrf/TermChat
cd TermChat
```

## 🚀 Start the sever
change directories to server and run it:
```bash
cd server && cargo run
```

leave the terminal open and start a new terminal

## 🚀 Start the terminal client
come to the terminal-client folder, and run it using:
```bash
cd terminal-client && cargo run
```

Open a new termina, and use the above command again to start another terminal client

## 💬 Communicate using terminal client
Write the following command in the first terminal client:
```bash
/create lodge
```

and in the second client, write:
```bash
/join lodge
```

Now, whatever message you send to the channel should appear in the second terminal

## 📖 Additional info
use the following command to see available options:
```bash
/help
```


