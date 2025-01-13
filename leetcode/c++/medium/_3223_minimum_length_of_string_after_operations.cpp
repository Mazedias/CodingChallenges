// Leetcode Problem 3223

#include <gtest/gtest.h>
#include <unordered_map>

using namespace std;

class Solution {
public:
    // Better solution
    int minimumLength(string s) {
        int counter[26]{};

        for (char& c : s) {
            ++counter[c - 'a'];
        }

        int count = 0;
        for (int x : counter) {
            if (x) {
                count += x % 2 ? 1 : 2;
            }
        }

        return count;
    }

    // My solution
    int minimumLength_my(string s) {
        unordered_map<char, int> occurances;

        for (char c : s) {
            if (occurances.contains(c)) {
                occurances[c]++;
            } else {
                occurances.insert({c, 1});
            }
        }

        int count = 0;
        for (const auto& pair : occurances) {
            if (pair.second % 2 != 0) {
                count++;
            } else {
                count += 2;
            }
        }

        return count;
    }
};


class TestMinimumLength : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestMinimumLength, SimpleTestCase) {
    string s = "abaacbcbb";

    EXPECT_EQ(solution.minimumLength(s), 5);
}

TEST_F(TestMinimumLength, SimpleTestCase2) {
    string s = "aa";

    EXPECT_EQ(solution.minimumLength(s), 2);
}