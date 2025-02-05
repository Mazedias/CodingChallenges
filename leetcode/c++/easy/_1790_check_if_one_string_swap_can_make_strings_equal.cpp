// Leetcode Problem 1790

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    bool areAlmostEqual(string s1, string s2) {
        if (s1 == s2) {
            return true;
        }

        int pos1 = -1;
        int pos2 = -1;

        for (int i = 0; i < s1.size(); i++) {
            if (s1[i] != s2[i]) {
                if (pos1 == -1) {
                    pos1 = i;
                } else if (pos2 == -1) {
                    pos2 = i;
                } else {
                    return false;
                }
            }
        }

        if (pos1 != -1 && pos2 == -1) {
            return false;
        }

        return ((s1[pos1] == s2[pos2]) && (s1[pos2] == s2[pos1]));
    }
};


class TestAreAlmostEqual : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestAreAlmostEqual, SimpleTestCase1) {
    string s1 = "bank";
    string s2 = "kanb";

    EXPECT_TRUE(solution.areAlmostEqual(s1, s2));
}

TEST_F(TestAreAlmostEqual, SimpleTestCase2) {
    string s1 = "attack";
    string s2 = "defense";

    EXPECT_FALSE(solution.areAlmostEqual(s1, s2));
}

TEST_F(TestAreAlmostEqual, SimpleTestCase3) {
    string s1 = "kelb";
    string s2 = "kelb";

    EXPECT_TRUE(solution.areAlmostEqual(s1, s2));
}

TEST_F(TestAreAlmostEqual, SimpleTestCase4) {
    string s1 = "ahusgbejsh";
    string s2 = "ahusgbjesh";

    EXPECT_TRUE(solution.areAlmostEqual(s1, s2));
}

TEST_F(TestAreAlmostEqual, SimpleTestCase5) {
    string s1 = "a";
    string s2 = "b";

    EXPECT_FALSE(solution.areAlmostEqual(s1, s2));
}