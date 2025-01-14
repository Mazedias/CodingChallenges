// Leetcode Problem 15

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> triplets; 
        
        stable_sort(nums.begin(), nums.end());

        for (int i = 0; i < nums.size(); i++) {
            if (i > 0 && nums[i] == nums[i - 1]) {
                continue;
            }

            int left = i + 1;
            int right = nums.size() - 1;

            while (left < right) {
                int sum = nums[i] + nums[left] + nums[right];

                if (sum > 0) {
                    right--;
                } else if (sum < 0) {
                    left++;
                } else {
                    triplets.push_back({nums[i], nums[left], nums[right]});
                    left++;

                    while (nums[left] == nums[left - 1] && left < right) {
                        left++;
                    }
                }
            }
        }

        return triplets;
    }
};


class TestThreeSum : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestThreeSum, SimpleTestCase1) {
    vector<int> nums = {-1, 0, 1, 2, -1, -4};
    vector<vector<int>> expected = {{-1, -1, 2}, {-1, 0, 1}};

    EXPECT_EQ(solution.threeSum(nums), expected);
}

TEST_F(TestThreeSum, SimpleTestCase2) {
    vector<int> nums = {0, 1, 1};
    vector<vector<int>> expected = {};

    EXPECT_EQ(solution.threeSum(nums), expected);
}

TEST_F(TestThreeSum, SimpleTestCase3) {
    vector<int> nums = {0, 0, 0};
    vector<vector<int>> expected = {{0, 0, 0}};

    EXPECT_EQ(solution.threeSum(nums), expected);
}