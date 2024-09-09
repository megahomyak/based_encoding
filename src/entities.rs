pub struct Messages {
    pub items: Vec<Message>,
}

impl Messages {
    pub fn decode(input: ) -> Self {

    }
}

pub struct Character {
    pub content: u8,
}

pub struct TextMessageContent {
    pub items: Vec<Character>,
}

pub struct UserId {
    pub content: u64,
}

pub struct TextMessage {
    pub sender_id: UserId,
    pub receiver_id: UserId,
    pub content: TextMessageContent,
}

pub enum Message {
    Text(TextMessage),
    UserJoined(UserId),
    UserLeft(UserId),
}
