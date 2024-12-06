// Leetcode Problem 2554

#include <gtest/gtest.h>
#include <vector>
#include <unordered_set>

using namespace std;

class Solution {
public:
    int maxCount(vector<int>& banned , int n, int maxSum) {
        unordered_set<int> banned_numbers(banned.begin(), banned.end());

        int sum = 0;
        int amount = 0;

        for (int i = 1; i <= n; i++) {
            if (banned_numbers.contains(i)) {
                continue;
            }

            sum += i;
            if (sum > maxSum) {
                break;
            }
            amount++;
        }

        return amount;
    }
};


class TestMaxCount : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestMaxCount, SimpleTestCase) {
    vector<int> banned = {1, 6, 5};

    EXPECT_EQ(solution.maxCount(banned, 5, 6), 2);
}

TEST_F(TestMaxCount, NoIntegerIsAvailable) {
    vector<int> banned = {1, 2, 3, 4, 5, 6, 7};

    EXPECT_EQ(solution.maxCount(banned, 8, 1), 0);
}

TEST_F(TestMaxCount, SimpleTestCase2) {
    vector<int> bannend = {11};

    EXPECT_EQ(solution.maxCount(bannend, 7, 50), 7);
}