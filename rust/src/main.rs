use std::collections::HashMap;

fn main() {
    let result = two_sum(vec![2, 7, 11, 15], 18);
    print!("{:?}", result);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // declare a map to store the index of the value
    let mut map = HashMap::with_capacity(nums.len());
    // iterate over the nums and store the index of the value
    for (current_index, value) in nums.iter().enumerate() {
        // calculate the search value
        let search = target - value;
        // check if the search value is in the map
        if let Some(stored_index) = map.get(&search) {
            // if the search value is in the map, return the index of the value
            return vec![current_index as i32, *stored_index as i32];
        }
        // insert the index of the value into the map
        map.insert(value, current_index);
    }
    // if the search value is not in the map, return an empty vector
    return vec![];
}
