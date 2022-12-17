use std::{env};
use std::string::ToString;
use std::thread::sleep;
use std::time::Duration;
use crate::service::{get_weather, save_location};
use crate::telegram::model::{UpdateResponse, Update, InlineKeyboardButton, InlineKeyboardMarkup, SendMessageInlineMarkup, SendMessageReplyMarkup, KeyboardButton, ReplyKeyboardMarkup};

pub fn process_updates() {
    let mut update_offset = 0;
    let get_updates_url = format!("https://api.telegram.org/bot{}/getUpdates", env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN must be set"));
    let client = reqwest::blocking::Client::new();
    loop {
        let params = [("offset", update_offset.to_string())];
        let client_response = client.get(&get_updates_url).query(&params).send();
        if let Ok(response) = client_response {
            if let Ok(updates_response) = response.json::<UpdateResponse>() {
                for update in &updates_response.result {
                    on_update(&update);
                }
                if let Some(last) = updates_response.result.last() {
                    println!("update_offset incremented");
                    update_offset = last.update_id + 1;
                } else {
                    println!("No updates received")
                }
            }
        }
        sleep(Duration::from_millis(200));
    }
}

fn on_update(update: &Update) {
    println!("{:?}", update);
    if let Some(message) = &update.message {
        let chat_id = &message.chat.id;
        if let Some(text) = &message.text {
            match text.as_str() {
                "/menu" => {
                    send_message_inline(&menu(chat_id));
                    return;
                }
                _ => {}
            }
        }
        if let Some(location) = &message.location {
            if let Some(user) = &message.from {
                save_location(user.id, location.longitude, location.latitude);
            }
        }
    }
    if let Some(callback) = &update.callback_query {
        // let chat = &callback.message.chat.id;
        let chat_id =
            match &callback.message {
                Some(message) => &message.chat.id,
                _ => &0
            };
        if let Some(data) = &callback.data {
            match data.as_str() {
                "weather" => {
                    let weather = &get_weather(callback.from.id).unwrap();
                    if weather.len() > 4095 {
                        send_message_inline(&SendMessageInlineMarkup::new(*chat_id, weather[..4095].to_string()));
                    } else {
                        send_message_inline(&SendMessageInlineMarkup::new(*chat_id, weather.to_string()));
                    }
                    return;
                }
                "set_location" => {
                    send_message_reply(&set_location_btn(chat_id));
                    return;
                }
                _ => {}
            }
            return;
        }
    }
}

fn send_message_inline(message: &SendMessageInlineMarkup) {
    let client = reqwest::blocking::Client::new();
    let send_message_url = format!("https://api.telegram.org/bot{}/sendMessage", env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN must be set"));
    let _ = client.post(send_message_url).json(message).send();
}

fn send_message_reply(message: &SendMessageReplyMarkup) {
    let client = reqwest::blocking::Client::new();
    let send_message_url = format!("https://api.telegram.org/bot{}/sendMessage", env::var("TG_BOT_TOKEN").expect("TG_BOT_TOKEN must be set"));
    let _ = client.post(send_message_url).json(message).send();
}

fn menu(chat_id: &i32) -> SendMessageInlineMarkup {

    let weather_button_row
        = vec![InlineKeyboardButton::new("Погода".to_string(), Some("weather".to_string()))];
    let location_button_row
        = vec![InlineKeyboardButton::new("Установить локацию".to_string(), Some("set_location".to_string()))];
    let reply_markup = vec![weather_button_row, location_button_row];

    let mut message = SendMessageInlineMarkup::new(chat_id.clone(), "menu".to_string());
    message.reply_markup = Some(InlineKeyboardMarkup::new(reply_markup));
    message
}

fn set_location_btn(chat_id: &i32) -> SendMessageReplyMarkup {

    let location_button_row
        = vec![KeyboardButton::new("Отправить мое местоположение".to_string(), Some(true))];

    let reply_markup = vec![location_button_row];

    let mut message = SendMessageReplyMarkup::new(chat_id.clone(), "Геолокация".to_string());
    message.reply_markup = Some(ReplyKeyboardMarkup::new(reply_markup, Some(true)));
    message
}