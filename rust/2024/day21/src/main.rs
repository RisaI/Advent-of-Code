use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{LazyLock, Mutex},
};

use aoc_utils::{FxHashMap, IVec2};

static KEYPAD_POSITIONS: LazyLock<FxHashMap<char, IVec2>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        ('7', IVec2::new(0, 0)),
        ('8', IVec2::new(1, 0)),
        ('9', IVec2::new(2, 0)),
        ('4', IVec2::new(0, 1)),
        ('5', IVec2::new(1, 1)),
        ('6', IVec2::new(2, 1)),
        ('1', IVec2::new(0, 2)),
        ('2', IVec2::new(1, 2)),
        ('3', IVec2::new(2, 2)),
        ('0', IVec2::new(1, 3)),
        ('A', IVec2::new(2, 3)),
    ])
});

fn keycode_to_directions(code: &str) -> String {
    let mut instructions = String::new();
    let mut position = IVec2::new(2, 3);

    for c in code.chars() {
        let target = KEYPAD_POSITIONS[&c];
        let dir = target - position;

        if dir.x < 0 && (target.x > 0 || position.y < 3) {
            instructions.extend((0..dir.x.abs()).map(|_| if dir.x > 0 { '>' } else { '<' }));
            instructions.extend((0..dir.y.abs()).map(|_| if dir.y > 0 { 'v' } else { '^' }));
        } else {
            instructions.extend((0..dir.y.abs()).map(|_| if dir.y > 0 { 'v' } else { '^' }));
            instructions.extend((0..dir.x.abs()).map(|_| if dir.x > 0 { '>' } else { '<' }));
        }

        instructions.push('A');

        position = target;
    }

    instructions
}

static DIRPAD_POSITIONS: LazyLock<FxHashMap<char, IVec2>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        ('^', IVec2::new(1, 0)),
        ('A', IVec2::new(2, 0)),
        ('<', IVec2::new(0, 1)),
        ('v', IVec2::new(1, 1)),
        ('>', IVec2::new(2, 1)),
    ])
});

static GROUP_CACHE: LazyLock<Mutex<FxHashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(Default::default()));

static COUNT_CACHE: LazyLock<Mutex<FxHashMap<String, FxHashMap<usize, usize>>>> =
    LazyLock::new(|| Mutex::new(Default::default()));

fn dircode_len_after_iterations(mut code: &str, rem_iters: usize) -> usize {
    if rem_iters == 0 {
        return code.len();
    }

    let mut result = 0;

    while let Some(idx) = code.find('A') {
        let group = &code[..=idx];

        let cached = COUNT_CACHE
            .lock()
            .unwrap()
            .get(group)
            .and_then(|v| v.get(&rem_iters))
            .copied();

        let r = if let Some(r) = cached {
            r
        } else {
            let sub =
                dircode_len_after_iterations(&directions_group_to_dircode(group), rem_iters - 1);

            COUNT_CACHE
                .lock()
                .unwrap()
                .entry(group.to_string())
                .or_default()
                .insert(rem_iters, sub);

            sub
        };

        result += r;
        code = &code[(idx + 1)..];
    }

    result
}

fn directions_group_to_dircode(group: &str) -> String {
    if let Some(r) = GROUP_CACHE.lock().unwrap().get(group) {
        return r.clone();
    }

    let mut instructions: String = String::new();
    let mut position = IVec2::new(2, 0);

    for char in group.chars() {
        let target = *DIRPAD_POSITIONS.get(&char).unwrap();
        let dir = target - position;

        if (dir.x < 0 && (target.x > 0 || position.y > 0)) || position == IVec2::new(0, 1) {
            instructions.extend((0..dir.x.abs()).map(|_| if dir.x > 0 { '>' } else { '<' }));
            instructions.extend((0..dir.y.abs()).map(|_| if dir.y > 0 { 'v' } else { '^' }));
        } else {
            instructions.extend((0..dir.y.abs()).map(|_| if dir.y > 0 { 'v' } else { '^' }));
            instructions.extend((0..dir.x.abs()).map(|_| if dir.x > 0 { '>' } else { '<' }));
        }

        instructions.push('A');
        position = target;
    }

    GROUP_CACHE
        .lock()
        .unwrap()
        .insert(group.to_string(), instructions.clone());

    instructions
}

#[cfg(test)]
fn directions_to_dircode(mut code: &str) -> String {
    let mut instructions: String = String::new();

    while let Some(idx) = code.find('A') {
        instructions.push_str(&directions_group_to_dircode(&code[..=idx]));
        code = &code[(idx + 1)..];
    }

    instructions
}

#[test]
fn p1_example_keycode_to_dir() {
    assert_eq!(keycode_to_directions("029A"), "<A^A^^>AvvvA");
}

#[test]
fn p1_example_sequence() {
    let mut base = keycode_to_directions("029A");

    for next in [
        "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
    ] {
        base = directions_to_dircode(&base);

        assert_eq!(base.len(), next.len(),);
    }
}

#[test]
fn p1_example() {
    let data = [
        [
            "029A",
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        ],
        [
            "980A",
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
        ],
        [
            "179A",
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ],
        [
            "456A",
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
        ],
        [
            "379A",
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ],
    ];

    for [keycode, directions] in data {
        let dirs = keycode_to_directions(keycode);
        let val = directions_to_dircode(&directions_to_dircode(&dirs));

        assert_eq!(
            val.len(),
            directions.len(),
            "min length differs for {keycode}\n{val}\n{directions}"
        );

        println!();
    }
}

fn main() -> anyhow::Result<()> {
    let (total_complexity_p1, total_complexity_p2) = BufReader::new(File::open("input.txt")?)
        .lines()
        .try_fold((0, 0), |(p1, p2), line| -> anyhow::Result<_> {
            let line = line?;

            Ok(if !line.is_empty() {
                let directions = keycode_to_directions(&line);
                let num_part = line[0..3].parse::<usize>()?;

                let f = dircode_len_after_iterations(&directions, 2);
                let g = dircode_len_after_iterations(&directions, 25);

                (p1 + f * num_part, p2 + g * num_part)
            } else {
                (p1, p2)
            })
        })?;

    println!("sum of complexities (p1) = {total_complexity_p1}");
    println!("sum of complexities (p2) = {total_complexity_p2}");

    Ok(())
}
