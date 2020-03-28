use crate::Vector;

pub struct Flag {
    pub name: String,
    pub usage: String,
    pub alias: Vector<String>,
    pub default_value: String,
}

impl Default for Flag {
    fn default() -> Flag {
        Flag {
            name: String::default(),
            usage: String::default(),
            alias: Vector::default(),
            default_value: String::default(),
        }
    }
}

impl Flag {
    pub fn new() -> Flag {
        Flag::default()
    }

    pub fn build_new(
        name: String,
        usage: String,
        alias: Vector<String>,
        default_value: String,
    ) -> Flag {
        Flag {
            name,
            usage,
            alias,
            default_value,
        }
    }
}
