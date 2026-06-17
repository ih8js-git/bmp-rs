use crate::joker::Joker;
use crate::shop::booster_pack::Pack;

pub const BOOSTER_PACK_AMOUNT: usize = 2;
pub const MAX_SHOP_SIZE: usize = 2;
pub struct Shop {
    pub items: [ShopItem; MAX_SHOP_SIZE],
    pub booster_packs: [Pack; BOOSTER_PACK_AMOUNT],  // fixed array of size 2
    pub reroll_cost: u8,


}


pub struct ShopItem {
    pub item_type: ShopItemType,
    pub cost: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ShopItemType {
    Joker(Joker),
    Planet,
    Tarot,
}