// Leetcode Problem 2966

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    vector<vector<int>> divideArray(vector<int>& nums, int k) {
        size_t n = nums.size();

        sort(nums.begin(), nums.end());

        vector<vector<int>> solution = {};
        for (int i = 0; i < n/3; i++) {
            if (nums[i*3+2] - nums[i*3] > k) {
                return {};
            }
            solution.push_back({nums[i*3], nums[i*3+1], nums[i*3+2]});
        }
        return solution;
    }
};


class TestDivideArray : public ::testing::Test {
protected:
    Solution solution;

    bool checkArray(vector<vector<int>> arr, int k) {
        for (vector<int> v : arr) {
            sort(v.begin(), v.end());
            if (v[2] - v[0] > k) {
                return false;
            }
        }
        return true;
    }
};

TEST_F(TestDivideArray, SimpleTestCase1) {
    vector<int> nums = {1,3,4,8,7,9,3,5,1};

    EXPECT_TRUE(this->checkArray(solution.divideArray(nums, 2), 2));
}

TEST_F(TestDivideArray, SimpleTestCase2) {
    vector<int> nums = {2,4,2,2,5,2};

    EXPECT_TRUE(this->checkArray(solution.divideArray(nums, 2), 2));
}

TEST_F(TestDivideArray, SimpleTestCase3) {
    vector<int> nums = {4,2,9,8,2,12,7,12,10,5,8,5,5,7,9,2,5,11};

    EXPECT_TRUE(this->checkArray(solution.divideArray(nums, 14), 14));
}

