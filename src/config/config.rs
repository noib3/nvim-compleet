use mlua::{Table, Value};

#[derive(Debug)]
pub struct Config {
    /// TODO: docs
    autoshow_menu: bool,

    /// TODO: docs
    enable_default_mappings: bool,

    /// TODO: docs
    show_hints: bool,
}

impl Config {
    fn field_names() -> &'static [&'static str] {
        &["autoshow_menu", "enable_default_mappings", "show_hints"]
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            autoshow_menu: true,
            enable_default_mappings: false,
            show_hints: false,
        }
    }
}

pub enum Error {
    Conversion {
        option: &'static str,
        expected: &'static str,
    },

    OptionDoesntExist {
        option: String,
    },

    Lua(mlua::Error),
}

impl From<mlua::Error> for Error {
    fn from(err: mlua::Error) -> Error {
        Error::Lua(err)
    }
}

impl<'a> TryFrom<Option<Table<'a>>> for Config {
    type Error = Error;

    fn try_from(preferences: Option<Table>) -> Result<Self, Error> {
        let mut config = Config::default();

        if preferences.is_none() {
            return Ok(config);
        }

        let preferences = preferences.unwrap();

        for pair in preferences.clone().pairs::<String, Value>() {
            let (key, _) = pair?;
            if !Config::field_names().contains(&key.as_str()) {
                return Err(Error::OptionDoesntExist { option: key });
            }
        }

        match preferences.get("autoshow_menu")? {
            Value::Nil => {},
            Value::Boolean(bool) => config.autoshow_menu = bool,
            _ => {
                return Err(Error::Conversion {
                    option: "autoshow_menu",
                    expected: "boolean",
                })
            },
        }

        match preferences.get("enable_default_mappings")? {
            Value::Nil => {},
            Value::Boolean(bool) => config.enable_default_mappings = bool,
            _ => {
                return Err(Error::Conversion {
                    option: "enable_default_mappings",
                    expected: "boolean",
                })
            },
        }

        match preferences.get("show_hints")? {
            Value::Nil => {},
            Value::Boolean(bool) => config.show_hints = bool,
            _ => {
                return Err(Error::Conversion {
                    option: "show_hints",
                    expected: "boolean",
                })
            },
        }

        Ok(config)
    }
}
