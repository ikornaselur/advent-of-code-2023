use advent_core::error::AdventError;
use advent_core::invalid_coordinate;

const INPUT: &str = include_str!("../input.txt");

struct Schematic {
    rows: Vec<String>,
    width: usize,
    height: usize,
}

trait Symbol {
    fn is_symbol(&self) -> bool;
}

impl Symbol for char {
    fn is_symbol(&self) -> bool {
        !self.is_ascii_digit() && self != &'.'
    }
}

enum Direction {
    Above,
    Below,
    Right,
    Left,
    AboveLeft,
    AboveRight,
    BelowLeft,
    BelowRight,
}

impl Schematic {
    fn from_str(input: &str) -> Self {
        let rows: Vec<String> = input.lines().map(|line| line.to_string()).collect();
        let height = rows.len();
        let width = rows[0].len();
        Self {
            rows,
            width,
            height,
        }
    }

    /// Returns a list of all the part numbers in the schematic.
    ///
    /// A number in the schematic is considered a part number only if it is adjacent to any symbol
    fn get_part_numbers(&self) -> Result<Vec<u32>, AdventError> {
        let mut part_numbers = Vec::new();

        for (row_index, row) in self.rows.iter().enumerate() {
            let mut number = 0; // We can treat 0 as not a number, because we need to return the
            let mut is_part_number = false;

            for (col_index, col) in row.chars().enumerate() {
                if col.is_ascii_digit() {
                    number =
                        number * 10 + col.to_digit(10).ok_or(AdventError::InvalidDigit(col))?;
                    // Check adjacent cells to see if there are symbols
                    if self.is_adjacent_to_symbol(row_index, col_index)? {
                        is_part_number = true;
                    }
                }
                if (!col.is_ascii_digit() || col_index == self.width - 1) && number > 0 {
                    if is_part_number {
                        part_numbers.push(number);
                    }
                    number = 0;
                    is_part_number = false;
                }
            }
        }

        Ok(part_numbers)
    }

    /// Return the charater, if any, adjacent to a coordinate
    fn get_char(
        &self,
        direction: Direction,
        row_index: usize,
        col_index: usize,
    ) -> Result<Option<char>, AdventError> {
        // Guard against out of bounds
        match direction {
            Direction::Above | Direction::AboveLeft | Direction::AboveRight if row_index == 0 => {
                return Ok(None)
            }
            Direction::Below | Direction::BelowLeft | Direction::BelowRight
                if row_index >= self.height - 1 =>
            {
                return Ok(None)
            }
            Direction::Left | Direction::AboveLeft | Direction::BelowLeft if col_index == 0 => {
                return Ok(None)
            }
            Direction::Right | Direction::AboveRight | Direction::BelowRight
                if col_index >= self.width - 1 =>
            {
                return Ok(None)
            }
            _ => {}
        }

        let (x, y) = match direction {
            Direction::Above => (row_index - 1, col_index),
            Direction::Below => (row_index + 1, col_index),
            Direction::Right => (row_index, col_index + 1),
            Direction::Left => (row_index, col_index - 1),
            Direction::AboveLeft => (row_index - 1, col_index - 1),
            Direction::AboveRight => (row_index - 1, col_index + 1),
            Direction::BelowLeft => (row_index + 1, col_index - 1),
            Direction::BelowRight => (row_index + 1, col_index + 1),
        };

        Ok(match (x, y) {
            (x, y) if x >= self.height || y >= self.width => None,
            (x, y) => Some(
                self.rows[x]
                    .chars()
                    .nth(y)
                    .ok_or(invalid_coordinate!(row_index, col_index))?,
            ),
        })
    }

    /// Return if there is a symbol at adjacent cells
    fn is_adjacent_to_symbol(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Result<bool, AdventError> {
        for direction in [
            Direction::Above,
            Direction::Below,
            Direction::Right,
            Direction::Left,
            Direction::AboveLeft,
            Direction::AboveRight,
            Direction::BelowLeft,
            Direction::BelowRight,
        ] {
            if let Some(c) = self.get_char(direction, row_index, col_index)? {
                if c.is_symbol() {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<u32, AdventError> {
    let schematic = Schematic::from_str(input);

    Ok(schematic.get_part_numbers()?.iter().sum())
}

fn part2(input: &str) -> Result<u32, AdventError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_1_TEST_INPUT: &str = include_str!("../part_1_test.txt");
    const PART_2_TEST_INPUT: &str = include_str!("../part_2_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(PART_1_TEST_INPUT).unwrap(), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(PART_2_TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_schematic_from_str() {
        let schematic = Schematic::from_str(".#.\n123\n$*#");

        assert_eq!(schematic.rows, vec![".#.", "123", "$*#"]);
    }

    #[test]
    fn test_get_char() {
        let schematic = Schematic::from_str(".#.\n123\n$*#");

        assert_eq!(schematic.get_char(Direction::Above, 0, 1).unwrap(), None);
        assert_eq!(
            schematic.get_char(Direction::Above, 1, 1).unwrap(),
            Some('#')
        );
        assert_eq!(
            schematic.get_char(Direction::Below, 1, 1).unwrap(),
            Some('*')
        );
        assert_eq!(
            schematic.get_char(Direction::Right, 1, 1).unwrap(),
            Some('3')
        );
        assert_eq!(
            schematic.get_char(Direction::Left, 1, 1).unwrap(),
            Some('1')
        );
        assert_eq!(schematic.get_char(Direction::Right, 2, 2).unwrap(), None);
        assert_eq!(schematic.get_char(Direction::Below, 2, 2).unwrap(), None);
        assert_eq!(
            schematic.get_char(Direction::AboveLeft, 1, 1).unwrap(),
            Some('.')
        );
        assert_eq!(
            schematic.get_char(Direction::BelowLeft, 1, 1).unwrap(),
            Some('$')
        );
        assert_eq!(
            schematic.get_char(Direction::BelowRight, 1, 1).unwrap(),
            Some('#')
        );
    }

    #[test]
    fn test_is_symbol() {
        assert!('#'.is_symbol());
        assert!('*'.is_symbol());
        assert!('$'.is_symbol());
        assert!(!'.'.is_symbol());
        assert!(!'7'.is_symbol());
    }

    #[test]
    fn test_is_adjacent_to_symbol() {
        let schematic = Schematic::from_str("...\n123\n..#");

        assert!(!schematic.is_adjacent_to_symbol(1, 0).unwrap());
        assert!(schematic.is_adjacent_to_symbol(1, 1).unwrap());
        assert!(schematic.is_adjacent_to_symbol(1, 2).unwrap());
    }

    #[test]
    fn test_get_part_numbers() {
        let schematic = Schematic::from_str(".#.\n123\n..#\n...\n456");
        assert_eq!(schematic.get_part_numbers().unwrap(), vec![123]);
    }
}
