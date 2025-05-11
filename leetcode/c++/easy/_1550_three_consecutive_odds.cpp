// Leetcode Problem 1550

#include <gtest/gtest.h>

using namespace std;

class Solution {
public:
    bool threeConsecutiveOdds(vector<int>& arr) {
        int count_odd = 0;

        for (int n : arr) {
            if (n % 2 == 1) {
                count_odd++;
                if (count_odd == 3) {
                    return true;
                }
            } else {
                count_odd = 0;
            }
        }
        return false;
    }
};


class TestThreeConsecutiveOdds : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestThreeConsecutiveOdds, SimpleTestCase1) {
    vector<int> arr = {2, 6, 4, 1};

    EXPECT_FALSE(solution.threeConsecutiveOdds(arr));
}

TEST_F(TestThreeConsecutiveOdds, SimpleTestCase2) {
    vector<int> arr = {1, 2, 34, 3, 4, 5, 7, 23, 12};

    EXPECT_TRUE(solution.threeConsecutiveOdds(arr));
}