use crate::utils::player::Player;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::sync::Arc;

pub struct GameState {
    players: (Player, Player),
    items: BTreeMap<Player, [GameItem; 4]>, // 双方道具、血量, 只有两个人的
    hp: BTreeMap<Player, i32>,              // 双方血量， 只有两个人的
    revolver: Vec<Bullet>,                  // 中间左轮的子弹状态
    current_player: Player,                 // 谁进行当前回合
    next_player: Player,                    // 谁进行下一回合
    round: u32,                             // 轮次数
    turn: u32,                              // 回合数
    last_use: Option<GameItemUse>,          // 上一次使用道具记录
}
impl GameState {
    pub fn new(p1: Player, p2: Player) -> Self {
        GameState {
            players: (p1, p2),
            items: BTreeMap::new(),
            hp: BTreeMap::new(),
            revolver: Vec::new(),
            current_player: Player::new(),
            next_player: Player::new(),
            round: 0,
            turn: 0,
            last_use: None,
        }
    }
    fn is_current_player(&self, player: Player) -> bool {
        self.current_player == player
    }
    fn is_next_player(&self, player: Player) -> bool {
        self.next_player == player
    }
    fn round_turn(&self) -> (u32, u32) {
        (self.round, self.turn)
    }
    fn set_next_player(&mut self, player: Player) {
        self.next_player = player;
    }
    fn pop_item(&mut self, player: Player, index: u32) -> Result<GameItem, &str> {
        match self.items.get_mut(&player) {
            Some(items) => items
                .get_mut(index as usize)
                .ok_or("index out of bound 4")
                .map(|item| {
                    let ret = item.clone();
                    *item = GameItem::Empty;
                    ret
                }),
            None => Err("Player not found"),
        }
    }
    pub fn open_state(&self, p: Player) -> Option<GameStateOpen> {
        let (p1, p2) = self.players;
        // p must in players
        if p != p1 && p != p2 {
            None
        } else {
            let (pl_self, pl_oppo) = if p == p1 { (p1, p2) } else { (p2, p1) };
            Some(GameStateOpen {
                round: self.round,
                turn: self.turn,
                hp_self: self.hp.get(&pl_self).unwrap().clone(),
                hp_oppo: self.hp.get(&pl_oppo).unwrap().clone(),
                items_self: self.items.get(&pl_self).unwrap().clone(),
                items_oppo: self.items.get(&pl_oppo).unwrap().clone(),
                playing: self.is_current_player(p),
                last_use: self.last_use,
            })
        }
    }
    pub fn hidden_state(&self) -> Option<GameStateHidden> {
        Some(GameStateHidden {
            num: self.revolver.len() as i32,
            bullets: self.revolver.clone(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
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
enum Bullet {
    Dummy, // 哑弹
    Real,  // 实弹
}

#[derive(Serialize)]
pub struct GameStateOpen {
    round: u32,
    turn: u32,
    hp_self: i32,
    hp_oppo: i32,
    items_self: [GameItem; 4],
    items_oppo: [GameItem; 4],
    playing: bool,
    last_use: Option<GameItemUse>,
}

#[derive(Serialize)]
pub struct GameStateHidden {
    num: i32,
    bullets: Vec<Bullet>,
}

#[derive(Serialize, Clone, Copy)]
pub struct GameItemUse {
    player: Player,
    item: GameItem,
    bullet: Bullet,
}
