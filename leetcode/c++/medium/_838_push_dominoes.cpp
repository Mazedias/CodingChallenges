// Leetcode Problem 838

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    string pushDominoes(string dominoes) {
        int n = dominoes.size();

        int position = 0;
        int pointer;
        char current_direction = '.';
        char next_direction;
        while (position < n) {
            pointer = position;

            while (pointer < n && dominoes[pointer] != 'L' && dominoes[pointer] != 'R') {
                pointer++;
            }

            if (pointer >= n) {
                break;
            }

            next_direction = dominoes[pointer];
            if ((current_direction == '.' && (next_direction == 'R' || next_direction == '.')) || (current_direction == 'L' && next_direction == 'R')) {
                current_direction = 'R';
            } else if (current_direction == 'R' && next_direction == 'R') {
                for (int i = position; i < pointer; i++) {
                    dominoes[i] = 'R';
                }
            } else if ((current_direction == '.' || current_direction == 'L') && next_direction == 'L') {
                for (int i = position; i < pointer; i++) {
                    dominoes[i] = 'L';
                }
                current_direction = '.';
            } else if (current_direction == 'R' && next_direction == 'L') {
                int start = position;
                int end = pointer-1;

                while (start < end) {
                    dominoes[start] = 'R';
                    dominoes[end] = 'L';
                    start++;
                    end--;
                }
                current_direction = '.';
            }
            position = pointer + 1;
        }

        if (current_direction == 'R') {
            for (int i = position; i < n; i++) {
                dominoes[i] = 'R';
            }
        }

        return dominoes;
    }
};


class TestPushDominoes : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestPushDominoes, SimpleTestCase1) {
    string dominoes = "RR.L";
    string expected = "RR.L";

    EXPECT_EQ(solution.pushDominoes(dominoes), expected);
}

TEST_F(TestPushDominoes, SimpleTestCase2) {
    string dominoes = ".L.R...LR..L..";
    string expected = "LL.RR.LLRRLL..";

    EXPECT_EQ(solution.pushDominoes(dominoes), expected);
}

TEST_F(TestPushDominoes, SimpleTestCase3) {
    string dominoes = "..R.R.R.LLLLL.....R...LR....L.R..";
    string expected = "..RRRRR.LLLLL.....RR.LLRRRLLL.RRR";

    EXPECT_EQ(solution.pushDominoes(dominoes), expected);
}