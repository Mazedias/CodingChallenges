// Leetcode Problem 1079

struct Solution;

fn num_tiles(nums: &mut [u8; 26]) -> i32 {
    let mut total = 1;
    for i in 0..26 {
        if nums[i] > 0 {
            nums[i] -= 1;
            total += num_tiles(nums);
            nums[i] += 1;
        }
    }
    total
}

#[allow(dead_code)]
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0; 26];
        for byte in tiles.bytes() {
            counts[(byte - b'A') as usize] += 1;
        }

        num_tiles(&mut counts) - 1
    }
}

/**
 * My Solution, passes my test but overflows the stack on leetcode
 * 

fn combinate(map: &mut HashMap<char, i32>, coutner: &mut i32) {
    let keys: Vec<char> = map.keys().cloned().collect();

    for key in keys {
        if let Some(_) = map.get_mut(&key) {
            *coutner += 1;
            map.entry(key).and_modify(|v| *v -= 1);

            combinate(map, coutner);

            map.entry(key).and_modify(|v| *v += 1);
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        if tiles.len() == 1 {
            return 1;
        }

        let mut amount = 0;

        let mut frequency: HashMap<char, i32> = HashMap::new();
        for c in tiles.chars() {
            frequency.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }

        combinate(&mut frequency, &mut amount);

        amount
    }
}
*/

#[test]
fn test_num_tile_possibilities() {
    let s1 = String::from("AAB");
    let s2 = String::from("AAABBC");
    let s3 = String::from("ABC");
    let s4 = String::from("V");

    assert_eq!(Solution::num_tile_possibilities(s1), 8);
    assert_eq!(Solution::num_tile_possibilities(s2), 188);
    assert_eq!(Solution::num_tile_possibilities(s3), 12);
    assert_eq!(Solution::num_tile_possibilities(s4), 1);
}