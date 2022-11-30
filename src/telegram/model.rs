use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateResponse {
    pub ok: bool,
    pub result: Vec<Update>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i32,
    pub from: Option<User>,
    pub text: Option<String>,
    pub chat: Chat,
    pub location: Option<Location>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub data: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageInlineMarkup {
    pub chat_id: i32,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageReplyMarkup {
    pub chat_id: i32,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyKeyboardMarkup>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {
pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
pub text: String,
pub callback_data: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub one_time_keyboard: Option<bool>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    pub text: String,
    pub request_location: Option<bool>
}

impl SendMessageInlineMarkup {
    pub fn new(chat_id: i32, text: String) -> SendMessageInlineMarkup {
        SendMessageInlineMarkup { chat_id, text, parse_mode: None, reply_markup: None }
    }
}

impl SendMessageReplyMarkup {
    pub fn new(chat_id: i32, text:String) -> SendMessageReplyMarkup {
        SendMessageReplyMarkup { chat_id, text, parse_mode: None, reply_markup:None }
    }
}

impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup { inline_keyboard }
    }
}

impl InlineKeyboardButton {
    pub fn new(text: String, callback_data: Option<String>) -> InlineKeyboardButton {
        InlineKeyboardButton { text, callback_data }
    }
}

impl ReplyKeyboardMarkup {
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>, one_time_keyboard: Option<bool>) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup { keyboard, one_time_keyboard }
    }
}

impl KeyboardButton {
    pub fn new(text: String, request_location: Option<bool>) -> KeyboardButton {
        KeyboardButton { text, request_location }
    }
}