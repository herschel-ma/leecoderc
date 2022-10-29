pub fn flood_fill(image_g:Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut image = image_g; 
    let key = image[sr as usize][sc as usize];
    let width = image[0].len() as i32;
    let height = image.len() as i32;

    if key == color {
        return image;
    }

    image[sr as usize][sc as usize] = key;

    let mut pos_list = vec![(sr, sc)];
    while let Some(pos) = pos_list.pop() {
        let left = (pos.0, 0.max(pos.1 - 1));
        let right = (pos.0, (pos.1 + 1).min(width - 1));
        let up = (0.max(pos.0 - 1), pos.1);
        let down = ((pos.0 + 1).min(height - 1), pos.1);

        for (row, col) in [left, right, up, down] {
            if image[row as usize][col as usize] == key {
                pos_list.push((row, col));
                image[row as usize][col as usize] = color;
            }
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
       let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0 ,1]];
       let output = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
       assert_eq!(flood_fill(image, 1, 1, 2), output);
    }

    #[test]
    fn ex2() {
       let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
       let output = vec![vec![0, 0, 0], vec![0, 0, 0]];
       assert_eq!(flood_fill(image, 0, 0, 0), output);
    }
}