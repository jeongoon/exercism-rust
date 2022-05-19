#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                mana: match self.level {
                    (0..=9) => None,
                    _ => Some(100),
                },
                level: self.level,
            }),

            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana_cost > mana => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                mana_cost.saturating_mul(2)
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}

fn main() {
    let dead_player = Player {
        health: 0,
        mana: Some(20),
        level: 22,
    };
    let mut new_player = dead_player.revive().unwrap();
    assert_eq!(new_player.cast_spell(10), 20);
}
