#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Import {
    pub qualified_name: &'static str,
    pub alias: Option<String>
}