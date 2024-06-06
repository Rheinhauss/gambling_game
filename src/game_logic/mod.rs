pub mod game_event {
    use crate::utils::player::Player;
    use log::{debug, error, info, trace, warn};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use tokio::sync::mpsc::UnboundedSender;

    pub enum GameEvent {}
}

pub mod game_logic {
    use crate::utils::player::Player;
    use log::{debug, error, info, trace, warn};
    use std::collections::btree_map::BTreeMap;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    pub struct GameState {
        items: BTreeMap<Player, Arc<[GameItem]>>, // 双方道具、血量
        hp: BTreeMap<Player, i32>,                // 双方血量
        revolver: Vec<Bullet>,                    // 中间左轮的子弹状态
        current_player: Player,                   // 谁进行当前回合
        next_player: Player,                      // 谁进行下一回合
    }
    impl GameState {
        pub fn new() -> Self {
            GameState {
                items: BTreeMap::new(),
                hp: BTreeMap::new(),
                revolver: Vec::new(),
                current_player: Player::new(),
                next_player: Player::new(),
            }
        }
    }

    enum GameItem {
        Knife,           // 折叠刀：使下一次开枪的伤害翻倍
        Cigarette,       // 香烟：回复玩家1点血量
        Beer,            // 啤酒：弹出当前枪膛的1枚子弹
        Handcuff,       // 手铐：对手下一回合无法行动
        Magnifier,       // 放大镜：是查看当前枪膛内子弹是实弹还是哑弹
        Reverser,        // 逆转器：修改当前枪膛内子弹的类型，反转哑弹-实弹
        Phone,           // 电话：若枪内仍有x颗子弹，查看第2颗到第x颗中随机一颗的子弹类型
        Medicine, // 药盒：50%概率回复玩家2点血量，50%概率扣除玩家1点血量
        Null,            // 空：无道具
    }

    enum Bullet {
        Dummy, // 哑弹
        Real,  // 实弹
    }

    pub mod game_match {
        use log::{debug, error, info, trace, warn};
        use uuid::Uuid;
        // random matchmaking
        pub async fn matchmake() {
            log::info!("Matchmaking");
        }

        // create a room
        pub fn create_room(uuid: Uuid) {
            log::info!("Creating a room by {}", uuid);
        }
    }
}
