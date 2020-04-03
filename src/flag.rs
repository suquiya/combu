use crate::Vector;

#[derive(Clone)]
pub struct Flag {
    pub name: String,
    pub usage: String,
    pub alias: Vector<String>,
    pub default_value: FlagValue,
    pub flag_type: FlagType,
}

#[derive(PartialOrd, PartialEq, Clone)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

impl Default for FlagType {
    fn default() -> Self {
        FlagType::String
    }
}

#[derive(PartialOrd, PartialEq, Clone)]
pub enum FlagValue {
    Bool(bool),
    String(String),
    Int(isize),
    Float(f64),
    None,
}

impl Default for FlagValue {
    fn default() -> Self {
        FlagValue::None
    }
}

impl Default for Flag {
    fn default() -> Flag {
        Flag {
            name: String::default(),
            usage: String::default(),
            alias: Vector::default(),
            flag_type: FlagType::default(),
            default_value: FlagValue::default(),
        }
    }
}

impl Flag {
    pub fn new(name: String, flag_type: FlagType) -> Flag {
        Flag {
            name,
            usage: String::default(),
            alias: Vector::default(),
            flag_type,
            default_value: FlagValue::None,
        }
    }

    pub fn build_new(
        name: String,
        usage: String,
        alias: Vector<String>,
        flag_type: FlagType,
        default_value: FlagValue,
    ) -> Flag {
        Flag {
            name,
            usage,
            alias,
            flag_type,
            default_value,
        }
    }
}
