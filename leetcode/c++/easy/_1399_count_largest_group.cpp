// Leetcode Problem 1399

#include <gtest/gtest.h>
#include <unordered_map>

using namespace std;

class Solution {
public:
    int countLargestGroup(int n) {
        unordered_map<int, int> groups;

        int sum;
        for (int i = 1; i <= n; i++) {
            sum = 0;
            int temp = i;
            while (temp > 0) {
                sum += temp % 10;
                temp /= 10;
            }
            groups[sum]++;
        }

        int max_occurances = 0;
        for (auto group = groups.begin(); group != groups.end(); ++group) {
            if (group->second > max_occurances) {
                max_occurances = group->second;
            }
        }
        int count = 0;
        for (auto group = groups.begin(); group != groups.end(); ++group) {
            if (group->second == max_occurances) {
                count++;
            }
        }

        return count;
    }
};


class TestCountLargestGroup : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestCountLargestGroup, SimpleTestCase1) {
    EXPECT_EQ(solution.countLargestGroup(13), 4);
    EXPECT_EQ(solution.countLargestGroup(2), 2);
    EXPECT_EQ(solution.countLargestGroup(99), 1);
}