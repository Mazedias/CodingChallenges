// Leetcode Problem 2558

#include <gtest/gtest.h>
#include <vector>
#include <queue>

using namespace std;

class Solution {
public:
    long long pickGifts(vector<int>& gifts, int k) {
        priority_queue<int> pq(gifts.begin(), gifts.end());

        while (k > 0) {
            int gift = pq.top();
            pq.pop();

            // Calculate root using binary search
            int left = 1;
            int right = min(gift, 46340);
            int ans = 0;

            while (left <= right) {
                int mid = (right + left) / 2;

                if (mid * mid == gift) {
                    ans = mid;
                    break;
                }

                if (mid * mid <= gift) {
                    left = mid + 1;
                    ans = mid;
                } else {
                    right = mid - 1;
                }
            }

            gift = ans;
            pq.push(gift);
            k--;
        }

        long long sum = 0;
        while (!pq.empty()) {
            sum += pq.top();
            pq.pop();
        }

        return sum;
    }
};


class TestPickGifts : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestPickGifts, TestPickGifts_SimpleCase1) {
    vector<int> gifts = {25, 64, 9, 4, 100};

    EXPECT_EQ(solution.pickGifts(gifts, 4), 29);
}

TEST_F(TestPickGifts, TestPickGifts_SimpleCase2) {
    vector<int> gifts = {1, 1, 1, 1};

    EXPECT_EQ(solution.pickGifts(gifts, 4), 4);
}