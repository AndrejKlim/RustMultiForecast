pub enum Source {
    OpenWeather
}

pub enum Duration {
    Weather,
    Nearest,
    LongTerm,
    Multi
}

pub enum Field {
    Temperature,
    WindSpeed,
    Pressure,
    Humidity
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::OpenWeather => "OpenWeather"
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Source::OpenWeather => String::from("OpenWeather")
        }
    }

    pub fn from_str(str: &str) -> Option<Source> {
        match str {
            "OpenWeather" => Some(Source::OpenWeather),
            _ => None
        }
    }
}

impl Duration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Duration::Weather => "Weather",
            Duration::Nearest => "Nearest",
            Duration::LongTerm => "LongTerm",
            Duration::Multi => "Multi"
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Duration::Weather => String::from("Weather"),
            Duration::Nearest => String::from("Nearest"),
            Duration::LongTerm => String::from("LongTerm"),
            Duration::Multi => String::from("Multi")
        }
    }

    pub fn from_str(str: &str) -> Option<Duration> {
        match str {
            "Weather" => Some(Duration::Weather),
            "Nearest" => Some(Duration::Nearest),
            "LongTerm" => Some(Duration::LongTerm),
            "Multi" => Some(Duration::Multi),
            _ => None
        }
    }
}

impl Field {
    pub fn as_str(&self) -> &'static str {
        match self {
            Field::Temperature => "Temperature",
            Field::WindSpeed => "WindSpeed",
            Field::Pressure => "Pressure",
            Field::Humidity => "Humidity"
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Field::Temperature => String::from("Temperature"),
            Field::WindSpeed => String::from("WindSpeed"),
            Field::Pressure => String::from("Pressure"),
            Field::Humidity => String::from("Humidity")
        }
    }

    pub fn from_str(str: &str) -> Option<Field> {
        match str {
            "Temperature" => Some(Field::Temperature),
            "WindSpeed" => Some(Field::WindSpeed),
            "Pressure" => Some(Field::Pressure),
            "Humidity" => Some(Field::Humidity),
            _ => None
        }
    }

    pub fn as_ru_str(&self) -> &'static str {
        match self {
            Field::Temperature => "Температура",
            Field::WindSpeed => "Скорость ветра",
            Field::Pressure => "Давление",
            Field::Humidity => "Влажность"
        }
    }

    pub fn units(&self) -> &'static str {
        match self {
            Field::Temperature => "°C",
            Field::WindSpeed => "м/с",
            Field::Pressure => "мм. рт. ст.",
            Field::Humidity => "%"
        }
    }

    pub fn convert(&self, value: String ) -> String {
        match self {
            Field::Pressure => format!("{:.0}", value.as_str().parse::<f64>().unwrap() * 0.75006).to_string(),
            _ => {
                if let Ok(num) = value.as_str().parse::<f64>() {
                    format!("{:.1}", num).to_string()
                } else {
                    value
                }
            }
        }
    }
}