// Leetcode Problem 3264

#include <gtest/gtest.h>
#include <vector>
#include <queue>

using namespace std;

class Solution {
public:
    vector<int> getFinalState(vector<int>& nums, int k, int multiplier) {
        // Custom min-heap comparator for the priority queue
        auto comp = [](const tuple<int, int>& a, const tuple<int, int>& b) {
            
            // Check if a == b because if so we want to put prioritze the one with the lower index (the one that occurs first)
            if (get<1>(a) == get<1>(b)) {
                return get<0>(a) > get<0>(b);
            }
            
            return get<1>(a) > get<1>(b);
        };

        priority_queue<tuple<int, int>, vector<tuple<int, int>>, decltype(comp)> minHeap(comp);

        for (int i = 0; i < nums.size(); i++) {
            minHeap.push(make_tuple(i, nums[i]));
        }

        for (int i = 0; i < k; i++) {
            tuple<int, int> element = minHeap.top();
            minHeap.pop();

            int index = get<0>(element);
            nums[index] *= multiplier;

            minHeap.push(make_tuple(index, nums[index]));
        }

        return nums;
    }
};


class TestGetFinalState : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestGetFinalState, SimpleTestCase1) {
    vector<int> nums = {2, 1, 3, 5, 6};
    vector<int> sol = {8, 4, 6, 5, 6};

    EXPECT_EQ(solution.getFinalState(nums, 5, 2), sol);
}

TEST_F(TestGetFinalState, SimpleTestCase2) {
    vector<int> nums = {1, 2};
    vector<int> sol = {16, 8};

    EXPECT_EQ(solution.getFinalState(nums, 3, 4), sol);
}

TEST_F(TestGetFinalState, SimpleTestCase3) {
    vector<int> nums = {1, 3, 5};
    vector<int> sol = {27, 9, 15};

    EXPECT_EQ(solution.getFinalState(nums, 5, 3), sol);
}