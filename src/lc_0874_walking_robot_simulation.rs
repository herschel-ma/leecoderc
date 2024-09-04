use crate::Solution;
use std::collections::HashSet;

pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (mut direction, mut coordinate) = (Direction::North, (0, 0));
        let obstacles: HashSet<(i32, i32)> =
            HashSet::from_iter(obstacles.iter().map(|x| (x[0], x[1])));

        for command in commands.iter() {
            match command {
                // turn left
                -2 => match direction {
                    Direction::North => {
                        direction = Direction::West;
                    }
                    Direction::South => {
                        direction = Direction::East;
                    }
                    Direction::West => {
                        direction = Direction::South;
                    }
                    Direction::East => {
                        direction = Direction::North;
                    }
                },
                // turn right
                -1 => match direction {
                    Direction::North => {
                        direction = Direction::East;
                    }
                    Direction::South => {
                        direction = Direction::West;
                    }
                    Direction::West => {
                        direction = Direction::North;
                    }
                    Direction::East => {
                        direction = Direction::South;
                    }
                },
                k => {
                    for _ in 0..*k {
                        match direction {
                            Direction::North => {
                                let tmp_coord = (coordinate.0, coordinate.1 + 1);
                                if obstacles.contains(&tmp_coord) {
                                    break;
                                }
                                coordinate = tmp_coord;
                            }
                            Direction::South => {
                                let tmp_coord = (coordinate.0, coordinate.1 - 1);
                                if obstacles.contains(&tmp_coord) {
                                    break;
                                }
                                coordinate = tmp_coord;
                            }
                            Direction::West => {
                                let tmp_coord = (coordinate.0 - 1, coordinate.1);
                                if obstacles.contains(&tmp_coord) {
                                    break;
                                }
                                coordinate = tmp_coord;
                            }
                            Direction::East => {
                                let tmp_coord = (coordinate.0 + 1, coordinate.1);
                                if obstacles.contains(&tmp_coord) {
                                    break;
                                }
                                coordinate = tmp_coord;
                            }
                        }
                    }
                    ans = ans.max(i32::pow(coordinate.0, 2) + i32::pow(coordinate.1, 2))
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let commands = vec![4, -1, 3];
        let obstacles: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::robot_sim(commands, obstacles), 25)
    }

    #[test]
    fn test_case_2() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        assert_eq!(Solution::robot_sim(commands, obstacles), 65)
    }

    #[test]
    fn test_case_3() {
        let commands = vec![6, -1, -1, 6];
        let obstacles: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::robot_sim(commands, obstacles), 36)
    }
}
