// Leetcode Problem 2825

#include <gtest/gtest.h>
#include <string>

using namespace std;

class Solution {
public:
    bool canMakeSubsequence(string str1, string str2) {
        int t_len = str2.size();
        char target_char = str2[0];

        int target_index = 0;

        for (int i = 0; i < str1.size(); i++) {
            char source_char = str1[i];

            if ((source_char == target_char) || (source_char + 1 == target_char) || (source_char == 'z' && target_char == 'a')) {
                target_index++;
                if (target_index == t_len) {
                    break;
                }
                target_char = str2[target_index];
            }
        }

        return target_index == t_len;
    }
};


class TestCanMakeSubsequence : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestCanMakeSubsequence, SimpleTrueCase) {
    string str1 = "abc";
    string str2 = "ad";

    EXPECT_TRUE(solution.canMakeSubsequence(str1, str2));
}

TEST_F(TestCanMakeSubsequence, SimpleTrueCase2) {
    string str1 = "zc";
    string str2 = "ad";

    EXPECT_TRUE(solution.canMakeSubsequence(str1, str2));
}

TEST_F(TestCanMakeSubsequence, SimpleFalseCase) {
    string str1 = "ab";
    string str2 = "d";

    EXPECT_FALSE(solution.canMakeSubsequence(str1, str2));
}