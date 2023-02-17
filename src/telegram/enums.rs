use crate::telegram::enums::Command::{ChooseForecastPreferences, DeleteNotification, SetLocation, SetNotificationTime, Weather};

#[derive(PartialEq, Debug)]
pub enum Command {
    Weather,
    SetLocation,
    ChooseForecastPreferences,
    SetNotificationTime,
    DeleteNotification,
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::Weather => "Weather",
            Command::SetLocation => "SetLocation",
            Command::ChooseForecastPreferences => "ChooseForecastPreferences",
            Command::SetNotificationTime => "SetNotificationTime",
            Command::DeleteNotification => "DeleteNotification",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Weather => String::from("Weather"),
            Command::SetLocation => String::from("SetLocation"),
            Command::ChooseForecastPreferences => String::from("ChooseForecastPreferences"),
            Command::SetNotificationTime => String::from("SetNotificationTime"),
            Command::DeleteNotification => String::from("DeleteNotification"),
        }
    }

    pub fn from_str(str: &str) -> Option<Command> {
        match str {
            "Weather" => Some(Command::Weather),
            "SetLocation" => Some(Command::SetLocation),
            "ChooseForecastPreferences" => Some(Command::ChooseForecastPreferences),
            "SetNotificationTime" => Some(Command::SetNotificationTime),
            "DeleteNotification" => Some(Command::DeleteNotification),
            _ => None
        }
    }

    pub fn values(&self) -> Vec<Command>{
        vec![Weather, SetLocation, ChooseForecastPreferences, SetNotificationTime, DeleteNotification]
    }
}