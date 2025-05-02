// Leetcode Problem 838

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut characters: Vec<char> = dominoes.chars().collect();
        let n = characters.len();

        let mut position = 0;
        let mut pointer: usize;
        let mut current_direction: char = '.';
        let mut next_direction: char;
        while position < n {
            pointer = position;

            // Search next character that indicates a "push"-direction of a dominoe
            while pointer < n && characters[pointer] != 'L' && characters[pointer] != 'R' {
                pointer += 1;
            }

            // If no character was found until the end of the string we can stop
            if pointer >= n {
                break;
            }

            next_direction = characters[pointer];
            if (current_direction == '.' && (next_direction == 'R' || next_direction == '.')) || (current_direction == 'L' && next_direction == 'R') {
                // The dominoes currently looked at are not pushed in any direction
                current_direction = 'R';
            } else if current_direction == 'R' && next_direction == 'R' {
                // We only push to the right
                for i in position..pointer {
                    characters[i] = 'R';
                }
                current_direction = 'R';
            } else if (current_direction == '.' || current_direction == 'L') && next_direction == 'L' {
                // We only push to the left
                for i in position..pointer {
                    characters[i] = 'L';
                }
                current_direction = '.';
            } else if current_direction == 'R' && next_direction == 'L' {
                // Left side falls to left and right side to the right
                for i in position..position+(pointer-position)/2 {
                    characters[i] = 'R';
                }
                for i in position+(pointer-position)/2..pointer {
                    characters[i] = 'L';
                }

                // Check if there is a middle piece
                if (pointer - position) % 2 == 1 {
                    characters[position+(pointer-position)/2] = '.';
                }
                current_direction = '.';
            }

            position = pointer + 1;
        }

        // Fill the end
        if current_direction == 'R' {
            for i in position..n {
                characters[i] = 'R';
            }
        }

        characters.iter().collect::<String>()
    }
}


#[test]
fn test_push_dominoes() {
    let dominoes1 = String::from("RR.L");
    let expected1 = String::from("RR.L");
    let dominoes2 = String::from(".L.R...LR..L..");
    let expected2 = String::from("LL.RR.LLRRLL..");
    let dominoes3 = String::from("..R.R.R.LLLLL.....R...LR....L.R..");
    let expected3 = String::from("..RRRRR.LLLLL.....RR.LLRRRLLL.RRR");

    assert_eq!(Solution::push_dominoes(dominoes1), expected1);
    assert_eq!(Solution::push_dominoes(dominoes2), expected2);
    assert_eq!(Solution::push_dominoes(dominoes3), expected3);
}