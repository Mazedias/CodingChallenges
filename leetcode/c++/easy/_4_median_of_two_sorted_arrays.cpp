// Leetcode Problem 4

#include <gtest/gtest.h>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>&nums2) {
        vector<int> result;
        result.reserve(nums1.size() + nums2.size());

        merge(nums1.begin(), nums1.end(),
              nums2.begin(), nums2.end(),
              back_inserter(result));

        if (result.size() % 2 == 0) {
            return (result[result.size() / 2] + result[result.size() / 2 - 1]) / 2.0;
        } else {
            return (double)(result[result.size() / 2]);
        }
    }
};


class TestFindMedian : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestFindMedian, TestCase1) {
    vector<int> nums1 = {1, 3};
    vector<int> nums2 = {2};

    EXPECT_EQ(solution.findMedianSortedArrays(nums1, nums2), 2.0);
}

TEST_F(TestFindMedian, TestCase2) {
    vector<int> nums1 = {1, 2};
    vector<int> nums2 = {3, 4};

    EXPECT_EQ(solution.findMedianSortedArrays(nums1, nums2), 2.5);
}