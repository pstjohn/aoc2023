use nom::{
    bytes::complete::take_while_m_n,
    character::complete::{self, newline},
    multi::{many1, many_till, separated_list1},
    *,
};
use std::{fs::File, io::Read};

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_next_number(input: &str) -> IResult<&str, &str> {
    let (input, (_, number)) = many_till(
        complete::alpha0,
        take_while_m_n(1, 1, |c| nom::character::is_digit(c as u8)),
    )(input)?;
    let (input, _) = complete::alpha0(input)?;
    IResult::Ok((input, number))
}

fn parse_line(input: &str) -> IResult<&str, u32> {
    let (input, numbers) = many1(parse_next_number)(input)?;
    let all_numbers = numbers.concat();
    let mut code = String::new();
    code.push(all_numbers.chars().next().unwrap());
    code.push(all_numbers.chars().last().unwrap());
    let code: u32 = code.parse().unwrap();
    Ok((input, code))
}

fn parse_file(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(newline, parse_line)(input)
}

fn part1(filename: &str) -> anyhow::Result<u32> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // dbg!(&contents);

    let str_contents = contents.as_str();
    let (_, all_numbers) = parse_file(str_contents).expect("File parsing failed");
    dbg!(&all_numbers);

    Ok(all_numbers.iter().sum())

    // let row_sum: Vec<u32> = all_numbers.iter().map(|x| x.iter().sum()).collect();
    // dbg!(row_sum);

    // Ok(142)
}

fn main() {
    let part1_value = part1("part1_input.txt").unwrap();
    println!("Part 1: {}", &part1_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_demo() {
        assert_eq!(part1("part1_demo.txt").unwrap(), 142);
    }
}
