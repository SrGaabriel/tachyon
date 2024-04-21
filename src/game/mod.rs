pub mod identifier;
pub mod entity;
pub mod math;
pub mod text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMode {
    Full,
    CommandsOnly,
    Hidden
}

impl ChatMode {
    pub fn accepts_chat(&self, chat: MessageType) -> bool {
        match self {
            ChatMode::Full => true,
            ChatMode::CommandsOnly => chat == MessageType::System,
            ChatMode::Hidden => false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Player,
    DisguisedPlayer,
    System
}