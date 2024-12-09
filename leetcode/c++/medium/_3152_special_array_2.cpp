// Leetcode Problem 3152

#include <gtest/gtest.h>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    vector<bool> isArraySpecial(vector<int>& nums, vector<vector<int>>& queries) {
        unordered_map<int, int> positions;

        int section = 0;
        bool was_last_even = nums[0] % 2 == 0;

        positions.insert({0, 0});

        for (int i = 1; i < nums.size(); i++) {
            if ((was_last_even && nums[i] % 2 != 0) || (!was_last_even && nums[i] % 2 == 0)) {
                positions.insert({i, section});
            } else {
                section++;
                positions.insert({i, section});
            }
            was_last_even = nums[i] % 2 == 0;
        }

        vector<bool> solution;
        for (int i = 0; i < queries.size(); i++) {
            if (positions[queries[i][0]] != positions[queries[i][queries[i].size() - 1]]) {
                solution.push_back(false);
            } else {
                solution.push_back(true);
            }
        }

        return solution;
    }
};


class TestIsArraySpecial : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestIsArraySpecial, SimpleSingleQueryCase) {
    vector<int> nums = {3, 4, 1, 2, 6};
    vector<vector<int>> queries = {{0, 4}};
    vector<bool> expected = {false};

    EXPECT_EQ(solution.isArraySpecial(nums, queries), expected);
}

TEST_F(TestIsArraySpecial, SimpleMultipleQueriesCase) {
    vector<int> nums = {4, 3, 1, 6};
    vector<vector<int>> queries = {{0, 2}, {2, 3}};
    vector<bool> expected = {false, true};

    EXPECT_EQ(solution.isArraySpecial(nums, queries), expected);
}