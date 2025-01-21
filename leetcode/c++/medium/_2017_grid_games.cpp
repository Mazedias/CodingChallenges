// Leetcode Problem 2017

#include <gtest/gtest.h>
#include <numeric>
#include <algorithm>

using namespace std;

class Solution {
public:
    long long gridGame(vector<vector<int>>& grid) {
        int n = grid[0].size();

        long long top_sum = accumulate(grid[0].begin() + 1, grid[0].end(), 0LL);
        long long bot_sum = 0;

        vector<long long> combinations(n, 0);

        combinations[0] = max(top_sum, bot_sum);

        for (int i = 1; i < n; i++) {
            top_sum -= grid[0][i];
            bot_sum += grid[1][i - 1];

            combinations[i] = max(top_sum, bot_sum);
        }

        return *min_element(combinations.begin(), combinations.end());
    }
};


class TestGridGame : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestGridGame, SimpleTestCase1) {
    vector<vector<int>> grid = {
        {2, 5, 4},
        {1, 5, 1}
    };

    EXPECT_EQ(solution.gridGame(grid), 4);
}

TEST_F(TestGridGame, SimpleTestCase2) {
    vector<vector<int>> grid = {
        {3, 3, 1},
        {8, 5, 2}
    };

    EXPECT_EQ(solution.gridGame(grid), 4);
}

TEST_F(TestGridGame, SimpleTestCase3) {
    vector<vector<int>> grid = {
        {1, 3, 1, 15},
        {1, 3, 3, 1}
    };

    EXPECT_EQ(solution.gridGame(grid), 7);
}

TEST_F(TestGridGame, SimpleTestCase4) {
    vector<vector<int>> grid = {
        {20, 3, 20, 17, 2, 12, 15, 17, 4, 15},
        {20, 10, 13, 14, 15, 5, 2, 3, 14, 3}
    };

    EXPECT_EQ(solution.gridGame(grid), 63);
}

TEST_F(TestGridGame, SimpleTestCase5) {
    vector<vector<int>> grid = {
        {100, 50, 50, 50, 10, 1},
        {1, 1, 1, 1, 1, 1}
    };

    EXPECT_EQ(solution.gridGame(grid), 4);
}