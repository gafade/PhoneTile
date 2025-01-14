use super::packet;
use std::fmt::{write, Display};
use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::{thread, time};

/// All of those functions are completely non-blocking

//////////////////////////////////////////////
///
///
/// Game flag
///
///
//////////////////////////////////////////////

#[derive(Clone, Copy)]
pub enum Game {
    Racer,
    Snake,
    MazeFight,
    Ping,
    Test,
    Unknown,
}

impl From<Game> for u16 {
    fn from(value: Game) -> Self {
        match value {
            Game::Racer => 1,
            Game::Snake => 2,
            Game::MazeFight => 3,
            Game::Ping => 4,
            Game::Test => 0x80,
            Game::Unknown => 0xff,
        }
    }
}

impl From<u16> for Game {
    fn from(value: u16) -> Self {
        match value {
            1 => Game::Racer,
            2 => Game::Snake,
            3 => Game::MazeFight,
            4 => Game::Ping,
            0x80 => Game::Test,
            _ => Game::Unknown,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Game::Racer => write!(f, "Racer"),
            Game::Snake => write!(f, "Snake"),
            Game::MazeFight => write!(f, "Maze-Fight"),
            Game::Ping => write!(f, "Ping"),
            Game::Test => write!(f, "Test"),
            Game::Unknown => write!(f, "Unknown"),
        }
    }
}

//////////////////////////////////////////////
///
///
/// Status flag
///
///
//////////////////////////////////////////////

#[derive(Clone)]
pub enum Status {
    Connected,
    Disconnected,
    InRoom,
    InLockRoom(u8),
    InGame,
}

//////////////////////////////////////////////
///
///
/// Network structure
///
///
//////////////////////////////////////////////

pub struct Network {
    stream: TcpStream,
    session_token: u16,
    room_token: u16,
    status: Status,
}

impl Network {
    //////////////////////////////////////////////
    ///
    ///
    /// Pre-game
    ///
    ///
    //////////////////////////////////////////////

    /// Connect to the server, you must do this action BEFORE ANYTHING ELSE
    pub fn connect(
        physical_height: f32,
        physical_width: f32,
        window_height: u32,
        window_width: u32,
    ) -> Result<Self, Error> {
        match TcpStream::connect("127.0.0.1:8888") {
            Ok(stream) => {
                stream.set_nonblocking(true)?;
                let mut network = Network {
                    stream,
                    session_token: 0,
                    room_token: 0,
                    status: Status::Connected,
                };
                network.init_handshake(
                    physical_height,
                    physical_width,
                    window_height,
                    window_width,
                )?;
                Ok(network)
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotConnected,
                "unable to connect to the server",
            )),
        }
    }

    /// Create a room and send back the ID of the room in order for the other
    /// to connect themselves to it
    pub fn create_room(&mut self) -> Result<u16, Error> {
        let packet_room_creation =
            packet::Packet::new(packet::Flag::Create, 0, self.session_token, 0, &[], 0);
        packet_room_creation.send_packet(&mut self.stream)?;

        let packet = packet::Packet::recv_packet(&mut self.stream)?;
        self.room_token = packet.room;
        self.status = Status::InRoom;
        Ok(packet.room)
    }

    /// Join a room with the given room ID
    pub fn join_room(&mut self, room_token: u16) -> Result<(), Error> {
        packet::Packet::new(
            packet::Flag::Join,
            0,
            self.session_token,
            room_token,
            &[],
            0,
        )
        .send_packet(&mut self.stream)?;

        let packet = packet::Packet::recv_packet(&mut self.stream)?;
        self.room_token = packet.room;
        self.status = Status::InRoom;

        Ok(())
    }

    /// Lock the room, so that no more user can join the room
    /// The position of each user is given from this point when the get_status is triggered
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn lock_room(&mut self, game_id: Game) -> Result<(), Error> {
        packet::Packet::new(
            packet::Flag::Lock,
            0,
            self.session_token,
            self.room_token,
            &[],
            game_id.into(),
        )
        .send_packet(&mut self.stream)
    }

    /// Launch the actual game
    /// THIS FUNCTION WILL WORK ONLY IF create_room HAS BEEN CALLED BEFORE THAT
    pub fn launch_game(&mut self) -> Result<(), Error> {
        match packet::Packet::new(
            packet::Flag::Launch,
            0,
            self.session_token,
            self.room_token,
            &[],
            0,
        )
        .send_packet(&mut self.stream)
        {
            Ok(_) => {
                self.status = Status::InGame;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    //////////////////////////////////////////////
    ///
    ///
    /// In-game
    ///
    ///
    //////////////////////////////////////////////

    /// Send data to the server ; this action can only be done in game
    /// If you use this function outisde of a game, this will simply discard the message
    pub fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        packet::Packet::new(packet::Flag::Transmit, 0, self.session_token, 0, data, 0)
            .send_packet(&mut self.stream)
    }

    /// Receive data from the server ; this action can only be done in game
    /// It return the amount of data read
    pub fn recv(&mut self, buffer: &mut [u8; packet::MAX_DATA_SIZE]) -> bool {
        match packet::Packet::try_recv_packet(&mut self.stream) {
            Some(packet) => {
                buffer.copy_from_slice(&packet.data);
                true
            }
            None => false,
        }
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Continuous
    ///
    ///
    //////////////////////////////////////////////

    /// Get the current status of the network
    pub fn get_status(&mut self) -> Status {
        match self.status {
            Status::InRoom => match packet::Packet::try_recv_packet(&mut self.stream) {
                Some(packet) => {
                    self.status = Status::InLockRoom(packet.option as u8);
                    self.status.clone()
                }
                None => self.status.clone(),
            },
            Status::InLockRoom(_) => match packet::Packet::try_recv_packet(&mut self.stream) {
                Some(_) => {
                    self.status = Status::InGame;
                    self.status.clone()
                }
                None => self.status.clone(),
            },
            _ => self.status.clone(),
        }
    }

    //////////////////////////////////////////////
    ///
    ///
    /// Helpers
    ///
    ///
    //////////////////////////////////////////////

    fn init_handshake(
        &mut self,
        physical_height: f32,
        physical_width: f32,
        window_height: u32,
        window_width: u32,
    ) -> Result<(), Error> {
        let mut data = [0_u8; 16];
        data[..4].copy_from_slice(&physical_height.to_be_bytes());
        data[4..8].copy_from_slice(&physical_width.to_be_bytes());
        data[8..12].copy_from_slice(&window_height.to_be_bytes());
        data[12..16].copy_from_slice(&window_width.to_be_bytes());

        packet::Packet::new(packet::Flag::Init, 0, 0, 0, &data, 0).send_packet(&mut self.stream)?;

        let packet = packet::Packet::recv_packet(&mut self.stream)?;
        self.session_token = packet.session;
        Ok(())
    }
}
