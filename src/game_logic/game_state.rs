use crate::utils::player::{self, *};
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::de::Read;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    RoundStart,
    SendItem(Player, [GameItem; 3]),
    Act(Player),
    GameOver(Player),
}

pub struct GameState {
    players: (Player, Player),
    items: HashMap<Player, Vec<GameItem>>, // 双方道具, 只有两个人的
    hps: HashMap<Player, i32>,             // 双方血量， 只有两个人的
    cuffs: HashMap<Player, bool>,
    revolver: Vec<Bullet>, // 中间左轮的子弹状态
    stage: Stage,
    damage: Damage,
    // current_player: Player,                 // 谁进行当前回合
    // next_player: Player,                    // 谁进行下一回合
    round: u32,                    // 轮次数
    turn: u32,                     // 回合数
    last_use: Option<GameItemUse>, // 上一次使用道具记录
}
impl GameState {
    pub fn new(p1: Player, p2: Player) -> Self {
        GameState {
            players: (p1, p2),
            items: HashMap::new(),
            hps: HashMap::new(),
            cuffs: HashMap::new(),
            revolver: Vec::new(),
            stage: Stage::RoundStart,
            damage: Damage::new(),
            // current_player: Player::new(),
            // next_player: Player::new(),
            round: 0,
            turn: 0,
            last_use: None,
        }
    }

    pub fn start_round(&mut self) {
        todo!("fill the bullets");
        if let Stage::RoundStart = self.stage {
            self.switch_to_player(if fastrand::bool() {
                self.players.0
            } else {
                self.players.1
            });
        }
    }

    fn is_current_player(&self, player: Player) -> bool {
        match self.stage {
            Stage::Act(current_player) => player == current_player,
            _ => false,
        }
    }

    pub fn current_stage(&self) -> Stage {
        self.stage
    }

    fn round_turn(&self) -> (u32, u32) {
        (self.round, self.turn)
    }

    pub fn use_item(&mut self, player: Player, item: GameItem) {
        if self.is_acting(player) {
            match item {
                GameItem::Knife => {
                    self.last_use = Some(GameItemUse::new(player, item, None, None));
                    self.damage.set_double();
                }
                GameItem::Medicine => {
                    let offset = if fastrand::bool() { -1 } else { 2 };
                    self.adapt_hp(player, offset);
                    self.last_use = Some(GameItemUse::new(player, item, None, Some(offset)));
                }
                GameItem::Beer => {
                    self.last_use = Some(GameItemUse::new(player, item, self.revolver.pop(), None));
                }

                GameItem::Cigarette => {
                    self.last_use = Some(GameItemUse::new(player, item, None, Some(1)));
                    self.adapt_hp(player, 1);
                }
                GameItem::Handcuff => {
                    self.last_use = Some(GameItemUse::new(player, item, None, None));
                    self.cuffs.iter_mut().for_each(|(p, c)| *c = !*c);
                }
                GameItem::Magnifier => {
                    let bullet_idx = self.revolver.len() - 1;
                    if let Some(bullet) = self.revolver.get(bullet_idx) {
                        self.last_use = Some(GameItemUse::new(
                            player,
                            item,
                            Some(*bullet),
                            Some(bullet_idx as i32),
                        ));
                    }
                }
                GameItem::Phone => {
                    let bullet_idx = fastrand::usize(..self.revolver.len());
                    if let Some(bullet) = self.revolver.get(bullet_idx) {
                        self.last_use = Some(GameItemUse::new(
                            player,
                            item,
                            Some(*bullet),
                            Some(bullet_idx as i32),
                        ));
                    }
                }
                GameItem::Reverser => {
                    self.last_use = Some(GameItemUse::new(player, item, None, None));
                    match self.revolver.pop() {
                        Some(bullet) => self.revolver.push(bullet.reverse()),
                        None => info! {"no bullet to reverse!"},
                    };
                }
                GameItem::Empty => {
                    self.last_use = Some(GameItemUse::new(player, item, None, None));
                }
            };
        }
        self.check_game_over();
    }

    pub fn draw_item(&mut self, player: Player, item: Option<GameItem>) {
        if self.is_acting(player) {
            if let Some(new_item) = item {
                if let Some(new_items) = self.items.get_mut(&player) {
                    new_items.push(new_item);
                }
            }
        }
    }

    pub fn item_sended(&mut self) {
        if let Stage::SendItem(player, _) = self.stage {
            self.stage = Stage::Act(player);
        }
    }

    fn opponent_of(&self, player: Player) -> Player {
        if player == self.players.0 {
            self.players.1
        } else {
            self.players.0
        }
    }

    fn get_random_item(&self) -> GameItem {
        match fastrand::u8(0..=7) {
            0 => GameItem::Beer,
            1 => GameItem::Cigarette,
            2 => GameItem::Handcuff,
            3 => GameItem::Knife,
            4 => GameItem::Reverser,
            5 => GameItem::Magnifier,
            6 => GameItem::Medicine,
            _ => GameItem::Phone,
        }
    }

    fn generate_items(&self) -> [GameItem; 3] {
        [
            self.get_random_item(),
            self.get_random_item(),
            self.get_random_item(),
        ]
    }

    fn switch_to_player(&mut self, player: Player) {
        let items = self.generate_items();
        self.stage = match self.cuffs.get(&player) {
            Some(false) => Stage::SendItem(player, items),
            _ => Stage::SendItem(self.opponent_of(player), items),
        }
    }

    fn is_acting(&self, player: Player) -> bool {
        if let Stage::Act(curr_player) = self.stage {
            if player == curr_player {
                return true;
            }
        }
        false
    }

    fn adapt_hp(&mut self, player: Player, offset: i32) {
        self.hps.get_mut(&player).map(|hp| *hp + offset);
    }

    pub fn shoot(&mut self, player: Player, is_suicide: bool) {
        if self.is_acting(player) {
            let opponent = self.opponent_of(player);
            if let Some(bullet) = self.revolver.pop() {
                match bullet {
                    Bullet::Real => {
                        if is_suicide {
                            let offset = -(self.damage.get_damage() as i32);
                            self.adapt_hp(player, offset);
                            if !self.check_game_over() {
                                self.switch_to_player(opponent);
                            }
                        } else {
                            let offset = -(self.damage.get_damage() as i32);
                            self.adapt_hp(opponent, offset);
                            self.check_game_over();
                        }
                    }
                    Bullet::Dummy => {
                        if !is_suicide {
                            self.switch_to_player(opponent);
                        }
                    }
                }
            }
        }
    }

    pub fn player_leave(&mut self, player: Player) {
        if let Some(hp) = self.hps.get_mut(&player) {
            *hp = 0;
        }
        self.stage = Stage::GameOver(self.opponent_of(player))
    }

    fn check_game_over(&mut self) -> bool {
        for (p, hp) in self.hps.iter() {
            if *hp <= 0 {
                self.stage = Stage::GameOver(self.opponent_of(*p));
                return true;
            }
        }
        false
    }

    pub fn pop_item(&mut self, player: Player, index: u32) -> Result<GameItem, &str> {
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
                hp_self: self.hps.get(&pl_self).unwrap().clone(),
                hp_oppo: self.hps.get(&pl_oppo).unwrap().clone(),
                items_self: self.items.get(&pl_self).unwrap()[0..4].try_into().unwrap(),
                items_oppo: self.items.get(&pl_oppo).unwrap()[0..4].try_into().unwrap(),
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
    bullet: Option<Bullet>,
    effect_num: Option<i32>,
}

impl GameItemUse {
    pub fn new(
        player: Player,
        item: GameItem,
        bullet: Option<Bullet>,
        effect_num: Option<i32>,
    ) -> Self {
        Self {
            player,
            item,
            bullet,
            effect_num,
        }
    }
}

struct Damage {
    is_double: bool,
    damage: u32,
}

impl Damage {
    pub fn new() -> Self {
        Self {
            is_double: false,
            damage: 1,
        }
    }

    pub fn get_damage(&mut self) -> u32 {
        if self.is_double {
            self.is_double = false;
            self.damage * 2
        } else {
            self.damage
        }
    }

    pub fn set_double(&mut self) {
        self.is_double = true;
    }
}
