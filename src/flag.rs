use crate::Vector;

#[derive(Clone, Debug)]
pub struct Flag {
    pub name: String,
    pub usage: String,
    pub alias: Vector<String>,
    pub default_value: FlagValue,
    pub flag_type: FlagType,
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

impl FlagType {
    pub fn name<'a>(&self) -> &'a str {
        match self {
            FlagType::Bool => "Bool",
            FlagType::String => "String",
            FlagType::Int => "Int",
            FlagType::Float => "Float",
            //_ => "Unknown",
        }
    }
    pub fn type_default(&self) -> FlagValue {
        match self {
            FlagType::Bool => FlagValue::Bool(bool::default()),
            FlagType::String => FlagValue::String(String::default()),
            FlagType::Int => FlagValue::Int(isize::default()),
            FlagType::Float => FlagValue::Float(f64::default()),
        }
    }
    pub fn is_tyoe_of(&self, val: &FlagValue) -> bool {
        Some(self) == val.get_type()
    }
}

impl Default for FlagType {
    fn default() -> Self {
        FlagType::String
    }
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
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

impl FlagValue {
    pub fn get_type(&self) -> Option<&FlagType> {
        match self {
            FlagValue::Bool(_) => Some(&FlagType::Bool),
            FlagValue::String(_) => Some(&FlagType::String),
            FlagValue::Int(_) => Some(&FlagType::Int),
            FlagValue::Float(_) => Some(&FlagType::Float),
            FlagValue::None => None,
        }
    }
    pub fn is_type(&self, flag_type: &FlagType) -> bool {
        Some(flag_type) == self.get_type()
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
        let default_value: FlagValue = match flag_type {
            FlagType::Bool => FlagValue::Bool(bool::default()),
            FlagType::String => FlagValue::String(String::default()),
            FlagType::Int => FlagValue::Int(isize::default()),
            FlagType::Float => FlagValue::Float(f64::default()),
        };
        Flag {
            name,
            usage: String::default(),
            alias: Vector::default(),
            flag_type,
            default_value,
        }
    }

    pub fn build_new(
        name: String,
        usage: String,
        alias: Vector<String>,
        flag_type: FlagType,
        default_value: FlagValue,
    ) -> Flag {
        let calculated_default_value = if default_value.is_type(&flag_type) {
            default_value
        } else {
            let flag_type_str = flag_type.name();
            eprintln!("FlagType is {},but inputted default_value is not {}. default_value will be {}'s default.",flag_type_str,flag_type_str,flag_type_str);
            flag_type.type_default()
        };
        Flag {
            name,
            usage,
            alias,
            flag_type,
            default_value: calculated_default_value,
        }
    }

    pub fn alias<T: Into<String>>(mut self, a: T) -> Self {
        self.alias.push(a.into());
        self
    }

    pub fn default_value(mut self, default_value: FlagValue) -> Self {
        if self.flag_type.is_tyoe_of(&default_value) {
            self.default_value = default_value
        } else {
            println!(
                "not match flag_type: {}. default_value is not changed.",
                self.flag_type.name()
            );
        }
        self
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
        self
    }
}
