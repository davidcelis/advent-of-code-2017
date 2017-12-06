const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    println!("Part 1: {}", solve(&input, 1));
    println!("Part 2: {}", solve(&input, input.len() / 2));
}

fn solve(numbers: &[u32], offset: usize) -> u32 {
    numbers.iter().
        zip(numbers.iter().cycle().skip(offset)).
        filter_map(|(n, o)| if n == o { Some(n) } else { None }).
        sum()
}

fn parse_input(input: &'static str) -> Vec<u32> {
    input.trim().chars().
        map(|c| c.to_digit(10).unwrap()).
        collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part_1() {
        assert_eq!(3, solve(&parse_input("1122"),     1));
        assert_eq!(4, solve(&parse_input("1111"),     1));
        assert_eq!(0, solve(&parse_input("1234"),     1));
        assert_eq!(9, solve(&parse_input("91212129"), 1));
    }

    #[test]
    fn it_solves_part_2() {
        assert_eq!(6,  solve(&parse_input("1212"),     2));
        assert_eq!(0,  solve(&parse_input("1221"),     2));
        assert_eq!(4,  solve(&parse_input("123425"),   3));
        assert_eq!(12, solve(&parse_input("123123"),   3));
        assert_eq!(4,  solve(&parse_input("12131415"), 4));
    }
}
