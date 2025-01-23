// Leetcode Problem 1267

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    int countServers(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();

        vector<int> rows(m, 0);
        vector<int> columns(n, 0);

        int sum = 0;

        for (int j = 0; j < m; j++) {
            for (int i = 0; i < n; i++) {
                if (grid[j][i] == 1) {
                    rows[j]++;
                    columns[i]++;
                    sum++;
                }
            }
        }

        for (int j = 0; j < m; j++) {
            for (int i = 0; i < n; i++) {
                if (grid[j][i] == 1 && rows[j] == 1 && columns[i] == 1) {
                    sum--;
                }
            }
        }

        return sum;
    }
};


class TestCountServers : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestCountServers, SimpleTestCase1) {
    vector<vector<int>> grid = {
        {1, 0},
        {0, 1}
    };

    EXPECT_EQ(solution.countServers(grid), 0);
}

TEST_F(TestCountServers, SimpleTestCase2) {
    vector<vector<int>> grid = {
        {1, 0},
        {1, 1}
    };

    EXPECT_EQ(solution.countServers(grid), 3);
}

TEST_F(TestCountServers, SimpleTestCase3) {
    vector<vector<int>> grid = {
        {1, 1, 0, 0},
        {0, 0, 1, 0},
        {0, 0, 1, 0},
        {0, 0, 0, 1}
    };

    EXPECT_EQ(solution.countServers(grid), 4);
}