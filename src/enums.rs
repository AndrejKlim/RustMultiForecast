pub enum Source {
    OpenWeather
}

pub enum Duration {
    Weather,
    Nearest,
    LongTerm,
    Multi
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

    pub fn from_str(str: &str) -> Result<Source, String> {
        match str {
            "OpenWeather" => Ok(Source::OpenWeather),
            _ => Err("No such source".to_string())
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

    pub fn from_str(str: &str) -> Result<Duration, String> {
        match str {
            "Weather" => Ok(Duration::Weather),
            "Nearest" => Ok(Duration::Nearest),
            "LongTerm" => Ok(Duration::LongTerm),
            "Multi" => Ok(Duration::Multi),
            _ => Err("No such duration".to_string())
        }
    }
}