// Leetcode Problem 23

#include <gtest/gtest.h>
#include <vector>
#include <queue>
#include "LinkedList.h"
#include "BinaryMinHeap.h"

using namespace std;

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        auto comp = [](ListNode* a, ListNode* b) {
            return a->val > b->val;
        };

        priority_queue<ListNode*, vector<ListNode*>, decltype(comp)> minHeap(comp);
    
        for (auto head : lists) {
            if (head) {
                minHeap.push(head);
            }
        }

        ListNode* head = new ListNode();
        ListNode* tempHead = head;

        while (!minHeap.empty()) {
            ListNode* temp = minHeap.top();
            minHeap.pop();

            head->next = temp;
            head = head->next;

            if (temp && temp->next) {
                minHeap.push(temp->next);
            }
        }

        return tempHead->next;
    }   

    /**
     * Solution function using own min heap implementation
     */
    ListNode* mergeKLists_customHeap(vector<ListNode*>& lists) {
        if (lists.empty()) {
            return nullptr;
        }

        MinHeap<int> heap;

        for (int i = 0; i < lists.size(); i++) {
            while (lists[i] != nullptr) {
                heap.insert(lists[i]->val);
                lists[i] = lists[i]->next;
            }
        }

        if (heap.size() == 0) {
            return nullptr;
        }

        ListNode* head = new ListNode(heap.extractMin());
        ListNode* temp = head;

        while (!heap.isEmpty()) {
            head->next = new ListNode(heap.extractMin());
            head = head->next;
        }
        
        return temp;
    }
};


class TestMergeKLists : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestMergeKLists, SimpleMergeCase) {
    vector<ListNode*> lists = {
        createLinkedList({1, 4, 5}),
        createLinkedList({1, 3, 4}),
        createLinkedList({2, 6})
    };

    ListNode* sol = createLinkedList({1, 1, 2, 3, 4, 4, 5, 6});

    EXPECT_TRUE(compareLists(sol, solution.mergeKLists(lists)));
}

TEST_F(TestMergeKLists, EmptyLists) {
    vector<ListNode*> lists = {};

    EXPECT_TRUE(compareLists(nullptr, solution.mergeKLists(lists)));
}

TEST_F(TestMergeKLists, EmptyList) {
    vector<ListNode*> lists = {{}};

    EXPECT_TRUE(compareLists(nullptr, solution.mergeKLists(lists)));   
}