use std::{
    cmp,
    sync::{
        atomic::{self, AtomicU32},
        Arc,
    },
};

use serde::{Serialize, Deserialize};

#[derive(Hash, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Player {
    id: u32,
}

#[allow(non_camel_case_types)]
pub type PlayerId_t = u32;

static IDGEN: AtomicU32 = AtomicU32::new(0);

impl Player {
    pub fn new() -> Self {
        Player {
            id: Self::get_new_id(),
        }
    }
    fn get_new_id() -> PlayerId_t {
        IDGEN.fetch_add(1, atomic::Ordering::SeqCst)
    }
    pub fn id(&self) -> PlayerId_t {
        self.id
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
#[serde(deny_unknown_fields)]
pub enum GameItem {
    Knife,     // 折叠刀：使下一次开枪的伤害翻倍
    Cigarette, // 香烟：回复玩家1点血量
    Beer,      // 啤酒：弹出当前枪膛的1枚子弹
    Handcuff,  // 手铐：对手下一回合无法行动
    Magnifier, // 放大镜：是查看当前枪膛内子弹是实弹还是哑弹
    Reverser,  // 逆转器：修改当前枪膛内子弹的类型，反转哑弹-实弹
    Phone,     // 电话：若枪内仍有x颗子弹，查看第2颗到第x颗中随机一颗的子弹类型
    Medicine,  // 药盒：50%概率回复玩家2点血量，50%概率扣除玩家1点血量
    Empty,     // 空：无道具
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all(serialize = "lowercase", deserialize = "PascalCase"))]
#[serde(deny_unknown_fields)]
pub enum Bullet {
    Dummy, // 哑弹
    Real,  // 实弹
}