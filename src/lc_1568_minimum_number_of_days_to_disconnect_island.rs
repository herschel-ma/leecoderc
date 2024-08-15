use crate::Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Helper function to check if a cell is within grid bounds
        fn is_valid(grid: &[Vec<i32>], x: i32, y: i32) -> bool {
            x >= 0
                && y >= 0
                && x < grid.len() as i32
                && y < grid[0].len() as i32
                && grid[x as usize][y as usize] == 1
        }

        // Helper function to perform BFS and return all cells of an island
        fn bfs(
            grid: &[Vec<i32>],
            start: (i32, i32),
            directions: &Vec<(i32, i32)>,
            visited: &mut HashSet<(i32, i32)>,
        ) -> Vec<(i32, i32)> {
            let mut queue = VecDeque::new();

            queue.push_back(start);
            visited.insert(start);

            let mut island = Vec::new();
            while let Some((x, y)) = queue.pop_front() {
                island.push((x, y));
                for &(dx, dy) in directions {
                    let nx = x + dx;
                    let ny = y + dy;
                    if is_valid(grid, nx, ny) && !visited.contains(&(nx, ny)) {
                        queue.push_back((nx, ny));
                        visited.insert((nx, ny));
                    }
                }
            }
            island
        }

        fn is_disconnected(grid: &[Vec<i32>], directions: &Vec<(i32, i32)>) -> bool {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut count = 0;

            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 1 && !visited.contains(&(i as i32, j as i32)) {
                        if count > 0 {
                            return true;
                        };
                        bfs(grid, (i as i32, j as i32), directions, &mut visited);
                        count += 1;
                    }
                }
            }
            count != 1
        }

        let mut land_count = 0;
        // count land cells
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    land_count += 1;
                }
            }
        }
        // check initial state
        if land_count == 0 || is_disconnected(&grid, &directions) {
            return 0;
        }
        if land_count == 1 {
            return 1;
        };
        // try removing each land cell;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    grid[row][col] = 0;
                    if is_disconnected(&grid, &directions) {
                        return 1;
                    }
                    grid[row][col] = 1;
                }
            }
        }
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let grid = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
        assert_eq!(Solution::min_days(grid), 2);
    }

    #[test]
    fn test_case_2() {
        let grid = vec![vec![1, 1]];
        assert_eq!(Solution::min_days(grid), 2);
    }
}
