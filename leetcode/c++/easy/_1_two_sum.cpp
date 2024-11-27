// Leetcode Problem 1

#include <gtest/gtest.h>
#include <vector>
#include <unordered_map>

using namespace std;


class Solution {
public:
     vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> map;

        for (int i = 0; i < nums.size(); i++) {
            int complement = target - nums[i];
            if (map.find(complement) != map.end()) {
                return {map[complement], i};
            }
            map[nums[i]] = i;
        }

        return {};
    }
};

class TestTwoSum : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestTwoSum, TestCase1) {
    vector<int> nums = {2, 7, 11, 15};
    vector<int> sol = {0, 1};

    EXPECT_EQ(solution.twoSum(nums, 9), sol);
}

TEST_F(TestTwoSum, TestCase2) {
    vector<int> nums = {3, 2, 4};
    vector<int> sol = {1, 2};

    EXPECT_EQ(solution.twoSum(nums, 6), sol);
}

TEST_F(TestTwoSum, TestCase3) {
    vector<int> nums = {3, 3};
    vector<int> sol = {0, 1};

    EXPECT_EQ(solution.twoSum(nums, 6), sol);
}