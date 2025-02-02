// Leetcode Problem 1752

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    bool check(vector<int>& nums) {
        bool rotationPoint = false;

        for (int i = 1; i < nums.size(); i++) {
            if ((nums[i-1] > nums[i] && rotationPoint)) {
                return false;
            } else if ((nums[i-1] > nums[i]) && !rotationPoint) {
                rotationPoint = true;
            }
        }

        if ((nums[0] < nums[nums.size() - 1]) && rotationPoint) {
            return false;
        }

        return true;
    }
};

class TestCheck : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestCheck, SimpleTestCase1) {
    vector<int> nums = {3, 4, 5, 1, 2};

    EXPECT_EQ(solution.check(nums), true);
}

TEST_F(TestCheck, SimpleTestCase2) {
    vector<int> nums = {2, 1, 3, 5};

    EXPECT_EQ(solution.check(nums), false);
}

TEST_F(TestCheck, SimpleTestCase3) {
    vector<int> nums = {1, 2, 3};

    EXPECT_EQ(solution.check(nums), true);
}