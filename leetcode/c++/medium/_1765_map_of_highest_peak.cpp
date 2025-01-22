// Leetcode Problem 1765

#include <gtest/gtest.h>
#include <queue>
#include <utility>

using namespace std;

class Solution {
public:
    vector<vector<int>> highestPeak(vector<vector<int>> isWater) {
        int m = isWater.size();
        int n = isWater[0].size();

        vector<vector<int>> hights(m, vector<int>(n, -1));
        queue<pair<int, int>> q;

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (isWater[i][j] == 1) {
                    hights[i][j] = 0;
                    q.push({i, j});
                }
            }
        }

        while (!q.empty()) {
            pair<int, int> pos = q.front();
            q.pop();

            vector<pair<int, int>> directions {
                {pos.first - 1, pos.second},
                {pos.first + 1, pos.second},
                {pos.first, pos.second - 1},
                {pos.first, pos.second + 1},
            };

            for (const auto& [x, y] : directions) {
                if ((x >= 0) && (x < m) && (y >= 0) && (y < n) && (hights[x][y] == -1)) {
                    hights[x][y] = hights[pos.first][pos.second] + 1;
                    q.push({x, y});
                }
            }

        }

        return hights;
    }
};


class TestHighestPeak : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestHighestPeak, SimpleTestCase1) {
    vector<vector<int>> isWater = {
        {0, 1},
        {0, 0}
    };

    vector<vector<int>> expextedHights = {
        {1, 0},
        {2, 1}
    };

    EXPECT_EQ(solution.highestPeak(isWater), expextedHights);
}

TEST_F(TestHighestPeak, SimpleTestCase2) {
    vector<vector<int>> isWater = {
        {0, 0, 1},
        {1, 0, 0},
        {0, 0 ,0}
    };

    vector<vector<int>> expectedHights = {
        {1, 1, 0},
        {0, 1, 1},
        {1, 2, 2}
    };

    EXPECT_EQ(solution.highestPeak(isWater), expectedHights);
}

TEST_F(TestHighestPeak, SimpleTestCase3) {
    vector<vector<int>> isWater = {
        {0, 0, 0, 0, 0},
        {1, 0, 0, 0, 1},
        {0, 0, 0, 0, 0},
        {0, 0, 0, 0, 0},
        {0, 1, 1, 0, 0},
        {0, 0, 0, 0, 1}
    };

    vector<vector<int>> expectedHights = {
        {1, 2, 3, 2, 1},
        {0, 1, 2, 1, 0},
        {1, 2, 2, 2, 1},
        {2, 1, 1, 2, 2},
        {1, 0, 0, 1, 1},
        {2, 1, 1, 1, 0}
    };

    EXPECT_EQ(solution.highestPeak(isWater), expectedHights);
}