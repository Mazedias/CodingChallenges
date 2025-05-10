// Leetcode Problem 2918

#include <gtest/gtest.h>
#include <algorithm>

using namespace std;

class Solution {
public:
    long long minSum(vector<int>& nums1, vector<int>& nums2) {
        long long sum1 = 0;
        long long sum2 = 0;
        int zeros1 = 0;
        int zeros2 = 0;

        for (int num : nums1) {
            sum1 += (long long)num;
            if (num == 0) {
                zeros1++;
                sum1++;
            }
        }

        for (int num : nums2) {
            sum2 += (long long)num;
            if (num == 0) {
                zeros2++;
                sum2++;
            }
        }

        if ((zeros1 == 0 && sum1 < sum2) || (zeros2 == 0 && sum2 < sum1)) {
            return -1;
        } else {
            return max(sum1, sum2);
        }
    }
};