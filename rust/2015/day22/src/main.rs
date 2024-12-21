#[derive(Clone, Copy, Debug)]
struct Player {
    hp: i8,
    mana: u16,
    armor: i8,
    statuses: [u8; 3],
    hard: bool,
}

impl Player {
    pub fn apply_statuses(&mut self, boss: &mut Boss) {
        if self.statuses[0] > 0 {
            self.armor = 7;
        } else {
            self.armor = 0;
        }

        if self.statuses[1] > 0 {
            boss.hp -= 3;
        }

        if self.statuses[2] > 0 {
            self.mana += 101;
        }

        self.statuses
            .iter_mut()
            .for_each(|v| *v = v.saturating_sub(1));
    }
}

#[derive(Clone, Copy, Debug)]
struct Boss {
    hp: i8,
    damage: i8,
}

fn find_cheapest(mut player: Player, mut boss: Boss, cost_so_far: u16, best: &mut u16) {
    if cost_so_far >= *best {
        return;
    }

    if cost_so_far > 0 {
        player.apply_statuses(&mut boss);

        if boss.hp <= 0 {
            *best = cost_so_far;
            return;
        }

        player.hp -= boss.damage.saturating_sub(player.armor).max(1);

        if player.hp <= 0 {
            return;
        }
    }

    if player.hard {
        player.hp -= 1;

        if player.hp <= 0 {
            return;
        }
    }

    player.apply_statuses(&mut boss);

    if boss.hp <= 0 {
        *best = cost_so_far;
        return;
    }

    (0..5).for_each(|i| {
        let mut player = player;
        let mut boss = boss;

        let cost = match i {
            0 => {
                boss.hp -= 4;

                53
            }
            1 => {
                boss.hp -= 2;
                player.hp += 2;

                73
            }
            2 => {
                if player.statuses[0] > 0 {
                    return;
                }

                player.statuses[0] = 6;

                113
            }
            3 => {
                if player.statuses[1] > 0 {
                    return;
                }

                player.statuses[1] = 6;

                173
            }
            4 => {
                if player.statuses[2] > 0 {
                    return;
                }

                player.statuses[2] = 5;

                229
            }
            _ => unreachable!(),
        };

        if player.mana < cost {
            return;
        }
        player.mana -= cost;

        find_cheapest(player, boss, cost_so_far + cost, best)
    })
}

fn main() {
    for hard in [false, true] {
        let mut result = u16::MAX;
        find_cheapest(
            Player {
                hp: 50,
                mana: 500,
                armor: 0,
                statuses: [0; 3],
                hard,
            },
            Boss { hp: 55, damage: 8 },
            0,
            &mut result,
        );

        println!("{result}");
    }
}
