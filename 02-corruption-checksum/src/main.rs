const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    println!("Part 1: {}", solve_part_1(&input));
    println!("Part 2: {}", solve_part_2(&input));
}

fn solve_part_1(matrix: &[Vec<u32>]) -> u32 {
    matrix.iter().map(largest_difference).sum()
}

fn solve_part_2(matrix: &[Vec<u32>]) -> u32 {
    matrix.iter().map(even_division).sum()
}

fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    input.trim().
        lines().
        map(parse_line).
        collect()
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().
        map(|word| word.parse::<u32>().unwrap()).
        collect()
}

fn largest_difference(row: &Vec<u32>) -> u32 {
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();

    max - min
}

fn even_division(row: &Vec<u32>) -> u32 {
    for x in row {
        for y in row {
            if x % y == 0 && x != y {
                return x / y
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part_1() {
        let input = parse_input("5 1 9 5\n7 5 3\n2 4 6 8");

        assert_eq!(18, solve_part_1(&input));
    }

    #[test]
    fn it_solves_part_2() {
        let input = parse_input("5 9 2 8\n9 4 7 3\n3 8 6 5");

        assert_eq!(9, solve_part_2(&input));
    }
}
