// Leetcode Problem 2657

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
        int n = A.size();
        vector<int> frequency(n + 1, 0);
        vector<int> prefixCommonArray(n);

        int common = 0;
        for (int i = 0; i < n; i++) {
            if (++frequency[A[i]] == 2) common++;
            if (++frequency[B[i]] == 2) common++;
            prefixCommonArray[i] = common;
        }

        return prefixCommonArray;
    }
};


class TestFindPrefixCommonArray : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestFindPrefixCommonArray, SimpleTestCase1) {
    vector<int> A = {1, 3, 2, 4};
    vector<int> B = {3, 1, 2, 4};
    vector<int> expected = {0, 2, 3, 4};

    EXPECT_EQ(solution.findThePrefixCommonArray(A, B), expected);
}

TEST_F(TestFindPrefixCommonArray, SimpleTestCase2) {
    vector<int> A = {2, 3, 1};
    vector<int> B = {3, 1, 2};
    vector<int> expected = {0, 1, 3};

    EXPECT_EQ(solution.findThePrefixCommonArray(A, B), expected);
}