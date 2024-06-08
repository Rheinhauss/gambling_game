use crate::utils::player::*;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::btree_map::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage{
    GameStart,
    DrawItem(GameItem, GameItem, GameItem),
    Act(Player),
    Restricted(Player),
    TurnCheck,
    GameOver,
}

pub struct GameState {
    players: (Player, Player),
    items: BTreeMap<Player, [GameItem; 4]>, // 双方道具、血量, 只有两个人的
    hp: BTreeMap<Player, i32>,              // 双方血量， 只有两个人的
    revolver: Vec<Bullet>,                  // 中间左轮的子弹状态
    stage: Stage,
    damage: Damage,
    // current_player: Player,                 // 谁进行当前回合
    // next_player: Player,                    // 谁进行下一回合
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
            stage: Stage::GameStart,
            damage: Damage::new(),
            // current_player: Player::new(),
            // next_player: Player::new(),
            round: 0,
            turn: 0,
            last_use: None,
        }
    }

    fn is_current_player(&self, player: Player) -> bool {
        match self.stage{
            Stage::Act(current_player) => player == current_player,
            Stage::Restricted(current_player) => player != current_player,
            _ => false,
        }
        
    }

    // fn is_next_player(&self, player: Player) -> bool {
    //     self.next_player == player
    // }
    fn round_turn(&self) -> (u32, u32) {
        (self.round, self.turn)
    }
    // fn set_next_player(&mut self, player: Player) {
    //     self.next_player = player;
    // }

    fn use_item(&mut self, player: Player, item: GameItem){
        match item {
            GameItem::Knife => {
                self.damage.set_double();
            }
            GameItem::Medicine => {

            }
        }
    }

    fn draw_item(&mut self, player: Player, item: Option<GameItem>){

    }

    fn player_leave(&mut self, player: Player){

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

struct Damage{
    is_double: bool,
    damage: u32,
}

impl Damage{
    pub fn new() -> Self {  
        Self{ is_double: false, damage: 1,}
    }

    pub fn get_damage(&mut self) -> u32 {
        if self.is_double {self.is_double = false; self.damage * 2} else { self.damage }
    }

    pub fn set_double(&mut self){
        self.is_double = true;
    }
}