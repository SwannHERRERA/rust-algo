use std::cmp;

fn main() {}

pub fn reverte_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn merge_two_arrays(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32> {
    let mut new_array = Vec::new();
    let maxlen = array1.len() + array2.len();
    let mut i = 0;
    let mut j = 0;
    while new_array.len() < maxlen {
        if i < array1.len() && j < array2.len() {
            let min_value = cmp::min(array1[i], array2[j]);
            if min_value == array1[i] {
                i += 1;
            } else {
                j += 1;
            }
            new_array.push(min_value);
        } else {
            if i >= array1.len() {
                new_array.push(array2[j]);
                j += 1;
            } else if j >= array2.len() {
                new_array.push(array1[i]);
                i += 1;
            }
        }
    }

    return new_array;
}

pub fn count_trap(labyrinth: [[char; 5]; 4]) -> u32 {
    let mut count = 0;
    for row in labyrinth.iter() {
        for c in row.iter() {
            if *c == 'x' {
                count += 1
            }
        }
    }
    return count;
}

#[derive(Clone)]
pub struct Solution {
    vec: Vec<u8>,
}

pub fn frog_jump(n: u8, solution: Solution) -> Option<Solution> {
    let sum: u8 = solution.vec.iter().sum();
    if sum == n {
        println!("{:?}", solution.vec);
        return Some(solution);
    }
    if sum < n {
        let mut add1 = solution.clone();
        let mut add2 = solution.clone();

        add1.vec.push(1);
        add2.vec.push(2);
        frog_jump(n, add1);
        frog_jump(n, add2);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverte_string() {
        let s = String::from("Hello, world!");
        let new_string = reverte_string(&s);
        assert_eq!(new_string, "!dlrow ,olleH");
    }
    #[test]
    fn test_merge_two_arrays() {
        let array1 = vec![1, 2, 3, 6, 11, 21];
        let array2 = vec![1, 2, 4, 6, 7, 41, 62];
        let new_length = array1.len() + array2.len();
        let new_array = merge_two_arrays(array1, array2);

        assert_eq!(new_array.len(), new_length);
        assert_eq!(new_array, [1, 1, 2, 2, 3, 4, 6, 6, 7, 11, 21, 41, 62]);
    }
    #[test]
    fn test_merge_two_simple_arrays() {
        let array1 = vec![1, 2, 3];
        let array2 = vec![4, 5, 6];
        let new_array = merge_two_arrays(array1, array2);

        assert_eq!(new_array, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_count_trap() {
        let labyrinth = [
            ['x', '.', '.', '.', '.'],
            ['.', 'x', '.', 'x', '.'],
            ['x', '.', '.', 'x', '.'],
            ['x', '.', 'x', 'x', '.'],
        ];
        let result = count_trap(labyrinth);
        assert_eq!(result, 8);
    }

    /**
     * Problème de la grennouille
     * Une grennouille veut aller de l'autre coté de la rivière
     * Pour ce la elle doit sauté les 11 nénufarts
     * Elle peut faire des bon de 1 nénufart ou de 2
     * Donner toutes les possibilité de la grennouille
     */
    #[test]
    fn test_frog_jump() {
        let result = frog_jump(3, Solution { vec: Vec::new() });
        let response = [vec![2, 1], vec![1, 1, 1], vec![1, 2]];
        // assert_eq!(result, response);
    }
}
