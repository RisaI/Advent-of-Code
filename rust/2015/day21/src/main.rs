use anyhow::Context;

struct Entity {
    pub hp: usize,
    pub damage: usize,
    pub armor: usize,
}

impl Entity {
    fn damage_against(&self, other: &Entity) -> usize {
        self.damage.saturating_sub(other.armor).max(1)
    }

    fn equip<'a, 'b>(&'a self, items: impl IntoIterator<Item = &'b Item>) -> Self {
        let (item_damage, item_armor) = items
            .into_iter()
            .fold((0, 0), |(prev_dmg, prev_armor), next| {
                (prev_dmg + next.damage, prev_armor + next.armor)
            });

        Self {
            hp: self.hp,
            damage: self.damage + item_damage,
            armor: self.armor + item_armor,
        }
    }

    fn defeats(&self, boss: &Entity) -> bool {
        let mut player_hp = self.hp;
        let mut boss_hp = boss.hp;

        loop {
            boss_hp = boss_hp.saturating_sub(self.damage_against(boss));

            if boss_hp == 0 {
                return true;
            }

            player_hp = player_hp.saturating_sub(boss.damage_against(self));

            if player_hp == 0 {
                return false;
            }
        }
    }
}

#[derive(Debug)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    pub const fn new_weapon(cost: usize, damage: usize) -> Self {
        Item {
            cost,
            damage,
            armor: 0,
        }
    }

    pub const fn new_armor(cost: usize, armor: usize) -> Self {
        Item {
            cost,
            armor,
            damage: 0,
        }
    }
}

static WEAPONS: &[Option<Item>] = &[
    Some(Item::new_weapon(8, 4)),
    Some(Item::new_weapon(10, 5)),
    Some(Item::new_weapon(25, 6)),
    Some(Item::new_weapon(40, 7)),
    Some(Item::new_weapon(74, 8)),
];

static ARMORS: &[Option<Item>] = &[
    None,
    Some(Item::new_armor(13, 1)),
    Some(Item::new_armor(31, 2)),
    Some(Item::new_armor(53, 3)),
    Some(Item::new_armor(75, 4)),
    Some(Item::new_armor(102, 5)),
];

static RINGS: &[Option<Item>] = &[
    None,
    Some(Item::new_weapon(25, 1)),
    Some(Item::new_weapon(50, 2)),
    Some(Item::new_weapon(100, 3)),
    Some(Item::new_armor(20, 1)),
    Some(Item::new_armor(40, 2)),
    Some(Item::new_armor(80, 3)),
];

fn main() -> anyhow::Result<()> {
    let player = Entity {
        hp: 100,
        armor: 0,
        damage: 0,
    };
    let boss = Entity {
        hp: 104,
        damage: 8,
        armor: 1,
    };

    let combos = WEAPONS
        .iter()
        .flat_map(|w| {
            ARMORS.iter().flat_map(move |a| {
                RINGS.iter().flat_map(move |r1| {
                    RINGS
                        .iter()
                        .map(move |r2| [w.as_ref(), a.as_ref(), r1.as_ref(), r2.as_ref()])
                })
            })
        })
        .filter(|[_, _, a, b]| a.is_some() || b.is_none());

    let cheapest = combos
        .clone()
        .filter(|l| player.equip(l.iter().filter_map(|i| *i)).defeats(&boss))
        .map(|l| {
            l.iter()
                .fold(0, |p, i| p + i.map(|i| i.cost).unwrap_or_default())
        })
        .min()
        .context("cannot win")?;

    println!("cheapest winning loadout costs {cheapest}");

    let expensive = combos
        .clone()
        .filter(|l| !player.equip(l.iter().filter_map(|i| *i)).defeats(&boss))
        .map(|l| {
            l.iter()
                .fold(0, |p, i| p + i.map(|i| i.cost).unwrap_or_default())
        })
        .max()
        .context("cannot lose")?;

    println!("the most expensive losing loadout costs {expensive}");

    Ok(())
}
