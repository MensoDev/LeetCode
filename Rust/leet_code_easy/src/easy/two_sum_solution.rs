use std::collections::HashMap;

pub fn run() {
    let response = two_sum(vec!(2,7,11,15), 9);
    println!("EAST => TWO SUM => RESPONSE => || {:?} ||", response);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut index: i32 = 0;
    for number in nums {

        
        let current = target - number;

        if map.contains_key(&current) {
            return vec![map[&current], index];
        }

        map.insert(number, index);
        index += 1;
    }

    Vec::new()
}
