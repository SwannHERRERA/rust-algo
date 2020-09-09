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
        let right_element = select_right_element_to_push(i, j, &array1, &array2);
        println!(
            "1 (i: {}, element: {})\n2 (j: {}, element: {})\n right_element: {}\n array: {:?}",
            i, array1[i], j, array2[j], right_element, new_array
        );
        if i < array1.len() - 1 && right_element == array1[i] {
            i += 1;
        } else {
            j += 1;
        }
        new_array.push(right_element);
    }
    return new_array;
}

fn select_right_element_to_push(i: usize, j: usize, array1: &Vec<i32>, array2: &Vec<i32>) -> i32 {
    // println!(
    //     "i: {}, len : {}, res {}",
    //     i,
    //     array1.len(),
    //     i >= array1.len()
    // );
    if i >= array1.len() - 1 {
        return array2[j];
    }
    if j >= array2.len() - 1 {
        return array1[i];
    }

    return cmp::min(array1[i], array2[j]);
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
    // #[test]
    // fn test_merge_two_complexe_arrays() {
    //     let array1 = vec![1, 2, 3, 6, 11, 21];
    //     let array2 = vec![1, 2, 4, 6, 7, 41, 62];
    //     let new_array = merge_two_arrays(array1, array2);

    //     assert_eq!(new_array, [1, 1, 2, 2, 3, 4, 6, 6, 7, 11, 21, 41, 62]);
    // }
    // #[test]
    // fn test_merge_two_simple_arrays() {
    //     let array1 = vec![1, 2, 3];
    //     let array2 = vec![4, 5, 6];
    //     let new_array = merge_two_arrays(array1, array2);

    //     assert_eq!(new_array, [1, 2, 3, 4, 5, 6]);
    // }

    #[test]
    fn test_access() {
        let array1 = vec![1, 2, 3];

        assert_eq!(array1[2], 3);
    }
}
