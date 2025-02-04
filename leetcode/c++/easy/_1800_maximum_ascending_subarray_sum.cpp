// Leetcode Problem 1800

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    int maxAscendingSum(vector<int>& nums) {
        int max_sum = nums[0];
        int temp_sum = nums[0];

        for (int i = 1; i < nums.size(); i++) {
            if (nums[i-1] >= nums[i]) {
                max_sum = max(max_sum, temp_sum);
                temp_sum = nums[i];
            } else {
                temp_sum += nums[i];
            }
        }

        return max(max_sum, temp_sum);
    }
};


class TestMaxAscendingSum : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestMaxAscendingSum, SimpleTestCase1) {
    vector<int> nums = {10, 20, 30, 5, 10, 50};

    EXPECT_EQ(solution.maxAscendingSum(nums), 65);
}

TEST_F(TestMaxAscendingSum, SimpleTestCase2) {
    vector<int> nums = {10, 20, 30, 40, 50};

    EXPECT_EQ(solution.maxAscendingSum(nums), 150);
}

TEST_F(TestMaxAscendingSum, SimpleTestCase3) {
    vector<int> nums = {12, 17, 15, 13, 10, 11, 12};

    EXPECT_EQ(solution.maxAscendingSum(nums), 33);
}

TEST_F(TestMaxAscendingSum, SimpleTestCase4) {
    vector<int> nums = {3, 6, 10, 1, 8, 9, 9, 8, 9};

    EXPECT_EQ(solution.maxAscendingSum(nums), 19);
}