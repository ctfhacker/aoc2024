#![feature(array_windows)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const INPUT: &str = include_str!("../input");

#[derive(Debug)]
enum TokenType {
    Mul,
    Enable,
    Disable,
    OpenParen,
    CloseParen,
    Comma,
    Number,
    Unknown,
}

#[derive(Debug, Default)]
struct Tokens {
    indexes: Vec<u32>,
    types: Vec<TokenType>,
}

impl Tokens {
    pub fn add(&mut self, index: u32, token_type: TokenType) {
        self.indexes.push(index);
        self.types.push(token_type);
    }
}

#[derive(Debug)]
enum Operation {
    Enable,
    Disable,
    Mul { x: u32, y: u32 },
}

#[derive(Debug, Default)]
pub struct State {
    pc: u32,
    sum: u32,
    enabled: bool,
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sum: 0,
            enabled: true,
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.sum = 0;
        self.enabled = true;
        self.pc = 0;
    }

    fn execute(&mut self, op: &Operation) {
        match op {
            Operation::Enable => self.enabled = true,
            Operation::Disable => self.enabled = false,
            Operation::Mul { x, y } => {
                if !self.enabled {
                    return;
                }
                self.sum += x.wrapping_mul(*y);
            }
        }
    }

    fn run(&mut self, ops: &[Operation]) {
        while let Some(op) = ops.get(self.pc as usize) {
            self.execute(op);
            self.pc += 1;
        }
    }
}

#[derive(Debug)]
pub struct Program {
    state: State,
    ops: Vec<Operation>,
}

impl Program {
    /// Create a program from the input
    pub fn from_input(input: &str) -> Result<Self> {
        let tokens = tokenize(input.as_bytes());
        let mut i = 0;
        let mut ops = Vec::new();

        loop {
            if i >= tokens.types.len() {
                break;
            }

            if let Some(TokenType::Enable) = tokens.types.get(i) {
                ops.push(Operation::Enable);
                i += 1;
                continue;
            }

            if let Some(TokenType::Disable) = tokens.types.get(i) {
                ops.push(Operation::Disable);
                i += 1;
                continue;
            }

            if let Some(
                [
                    TokenType::Mul,
                    TokenType::OpenParen,
                    TokenType::Number,
                    TokenType::Comma,
                    TokenType::Number,
                    TokenType::CloseParen,
                ],
            ) = tokens.types.get(i..i + 6)
            {
                let [_mul, _open, num1, comma, num2, close] = tokens.indexes[i..i + 6] else {
                    unreachable!();
                };

                let x = input[num1 as usize..comma as usize]
                    .parse::<u32>()
                    .map_err(|_| format!("Failed to parse number1 at index: {num1}"))?;
                let y = input[num2 as usize..close as usize]
                    .parse::<u32>()
                    .map_err(|_| format!("Failed to parse number2 at index: {num2}"))?;

                let op = Operation::Mul { x, y };
                ops.push(op);

                i += 6;
                continue;
            }

            i += 1;
        }

        let state = State::new();
        Ok(Self { state, ops })
    }

    pub fn run(&mut self) {
        self.state.run(&self.ops);
    }
}

fn tokenize(input: &[u8]) -> Tokens {
    let mut i = 0;
    let mut tokens = Tokens::default();
    while (i as usize) < input.len() {
        match &input[i as usize..] {
            [b'd', b'o', b'(', b')', ..] => {
                tokens.add(i, TokenType::Enable);
                i += 4;
            }
            [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..] => {
                tokens.add(i, TokenType::Disable);
                i += 7;
            }
            [b'm', b'u', b'l', ..] => {
                tokens.add(i, TokenType::Mul);
                i += 3;
            }
            [b'(', ..] => {
                tokens.add(i, TokenType::OpenParen);
                i += 1;
            }
            [b')', ..] => {
                tokens.add(i, TokenType::CloseParen);
                i += 1;
            }
            [b',', ..] => {
                tokens.add(i, TokenType::Comma);
                i += 1;
            }
            [x, end, ..] if x.is_ascii_digit() && !end.is_ascii_digit() => {
                tokens.add(i, TokenType::Number);
                i += 1;
            }
            [x, y, end, ..]
                if x.is_ascii_digit() && y.is_ascii_digit() && !end.is_ascii_digit() =>
            {
                tokens.add(i, TokenType::Number);
                i += 2;
            }
            [x, y, z, end, ..]
                if x.is_ascii_digit()
                    && y.is_ascii_digit()
                    && z.is_ascii_digit()
                    && !end.is_ascii_digit() =>
            {
                tokens.add(i, TokenType::Number);
                i += 3;
            }
            _ => {
                tokens.add(i, TokenType::Unknown);
                i += 1;
            }
        }
    }

    tokens
}

fn part1(input: &str) -> Result<u32> {
    let mut program = Program::from_input(input)?;
    program.state.enabled = true;
    program.ops.retain(|x| matches!(x, Operation::Mul { .. }));
    program.run();
    Ok(program.state.sum)
}

fn part2(input: &str) -> Result<u32> {
    let mut program = Program::from_input(input)?;
    program.run();
    Ok(program.state.sum)
}

fn main() {
    println!("Part 1: {:?}", part1(INPUT));
    println!("Part 2: {:?}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT: &str = include_str!("../test_input");
    const TEST_INPUT2: &str = include_str!("../test_input2");

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 161);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(TEST_INPUT2).unwrap(), 48);
    }
}
