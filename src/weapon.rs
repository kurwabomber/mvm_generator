pub struct Weapon{
    pub name: String,
}
impl Default for Weapon{
    fn default() -> Self {
        Weapon { 
            name: "".to_string(),
        }
    }
}