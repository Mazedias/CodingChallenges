// Leetcode Problem 3169

struct Solution;

#[allow(dead_code)]
impl Solution {
    // Second approach (iterate over the meetings)
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        if meetings.is_empty() {
            return days;
        }

        meetings.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut free_days = 0;
        let mut current_day = 1;

        // Loop through the meetings and calculate free days between the meetings
        for meeting in &meetings {
            if current_day < meeting[0] {
                free_days += meeting[0] - current_day;
            }

            current_day = current_day.max(meeting[1] + 1);

            if current_day > days {
                return free_days;
            }
        }

        // Calculate free days after the last meeting
        if current_day <= days {
            free_days += days - current_day + 1;
        }

        free_days
    }

    // First approach (iterating over the days) but very slow for big numbers
    pub fn count_days_slow(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut free_days: i32 = 0;
        let mut meeting_pointer: usize = 0;
        let n = meetings.len();

        let mut day: i32 = 1;
        while day <= days {
            // Move meeting pointer to the next relevant meeting (skip past meetings that are in the past)
            while (meeting_pointer + 1 < n) && (meetings[meeting_pointer][0] < day) && (meetings[meeting_pointer][1] < day) {
                meeting_pointer += 1;
            }

            // If there is meeting on the current day skip the day
            if meetings[meeting_pointer][0] <= day && meetings[meeting_pointer][1] >= day {
                day = meetings[meeting_pointer][1] + 1;
                continue;
            }
            // No meetings -> increase free day counter
            free_days += 1;
            day += 1;
        }

        free_days
    }
}

#[test]
pub fn test_count_days() {
    let meetings1 = vec![vec![5, 7], vec![1, 3], vec![9, 10]];
    let meetings2 = vec![vec![2, 4], vec![1, 3]];
    let meetings3 = vec![vec![1, 6]];
    let meetings4 = vec![vec![1, 2], vec![4, 10], vec![5, 10], vec![7, 10], vec![8, 10], vec![9, 10], vec![13, 1000], vec![500, 1000]];
    let meetings5 = vec![vec![1, 1_000_000_000]];
    let meetings6 = vec![vec![1, 1], vec![1_000_000_000, 1_000_000_000]];
    let meetings7 = vec![vec![3, 49], vec![23, 44], vec![21, 56], vec![26, 55], vec![23, 52], vec![2, 9], vec![1, 48], vec![3, 31]];

    assert_eq!(Solution::count_days(10, meetings1), 2);
    assert_eq!(Solution::count_days(5, meetings2), 1);
    assert_eq!(Solution::count_days(6, meetings3), 0);
    assert_eq!(Solution::count_days(1000, meetings4), 3);
    assert_eq!(Solution::count_days(1_000_000_000, meetings5), 0);
    assert_eq!(Solution::count_days(1_000_000_000, meetings6), 999_999_998);
    assert_eq!(Solution::count_days(57, meetings7), 1);
}