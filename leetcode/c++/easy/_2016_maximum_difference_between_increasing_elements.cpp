// Leetcode Problem 2016

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    int maximumDifference(vector<int>& nums) {
        int diff = -1;
        for (int i = 0; i < nums.size(); i++) {
            int pos = i + 1;

            while (pos < nums.size()) {
                if (nums[i] < nums[pos] && nums[pos] - nums[i] > diff) {
                    diff = nums[pos] - nums[i];
                }
                pos++;
            }
        }
        return diff;
    }
};


class TestMaximumDifference : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestMaximumDifference, SimpleTestCase1) {
    vector<int> nums = {7, 1, 5, 4};

    EXPECT_EQ(solution.maximumDifference(nums), 4);
}

TEST_F(TestMaximumDifference, SimpleTestCase2) {
    vector<int> nums = {9, 4, 3, 2};

    EXPECT_EQ(solution.maximumDifference(nums), -1);
}

TEST_F(TestMaximumDifference, SimpleTestCase3) {
    vector<int> nums = {1, 5, 2, 10};

    EXPECT_EQ(solution.maximumDifference(nums), 9);
}