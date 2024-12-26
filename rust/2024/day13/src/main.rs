use winnow::{
    ascii::{digit1, line_ending},
    combinator::preceded,
    error::InputError,
    Parser,
};

type IVec2 = (isize, isize);

fn idet(a: IVec2, b: IVec2) -> isize {
    a.0 * b.1 - a.1 * b.0
}

fn smallest_price(a: IVec2, b: IVec2, prize: IVec2) -> Option<isize> {
    fn exact_div(a: isize, b: isize) -> Option<isize> {
        if a % b == 0 {
            Some(a / b)
        } else {
            None
        }
    }

    let det = idet(a, b);

    Some(exact_div(idet(prize, b), det)? * 3 + exact_div(idet(a, prize), det)?)
}

#[test]
pub fn example_works() {
    assert_eq!(smallest_price((94, 34), (22, 67), (8400, 5400)), Some(280));
}

fn button_parser<'a>() -> impl Parser<&'a str, (char, IVec2), InputError<&'a str>> {
    preceded(
        "Button ",
        (
            winnow::token::any,
            ": X+",
            digit1.parse_to(),
            ", Y+",
            digit1.parse_to(),
        ),
    )
    .map(|(a, _, c, _, e)| (a, (c, e)))
}

fn block_parser<'a>() -> impl Parser<&'a str, (IVec2, IVec2, IVec2), InputError<&'a str>> {
    (
        button_parser(),
        line_ending,
        button_parser(),
        line_ending,
        ("Prize: X=", digit1.parse_to(), ", Y=", digit1.parse_to()),
    )
        .map(|((_, a), _, (_, b), _, (_, px, _, py))| (a, b, (px, py)))
}

#[test]
fn parser_works() {
    let (a, b, prize) = block_parser()
        .parse(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
        )
        .unwrap();

    assert_eq!(a, (94, 34));
    assert_eq!(b, (22, 67));
    assert_eq!(prize, (8400, 5400));
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?;

    let (p1, p2) = data.split("\n\n").fold((0, 0), |(p1, p2), block| {
        let mut parser = block_parser();

        let (a, b, prize) = parser.parse(block.trim()).unwrap();
        // .map_err(|e| anyhow::format_err!("{e}"))?;

        (
            p1 + smallest_price(a, b, prize).unwrap_or_default(),
            p2 + smallest_price(
                a,
                b,
                (prize.0 + 10_000_000_000_000, prize.1 + 10_000_000_000_000),
            )
            .unwrap_or_default(),
        )
    });

    println!("p1 = {p1}");
    println!("p2 = {p2}");

    Ok(())
}
