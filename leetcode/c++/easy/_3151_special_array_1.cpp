// Leetcode Problem 3151

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    bool isArraySpecial(vector<int>& nums) {
        if (nums.size() < 2) {
            return true;
        }

        for (int i = 0; i < nums.size() - 1; i++) {
            if ((nums[i] % 2) == (nums[i + 1] % 2)) {
                return false;
            }
        }
        return true;
    }
};

class TestIsArraySpecial : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestIsArraySpecial, SimpleTestCase1) {
    vector<int> nums = {1};

    EXPECT_EQ(solution.isArraySpecial(nums), true);
}

TEST_F(TestIsArraySpecial, SimpleTestCase2) {
    vector<int> nums = {2, 1, 4};

    EXPECT_EQ(solution.isArraySpecial(nums), true);
}

TEST_F(TestIsArraySpecial, SimpleTestCase2) {
    vector<int> nums = {4, 3, 1, 6};

    EXPECT_EQ(solution.isArraySpecial(nums), false);
}