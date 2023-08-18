use actix_ws::Message;
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use tokio::{pin, sync::mpsc, time::interval, 
    task::{spawn, spawn_local},
    try_join,
    sync::oneshot
};
use actix_files::NamedFile;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use std::{
    collections::{HashMap, HashSet},
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant}
};
use rand::{thread_rng, Rng as _};









/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn chat_ws_handle(
    chat_server: ChatServerHandle,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    log::info!("connected");

    let mut name = None;
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    // unwrap: chat server is not dropped before the HTTP server
    let conn_id = chat_server.connect(conn_tx).await;

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()

        let tick = interval.tick();
        pin!(tick);

        let msg_rx = conn_rx.recv();
        pin!(msg_rx);

        // TODO: nested select is pretty gross for readability on the match
        let messages = select(msg_stream.next(), msg_rx);
        pin!(messages);

        match select(messages, tick).await {
            // commands & messages received from client
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                log::debug!("msg: {msg:?}");

                match msg {
                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // unwrap:
                        session.pong(&bytes).await.unwrap();
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Text(text) => {
                        process_text_msg(&chat_server, &mut session, &text, conn_id, &mut name)
                            .await;
                    }

                    Message::Binary(_bin) => {
                        log::warn!("unexpected binary message");
                    }

                    Message::Close(reason) => break reason,

                    _ => {
                        break None;
                    }
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                log::error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            // chat messages received from other room participants
            Either::Left((Either::Right((Some(chat_msg), _)), _)) => {
                session.text(chat_msg).await.unwrap();
            }

            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connection message senders were dropped; chat server may have panicked"
            ),

            // heartbeat internal tick
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    log::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );
                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        };
    };

    chat_server.disconnect(conn_id);

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}

async fn process_text_msg(
    chat_server: &ChatServerHandle,
    session: &mut actix_ws::Session,
    text: &str,
    conn: ConnId,
    name: &mut Option<String>,
) {
    // strip leading and trailing whitespace (spaces, newlines, etc.)
    let msg = text.trim();

    // we check for /<cmd> type of messages
    if msg.starts_with('/') {
        let mut cmd_args = msg.splitn(2, ' ');

        // unwrap: we have guaranteed non-zero string length already
        match cmd_args.next().unwrap() {
            "/list" => {
                log::info!("conn {conn}: listing rooms");

                let rooms = chat_server.list_rooms().await;

                for room in rooms {
                    session.text(room).await.unwrap();
                }
            }

            "/join" => match cmd_args.next() {
                Some(room) => {
                    log::info!("conn {conn}: joining room {room}");

                    chat_server.join_room(conn, room).await;

                    session.text(format!("joined {room}")).await.unwrap();
                }

                None => {
                    session.text("!!! room name is required").await.unwrap();
                }
            },

            "/name" => match cmd_args.next() {
                Some(new_name) => {
                    log::info!("conn {conn}: setting name to: {new_name}");
                    name.replace(new_name.to_owned());
                }
                None => {
                    session.text("!!! name is required").await.unwrap();
                }
            },

            _ => {
                session
                    .text(format!("!!! unknown command: {msg}"))
                    .await
                    .unwrap();
            }
        }
    } else {
        // prefix message with our name, if assigned
        let msg = match name {
            Some(ref name) => format!("{name}: {msg}"),
            None => msg.to_owned(),
        };

        chat_server.send_message(conn, msg).await
    }
}

/// Connection ID.
pub type ConnId = usize;

/// Room ID.
pub type RoomId = String;

/// Message sent to a room/client.
pub type Msg = String;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Handshake and start WebSocket handler with heartbeats.
async fn chat_ws_main(
    req: HttpRequest,
    stream: web::Payload,
    chat_server: web::Data<ChatServerHandle>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(chat_ws_handle(
        (**chat_server).clone(),
        session,
        msg_stream,
    ));

    Ok(res)
}

// note that the `actix` based WebSocket handling would NOT work under `tokio::main`
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    let (chat_server, server_tx) = ChatServer::new();

    let chat_server = spawn(chat_server.run());

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_tx.clone()))
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket routes
            .service(web::resource("/ws").route(web::get().to(chat_ws_main)))
            // enable logger
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run();

    try_join!(http_server, async move { chat_server.await.unwrap() })?;

    Ok(())
}

/// A command received by the [`ChatServer`].
#[derive(Debug)]
enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
    },

    Disconnect {
        conn: ConnId,
    },

    List {
        res_tx: oneshot::Sender<Vec<RoomId>>,
    },

    Join {
        conn: ConnId,
        room: RoomId,
        res_tx: oneshot::Sender<()>,
    },

    Message {
        msg: Msg,
        conn: ConnId,
        res_tx: oneshot::Sender<()>,
    },
}

/// A multi-room chat server.
///
/// Contains the logic of how connections chat with each other plus room management.
///
/// Call and spawn [`run`](Self::run) to start processing commands.
#[derive(Debug)]
pub struct ChatServer {
    /// Map of connection IDs to their message receivers.
    sessions: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,

    /// Map of room name to participant IDs in that room.
    rooms: HashMap<RoomId, HashSet<ConnId>>,

    /// Tracks total number of historical connections established.
    visitor_count: Arc<AtomicUsize>,

    /// Command receiver.
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl ChatServer {
    pub fn new() -> (Self, ChatServerHandle) {
        // create empty server
        let mut rooms = HashMap::with_capacity(4);

        // create default room
        rooms.insert("main".to_owned(), HashSet::new());

        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                rooms,
                visitor_count: Arc::new(AtomicUsize::new(0)),
                cmd_rx,
            },
            ChatServerHandle { cmd_tx },
        )
    }

    /// Send message to users in a room.
    ///
    /// `skip` is used to prevent messages triggered by a connection also being received by it.
    async fn send_system_message(&self, room: &str, skip: ConnId, msg: impl Into<String>) {
        if let Some(sessions) = self.rooms.get(room) {
            let msg = msg.into();

            for conn_id in sessions {
                if *conn_id != skip {
                    if let Some(tx) = self.sessions.get(conn_id) {
                        // errors if client disconnected abruptly and hasn't been timed-out yet
                        let _ = tx.send(msg.clone());
                    }
                }
            }
        }
    }

    /// Send message to all other users in current room.
    ///
    /// `conn` is used to find current room and prevent messages sent by a connection also being
    /// received by it.
    async fn send_message(&self, conn: ConnId, msg: impl Into<String>) {
        if let Some(room) = self
            .rooms
            .iter()
            .find_map(|(room, participants)| participants.contains(&conn).then_some(room))
        {
            self.send_system_message(room, conn, msg).await;
        };
    }

    /// Register new session and assign unique ID to this session
    async fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>) -> ConnId {
        log::info!("Someone joined");

        // notify all users in same room
        self.send_system_message("main", 0, "Someone joined").await;

        // register session with random connection ID
        let id = thread_rng().gen::<usize>();
        self.sessions.insert(id, tx);

        // auto join session to main room
        self.rooms
            .entry("main".to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_system_message("main", 0, format!("Total visitors {count}"))
            .await;

        // send id back
        id
    }

    /// Unregister connection from room map and broadcast disconnection message.
    async fn disconnect(&mut self, conn_id: ConnId) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove sender
        if self.sessions.remove(&conn_id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&conn_id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        // send message to other users
        for room in rooms {
            self.send_system_message(&room, 0, "Someone disconnected")
                .await;
        }
    }

    /// Returns list of created room names.
    fn list_rooms(&mut self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }

    /// Join room, send disconnect message to old room send join message to new room.
    async fn join_room(&mut self, conn_id: ConnId, room: String) {
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&conn_id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_system_message(&room, 0, "Someone disconnected")
                .await;
        }

        self.rooms
            .entry(room.clone())
            .or_insert_with(HashSet::new)
            .insert(conn_id);

        self.send_system_message(&room, conn_id, "Someone connected")
            .await;
    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::List { res_tx } => {
                    let _ = res_tx.send(self.list_rooms());
                }

                Command::Join { conn, room, res_tx } => {
                    self.join_room(conn, room).await;
                    let _ = res_tx.send(());
                }

                Command::Message { conn, msg, res_tx } => {
                    self.send_message(conn, msg).await;
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }
}

/// Handle and command sender for chat server.
///
/// Reduces boilerplate of setting up response channels in WebSocket handlers.
#[derive(Debug, Clone)]
pub struct ChatServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl ChatServerHandle {
    /// Register client message sender and obtain connection ID.
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<String>) -> ConnId {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Connect { conn_tx, res_tx })
            .unwrap();

        // unwrap: chat server does not drop out response channel
        res_rx.await.unwrap()
    }

    /// List all created rooms.
    pub async fn list_rooms(&self) -> Vec<String> {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx.send(Command::List { res_tx }).unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap()
    }

    /// Join `room`, creating it if it does not exist.
    pub async fn join_room(&self, conn: ConnId, room: impl Into<String>) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Join {
                conn,
                room: room.into(),
                res_tx,
            })
            .unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap();
    }

    /// Broadcast message to current room.
    pub async fn send_message(&self, conn: ConnId, msg: impl Into<String>) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Message {
                msg: msg.into(),
                conn,
                res_tx,
            })
            .unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap();
    }

    /// Unregister message sender and broadcast disconnection message to current room.
    pub fn disconnect(&self, conn: ConnId) {
        // unwrap: chat server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { conn }).unwrap();
    }
}