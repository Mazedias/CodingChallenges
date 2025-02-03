// Leetcode Problem 3105

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    int longestMonotonicSubarray(vector<int>& nums) {
        int longest = 1;

        int increasing_start = 0;
        int decreasing_start = 0;

        for (int i = 1; i < nums.size(); i++) {
            longest = max(longest, max(i-increasing_start, i-decreasing_start));

            if (nums[i-1] > nums[i]) {
                increasing_start = i;
            } else if (nums[i-1] < nums[i]) {
                decreasing_start = i;
            } else {
                increasing_start = i;
                decreasing_start = i;
            }
        }

        return max(longest, int(max(nums.size()-increasing_start, nums.size()-decreasing_start)));
    }
};


class TestLongestMonotonicSubarray : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestLongestMonotonicSubarray, SimpleTestCase1) {
    vector<int> nums = {1, 4, 3, 3, 2};

    EXPECT_EQ(solution.longestMonotonicSubarray(nums), 2);
}

TEST_F(TestLongestMonotonicSubarray, SimpleTestCase2) {
    vector<int> nums = {3, 3, 3};

    EXPECT_EQ(solution.longestMonotonicSubarray(nums), 1);
}

TEST_F(TestLongestMonotonicSubarray, SimpleTestCase3) {
    vector<int> nums = {3, 2, 1};

    EXPECT_EQ(solution.longestMonotonicSubarray(nums), 3);
}

TEST_F(TestLongestMonotonicSubarray, SimpleTestCase4) {
    vector<int> nums = {1, 4, 3, 2, 1, 2, 3, 4, 5, 6};

    EXPECT_EQ(solution.longestMonotonicSubarray(nums), 6);
}