pub struct Joker {
    pub scaled: u32,
    pub id: u16,
}

pub struct JokerSlots {
    pub joker_list: Vec<(Joker, Postion)>,
    pub max_joker_count: u8,
}
