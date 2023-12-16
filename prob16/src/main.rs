use advent_core::error::AdventError;
use advent_core::generic_error;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

type Coordinate = (usize, usize);
type Beam = (Coordinate, Direction);

#[derive(Debug, PartialEq)]
struct Layout {
    grid: Vec<Vec<Node>>,
}

fn get_next_coordinate(beam: Beam) -> Coordinate {
    match beam {
        ((x, y), Direction::Right) => (x, y + 1),
        ((x, y), Direction::Down) => (x + 1, y),
        ((x, y), Direction::Left) => (x, y - 1),
        ((x, y), Direction::Up) => (x - 1, y),
    }
}

impl Layout {
    /// Send a beam through the layout
    ///
    /// Keep track of all the nodes it passes through
    fn beam(&self) -> Result<usize, AdventError> {
        // The current beams we are tracking, containing the current node and the direction we will
        // take in the next iteration
        // We start in the top right (0, 0) and go right
        let mut queue: VecDeque<Beam> = VecDeque::new();

        // Keep track of the paths we have taken, which are the coordinate and the direction
        // There is no need to repeatedly following the beam again if it loops back around
        let mut paths_taken: HashSet<Beam> = HashSet::new();

        // Handle the first node separately
        let first_node = self.grid[0][0];
        match first_node {
            Node::Empty | Node::Horizontal => {
                // We just continue in the same direction
                queue.push_back(((0, 0), Direction::Right));
            }
            Node::Up => {
                // Uhm.. we just go right off ASAP
                return Ok(1);
            }
            Node::Down | Node::Vertical => {
                // We are going to start by going down instead
                queue.push_back(((0, 0), Direction::Down));
            }
        }

        while let Some(beam) = queue.pop_front() {
            if paths_taken.contains(&beam) {
                continue;
            }
            paths_taken.insert(beam);

            if self.beam_going_off_grid(beam) {
                continue;
            }

            // Get the next node
            let coord = get_next_coordinate(beam);
            let node = self.grid[coord.0][coord.1];
            match (node, beam.1) {
                (Node::Empty, dir) => {
                    // Beam just continues in the same direction
                    queue.push_back((coord, dir));
                }
                (Node::Horizontal, dir) => {
                    if dir == Direction::Right || dir == Direction::Left {
                        // We just pass through
                        queue.push_back((coord, dir));
                    } else {
                        // The beam splits and goes left and right
                        queue.push_back((coord, Direction::Left));
                        queue.push_back((coord, Direction::Right));
                    }
                }
                (Node::Vertical, dir) => {
                    if dir == Direction::Up || dir == Direction::Down {
                        // We just pass through
                        queue.push_back((coord, dir));
                    } else {
                        // The beam splits and goes up and down
                        queue.push_back((coord, Direction::Up));
                        queue.push_back((coord, Direction::Down));
                    }
                }
                (Node::Up, dir) => {
                    match dir {
                        Direction::Right => {
                            // Beam goes up
                            queue.push_back((coord, Direction::Up));
                        }
                        Direction::Down => {
                            // Beam goes left
                            queue.push_back((coord, Direction::Left));
                        }
                        Direction::Left => {
                            // Beam goes down
                            queue.push_back((coord, Direction::Down));
                        }
                        Direction::Up => {
                            // Beam goes right
                            queue.push_back((coord, Direction::Right));
                        }
                    }
                }
                (Node::Down, dir) => {
                    match dir {
                        Direction::Right => {
                            // Beam goes down
                            queue.push_back((coord, Direction::Down));
                        }
                        Direction::Down => {
                            // Beam goes right
                            queue.push_back((coord, Direction::Right));
                        }
                        Direction::Left => {
                            // Beam goes up
                            queue.push_back((coord, Direction::Up));
                        }
                        Direction::Up => {
                            // Beam goes left
                            queue.push_back((coord, Direction::Left));
                        }
                    }
                }
            }
        }

        // We need to count the nodes that have been visited, so we can count just the coordinates,
        // ignoring the directions
        let coords = paths_taken
            .iter()
            .map(|(coord, _)| coord)
            .collect::<HashSet<_>>();

        /*
        // Debug print the layout
        for (x, row) in self.grid.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                if coords.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        */

        Ok(coords.len())
    }

    fn beam_going_off_grid(&self, beam: Beam) -> bool {
        match beam {
            ((x, _), Direction::Down) if x >= self.grid.len() - 1 => true,
            ((_, y), Direction::Right) if y >= self.grid[0].len() - 1 => true,
            ((0, _), Direction::Up) => true,
            ((_, 0), Direction::Left) => true,
            _ => false,
        }
    }
}

impl FromStr for Layout {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(Node::try_from)
                    .collect::<Result<Vec<Node>, AdventError>>()
            })
            .collect::<Result<Vec<Vec<Node>>, AdventError>>()?;

        Ok(Self { grid })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Node {
    Empty,      // .
    Horizontal, // -
    Vertical,   // |
    Up,         // /
    Down,       // \
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl TryFrom<char> for Node {
    type Error = AdventError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Node::Empty),
            '-' => Ok(Node::Horizontal),
            '|' => Ok(Node::Vertical),
            '/' => Ok(Node::Up),
            '\\' => Ok(Node::Down),
            _ => Err(generic_error!("Unknown node type: {}", c)),
        }
    }
}

fn main() -> Result<(), AdventError> {
    println!("## Part 1");
    println!(" > {}", part1(INPUT)?);

    println!("## Part 2");
    println!(" > {}", part2(INPUT)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize, AdventError> {
    let layout: Layout = input.parse()?;

    Ok(layout.beam()?)
}

fn part2(input: &str) -> Result<usize, AdventError> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), 0);
    }

    #[test]
    fn test_layout_from_str() {
        let layout: Layout = ".|.\n-..\n/\\.".parse().unwrap();

        assert_eq!(
            layout,
            Layout {
                grid: vec![
                    vec![Node::Empty, Node::Vertical, Node::Empty],
                    vec![Node::Horizontal, Node::Empty, Node::Empty],
                    vec![Node::Up, Node::Down, Node::Empty],
                ]
            }
        );
    }

    #[test]
    fn test_layout_beam_going_off_grid() {
        let layout: Layout = "...\n...\n...".parse().unwrap();

        assert!(layout.beam_going_off_grid(((0, 0), Direction::Up)));
        assert!(layout.beam_going_off_grid(((0, 0), Direction::Left)));
        assert!(layout.beam_going_off_grid(((0, 2), Direction::Right)));
        assert!(layout.beam_going_off_grid(((2, 0), Direction::Down)));
    }

    #[test]
    fn test_beam_empty_layout() {
        let layout: Layout = "...\n...\n...".parse().unwrap();

        // Should just pass straight through
        assert_eq!(layout.beam().unwrap(), 3);
    }

    #[test]
    fn test_beam_simple_mirror() {
        let layout: Layout = "..\\\n...\n...".parse().unwrap();

        // Should redirect down in the corner
        assert_eq!(layout.beam().unwrap(), 5);
    }

    #[test]
    fn test_beam_more_complex() {
        let layout: Layout = ".\\.\n.-.\n...".parse().unwrap();

        // Should redirect down in the middle, then split to left and right
        assert_eq!(layout.beam().unwrap(), 5);
    }

    #[test]
    fn test_loops() {
        let layout: Layout = ".\\.\n/-.\n\\/.".parse().unwrap();

        // The beam will split in the middle and go around in a loop, which should be ignored and
        // all be good
        assert_eq!(layout.beam().unwrap(), 7);
    }

    #[test]
    fn test_immediate_mirror() {
        let layout: Layout = "\\/.\n...\n\\..".parse().unwrap();

        // The beam should go down immediately, then again in the corner to the right
        assert_eq!(layout.beam().unwrap(), 5);
    }
}