#ifndef LEETCODE_LINKED_LIST_H
#define LEETCODE_LINKED_LIST_H

#include <vector>
#include <iostream>

using namespace std;

struct ListNode
{
    int val;
    ListNode *next;

    // Constructors
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

/**
 * Creates a linked list from a vector and returns a pointer to the
 * head of the list.
 * 
 * If the vector is empty a nullptr is returned. 
 */
ListNode* createLinkedList(const vector<int>& nums) {
    if (nums.empty()) return nullptr;

    ListNode* head = new ListNode(nums[0]);
    ListNode* current = head;

    for (size_t i = 1; i < nums.size(); i++) {
        current->next = new ListNode(nums[i]);
        current = current->next;
    }

    return head;
}

/**
 * Utility function to print a linked list
 */
void printLinkedList(ListNode* head) {
    while (head != nullptr) {
        cout << head->val;
        if (head->next != nullptr) cout << " --> ";
        head = head->next;
    }
    cout << endl;
}

/**
 * Utility function to delete a linked list and free the memory
 */
void deleteLinkedList(ListNode* head) {
    while (head != nullptr) {
        ListNode* temp = head;
        head = head->next;
        delete temp;
    }
}

/**
 * Utility function to compare two lists
 */
bool compareLists(ListNode* list1, ListNode* list2) {
    while (list1 != nullptr && list2 != nullptr) {
        if (list1->val != list2->val) {
            return false;
        }
        list1 = list1->next;
        list2 = list2->next;
    }

    if (list1 != nullptr || list2 != nullptr) {
        return false;
    }

    return true;
}

#endif