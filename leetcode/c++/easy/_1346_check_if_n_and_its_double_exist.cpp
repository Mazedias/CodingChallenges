// Leetcode Problem 1346

#include <gtest/gtest.h>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
public:
    bool checkIfExist(vector<int>& arr) {
        unordered_map<int, int> seen;

        for (size_t i = 0; i < arr.size(); i++) {
            if (seen.contains(arr[i] * 2)) {
                return true;
            }

            if (arr[i] % 2 == 0 && seen.contains(arr[i] / 2)) {
                return true;
            }

            seen.insert({arr[i], i});
        }

        return false;
    }
};


class TestClass : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestClass, ForwardSearchPossible) {
    vector<int> arr = {10, 2, 5, 3};

    EXPECT_TRUE(solution.checkIfExist(arr));
}

TEST_F(TestClass, BackwardsSearchNeeded) {
    vector<int> arr = {7, 1, 14, 11};

    EXPECT_TRUE(solution.checkIfExist(arr));
}

TEST_F(TestClass, NoSolutionNormalCase) {
    vector<int> arr = {3, 1, 7, 11};

    EXPECT_FALSE(solution.checkIfExist(arr));
}

TEST_F(TestClass, CheckIfZeroWorks) {
    vector<int> arr = {0, 0};

    EXPECT_TRUE(solution.checkIfExist(arr));
}