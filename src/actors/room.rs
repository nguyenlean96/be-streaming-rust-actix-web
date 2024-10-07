use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use crate::actors::chat::ChatSession;
use crate::actors::video::VideoStream;

// Room struct that holds the users
pub struct Room {
    id: String,
    users: HashSet<Addr<ChatSession>>, // Users in the room
    video_stream: Option<Addr<VideoStream>>, // Video stream address for the room
}

impl Room {
    // Create a new room
    pub fn new(id: String) -> Self {
        Self {
            id,
            users: HashSet::new(),
            video_stream: None,
        }
    }

    // Add a user to the room
    pub fn add_user(&mut self, user_addr: Addr<ChatSession>) {
        self.users.insert(user_addr);
    }

    // Remove a user from the room
    pub fn remove_user(&mut self, user_addr: &Addr<ChatSession>) {
        self.users.remove(user_addr);
    }

    // Start a video stream for the room
    pub fn start_video_stream(&mut self, stream_addr: Addr<VideoStream>) {
        self.video_stream = Some(stream_addr);
    }

    // Stop the video stream for the room
    pub fn stop_video_stream(&mut self) {
        self.video_stream = None;
    }
}

// RoomManager to manage multiple rooms
pub struct RoomManager {
    rooms: HashMap<String, Room>,
}

impl RoomManager {
    // Create a new RoomManager
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    // Create a new room
    pub fn create_room(&mut self, room_id: String) -> &mut Room {
        self.rooms.entry(room_id.clone()).or_insert_with(|| Room::new(room_id))
    }

    // Get an existing room
    pub fn get_room(&mut self, room_id: &String) -> Option<&mut Room> {
        self.rooms.get_mut(room_id)
    }

    // Remove an empty room
    pub fn remove_room(&mut self, room_id: &String) {
        self.rooms.remove(room_id);
    }
}

impl Actor for RoomManager {
    type Context = Context<Self>;
}

// Implement message handling for room management (optional for custom logic)
impl Handler<CreateRoom> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: CreateRoom, _ctx: &mut Self::Context) {
        self.create_room(msg.room_id);
    }
}

// Define messages to interact with rooms
pub struct CreateRoom {
    pub room_id: String,
}
impl Message for CreateRoom {
    type Result = ();
}