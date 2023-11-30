use bincode::{config, Decode, Encode};

pub type RoomName = String;
pub type RoomId = i64;
pub type Message = Vec<u8>;

#[derive(Encode, Decode, PartialEq, Debug)]
pub enum Command {
    CreateRoom(RoomName),
    JoinRoom(RoomId),
    LeaveRoom(RoomId),
    SendMessage(RoomId, Message),
}

pub fn encode(cmd: Command) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::encode_to_vec(&cmd, config::standard())?)
}

pub fn decode(encoded: Vec<u8>) -> anyhow::Result<Command> {
    let (decoded, _) = bincode::decode_from_slice(&encoded[..], config::standard())?;
    Ok(decoded)
}
