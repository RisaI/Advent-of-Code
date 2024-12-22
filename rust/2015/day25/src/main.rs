fn to_linear_index(row: usize, col: usize) -> usize {
    let start_row = row + col - 2;

    let n = start_row * (start_row + 1) / 2;

    n + col - 1
}

#[test]
fn linear_index_works() {
    assert_eq!(to_linear_index(1, 1), 0);
    assert_eq!(to_linear_index(4, 3), 17);
}

fn main() {
    println!(
        "the code is {}",
        (0..to_linear_index(3010, 3019))
            .fold(20151125u128, |prev, _| { (prev * 252533) % 33554393 })
    );
}
