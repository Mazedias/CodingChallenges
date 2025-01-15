// Leetcode Problem 2429

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    int minimizeXor(int num1, int num2) {
        int x = 0;
        int bits = 0;

        while (num2 != 0) {
            bits += num2 & 1;
            num2 >>= 1;
        }

        for (int i = 32; i > 0; i--) {
            if ((num1 & (1 << (i-1))) != 0 && bits > 0) {
                x |= 1 << (i-1);
                bits--;
            }
        }

        for (int i = 0; i < 32; i++) {
            if (bits == 0) {
                break;
            }

            if ((x & (1 << i)) == 0) {
                x |= 1 << i;
                bits--;
            }
        }

        return x;
    }
};


class TestMinimizeXOR : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestMinimizeXOR, SimpleTestCase1) {
    EXPECT_EQ(solution.minimizeXor(3, 5), 3);
}

TEST_F(TestMinimizeXOR, SimpleTestCase2) {
    EXPECT_EQ(solution.minimizeXor(1, 12), 3);
}

TEST_F(TestMinimizeXOR, SimpleTestCase3) {
    EXPECT_EQ(solution.minimizeXor(15, 1), 8);
}

TEST_F(TestMinimizeXOR, SimpleTestCase4) {
    EXPECT_EQ(solution.minimizeXor(15, 15), 15);
}