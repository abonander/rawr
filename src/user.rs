use time::Timespec;

pub struct User {
    id: String,
    pub name: String,
    pub created_utc: Timespec,
    pub link: u32,
    pub comment: u32,
    pub over_18: bool,
    pub is_gold: bool,
    pub is_mod: bool, 
}

pub struct Message;

