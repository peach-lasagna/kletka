use std::cmp::PartialEq;
use crate::{pub_struct, struct_w_weight};

enum NPC {
    Dealer,
    Pilgrim,
    Death
}

#[derive(PartialEq, Clone, Copy)]
pub enum Cells {
    Nothing,
    Start,
    Enemy,
    Exit,
    // NPC(NPC),
    Weapon,
    Gold,
    Heal,
    Border
    // StatsUp,
}



struct_w_weight!(Gold, weight=1, count=10);



pub_struct!( Items {
    // weapon: Weapon,
    // armor: Armor,
    gold: Gold,
});

// pub_struct!( Stats{
//     hp: u8,
//     max_hp: u8,
// });

