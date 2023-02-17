use std::{env};
use std::string::ToString;
use std::thread::sleep;
use std::time::Duration;
use log::{debug, trace};
use crate::enums::Field;
use crate::service::command::{get_last_user_command, save_last_user_command};
use crate::service::location::save_location;
use crate::service::preferences::save_forecast_preferences;
use crate::service::weather::get_weather;
use crate::telegram::enums::Command;
use crate::telegram::enums::Command::{ChooseForecastPreferences, SetLocation, Weather};
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
                    trace!("Update_offset incremented");
                    update_offset = last.update_id + 1;
                } else {
                    trace!("No updates received")
                }
            }
        }
        sleep(Duration::from_millis(200));
    }
}

fn on_update(update: &Update) {
    debug!("Update received {:?}", update);
    if let Some(message) = &update.message {

        let last_command = get_last_user_command(&message.from.as_ref().unwrap().id);
        debug!("Last command - {:?}", &last_command);
        let chat_id = &message.chat.id;
        if let Some(text) = &message.text {
            match text.as_str() {
                "/menu" => {
                    send_message_inline(&menu(chat_id));
                    return;
                }
                _ => {}
            }

            if ChooseForecastPreferences.to_string().eq(&last_command) {
                save_forecast_preferences(&text, &message.from.as_ref().unwrap().id);
                return;
            }
        }
        if let Some(location) = &message.location {
            if let Some(user) = &message.from {
                save_location(user.id, location.longitude, location.latitude);
            }
        }
    }
    if let Some(callback) = &update.callback_query {
        let chat_id =
            match &callback.message {
                Some(message) => &message.chat.id,
                _ => &0
            };
        if let Some(data) = &callback.data {
            match Command::from_str(data.as_str()).unwrap() {
                Weather => {
                    let weather = &get_weather(callback.from.id).unwrap();
                    if weather.len() > 4095 {
                        // TODO переделать на отправку нескольких сообщений
                        send_message_inline(&SendMessageInlineMarkup::new(*chat_id, weather[..4095].to_string()));
                    } else {
                        send_message_inline(&SendMessageInlineMarkup::new(*chat_id, weather.to_string()));
                    }
                    save_last_user_command(&callback.from.id, Weather.to_string());
                    return;
                }
                SetLocation => {
                    send_message_reply(&set_location_btn(chat_id));
                    save_last_user_command(&callback.from.id, SetLocation.to_string());
                    return;
                }
                ChooseForecastPreferences => {
                    send_message_inline(&SendMessageInlineMarkup::new(*chat_id, set_forecast_preferences_response()));
                    save_last_user_command(&callback.from.id, ChooseForecastPreferences.to_string());
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
        = vec![InlineKeyboardButton::new("Погода".to_string(), Some(Weather.to_string()))];
    let location_button_row
        = vec![InlineKeyboardButton::new("Установить локацию".to_string(), Some(SetLocation.to_string()))];
    let choose_forecast_preferences_row
        = vec![InlineKeyboardButton::new("Выбрать отображаемые погодные данные".to_string(), Some(ChooseForecastPreferences.to_string()))];
    let reply_markup = vec![weather_button_row, location_button_row, choose_forecast_preferences_row];

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

fn set_forecast_preferences_response() -> String {
    let preferences: Vec<String> = Field::values()
        .iter()
        .enumerate()
        .map(|(num, f)| {
            let mut temp = num.to_string();
            temp.push_str(". ");
            temp.push_str(f.as_ru_str());
            temp
        })
        .collect();
    let joined = preferences.join("\n");
    let mut text_to_show = String::from("Отправьте интересующие вас номера показателей через запятую. Например, 1,2,4\n");
    text_to_show.push_str(&joined);
    text_to_show
}