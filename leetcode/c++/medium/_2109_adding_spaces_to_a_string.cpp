// Leetcode Problem 2109

#include <gtest/gtest.h>
#include <string>

using namespace std;

class Solution {
public:
    string addSpaces(string s, vector<int>& spaces) {
        string solution;
        int next_space_index = 0;

        for (int i = 0; i < s.size(); i++) {
            if ((next_space_index < spaces.size()) && i == spaces[next_space_index]) {
                solution += ' ';
                next_space_index++;
            }
            solution += s[i];
        }

        return solution;
    }
};


class TestClass : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestClass, SimpleInsertionCase) {
    string s = "LeetcodeHelpsMeLearn";
    vector<int> spaces = {8, 13, 15};
    string sol = "Leetcode Helps Me Learn";

    EXPECT_EQ(solution.addSpaces(s, spaces), sol);
}

TEST_F(TestClass, SimpleInsertionCase2) {
    string s = "icodeinpython";
    vector<int> spaces = {1, 5, 7, 9};
    string sol = "i code in py thon";

    EXPECT_EQ(solution.addSpaces(s, spaces), sol);
}

TEST_F(TestClass, InsertAtEveryCharacter) {
    string s = "spacing";
    vector<int> spaces = {0, 1, 2, 3, 4, 5, 6};
    string sol = " s p a c i n g";

    EXPECT_EQ(solution.addSpaces(s, spaces), sol);
}