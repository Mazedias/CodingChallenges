// Leetcode Problem 2661

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    int firstCompleteIndex(vector<int>& arr, vector<vector<int>>& mat) {
        int m = mat.size();
        int n = mat[0].size();

        vector<int> frequency_column(m, 0);
        vector<int> frequency_row(n, 0);
        vector<tuple<int, int>> positions(m*n, make_tuple(0, 0));

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                positions[mat[i][j] - 1] = make_tuple(i, j);
            }
        }

        for (int pos = 0; pos < arr.size(); pos++) {
            tuple<int, int> position = positions[arr[pos] - 1];
            
            frequency_column[get<0>(position)]++;
            frequency_row[get<1>(position)]++;

            if (frequency_column[get<0>(position)] == n || frequency_row[get<1>(position)] == m) {
                return pos;
            }
        }

        return arr.size() - 1;
    }
};


class TestFirstCompleteIndex : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestFirstCompleteIndex, SimpleTestCase1) {
    vector<int> arr = {1, 3, 4, 2};
    vector<vector<int>> mat = {
        {1, 4},
        {2, 3}
    };

    EXPECT_EQ(solution.firstCompleteIndex(arr, mat), 2);
}

TEST_F(TestFirstCompleteIndex, SimpleTestCase2) {
    vector<int> arr = {2, 8, 7, 4, 1, 3, 5, 6, 9};
    vector<vector<int>> mat = {
        {3, 2, 5},
        {1, 4, 6},
        {8, 7, 9}
    };

    EXPECT_EQ(solution.firstCompleteIndex(arr, mat), 3);
}

TEST_F(TestFirstCompleteIndex, SimpleTestCase3) {
    vector<int> arr = {1, 4, 5, 2, 6, 3};
    vector<vector<int>> mat = {
        {4, 3, 5},
        {1, 2, 6}
    };

    EXPECT_EQ(solution.firstCompleteIndex(arr, mat), 1);
}

TEST_F(TestFirstCompleteIndex, SimpleTestCase4) {
    vector<int> arr = {6, 2, 3, 1, 4, 5};
    vector<vector<int>> mat = {
        {5, 1},
        {2, 4},
        {6, 3}
    };

    EXPECT_EQ(solution.firstCompleteIndex(arr, mat), 2);
}