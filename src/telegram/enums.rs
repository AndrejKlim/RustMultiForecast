use crate::telegram::enums::Command::{ChooseForecastPreferences, SetLocation, Weather};

#[derive(PartialEq, Debug)]
pub enum Command {
    Weather,
    SetLocation,
    ChooseForecastPreferences
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        match self {
            Command::Weather => "Weather",
            Command::SetLocation => "SetLocation",
            Command::ChooseForecastPreferences => "ChooseForecastPreferences",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Weather => String::from("Weather"),
            Command::SetLocation => String::from("SetLocation"),
            Command::ChooseForecastPreferences => String::from("ChooseForecastPreferences"),
        }
    }

    pub fn from_str(str: &str) -> Option<Command> {
        match str {
            "Weather" => Some(Command::Weather),
            "SetLocation" => Some(Command::SetLocation),
            "ChooseForecastPreferences" => Some(Command::ChooseForecastPreferences),
            _ => None
        }
    }

    pub fn values(&self) -> Vec<Command>{
        vec![Weather, SetLocation, ChooseForecastPreferences]
    }
}