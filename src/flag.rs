pub struct Flag {
    pub name: String,
    pub usage: String,
    pub alias: Option<Vec<String>>,
    pub default_value: String,
}
