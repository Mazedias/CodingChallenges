/**
 * This BinaryMinHeap implementation is from https://www.geeksforgeeks.org/cpp-program-to-implement-binary-heap/
 */

#ifndef LEETCODE_BINARY_MIN_HEAP_H
#define ELETCODE_BINARY_MIN_HEAP_H

#include <iostream>
#include <stdexcept>
#include <vector>

using namespace std;

template <typename T> class MinHeap {
private:
    vector<T> heap;

    int parent(int index) { return (index - 1) / 2; }

    int leftChild(int index) { return (2 * index + 1); }

    int rightChild(int index) { return (2 * index + 2); }

    void heapifyUp(int index) {
        while (index && heap[index] < heap[parent(index)]) {
            swap(heap[index], heap[parent(index)]);
            index = parent(index);
        }
    }

    void heapifyDown(int index) {
        int left = leftChild(index);
        int right = rightChild(index);
        int smallest = index;

        if (left < heap.size() && heap[left] < heap[smallest]) {
            smallest = left;
        }

        if (right < heap.size() && heap[right] < heap[smallest]) {
            smallest = right;
        }

        if (smallest != index) {
            swap(heap[index], heap[smallest]);
            heapifyDown(smallest);
        }
    }
public:

    /**
     * Check if heap is empty
     */
    bool isEmpty() const { return heap.empty(); }

    /**
     * Get size of the heap
     */
    int size() const { return heap.size(); }

    /**
     * Get miminum element
     * 
     * Throws a runtime error if the heap is empty
     */
    T getMin() const {
        if (isEmpty()) {
            throw runtime_error("Heap is empty");
        }
        return heap.front();
    }

    /**
     * Insert a new key
     */
    void insert(T key) {
        heap.push_back(key);
        int index = size() - 1;
        heapifyUp(index);
    }

    /**
     * Extract the minimum element.
     * Returns the minimum element and removes it from the heap.
     * 
     * Throws a runtime error if the heap is emtpy
     */
    T extractMin() {
        if (isEmpty()) {
            throw runtime_error("Heap is empty");
        }

        T root = heap.front();
        heap[0] = heap.back();
        heap.pop_back();
        heapifyDown(0);
        return root;
    }

    /**
     * Deletes a node from the heap and restores the heap property afterwards.
     * 
     * Throws a runtime error if the key is not in the heap
     */
    void deleteNode(T key) {
        int index = -1;

        // Find the index of the node that shall be deleted
        for (int i = 0; i < size(); ++i) {
            if (heap[i] == key) {
                index = i;
                break;
            }
        }

        if (index == -1) {
            throw runtime_error("Key not found in the heap");
        }

        heap[index] = heap.back();
        heap.pop_back();

        // Restore heap property
        heapifyUp(index);
        heapifyDown(index);
    }

    /**
     * Decreases the value of a key
     * 
     * Throws a invalid argument if the index is invalid (< 0, > size()) or if
     * the value is greater than the existing one
     */
    void decreaseKey(int i, T newValue) {
        if (i < 0 || i > size() || newValue > heap[i]) {
            throw invalid_argument("Invalid index or newValue");
        }

        heap[i] = newValue;
        heapifyUp(i);
    }

    /**
     * Print elements in the heap
     */
    void printHeap() const {
        for (const T& elemenet : heap) {
            cout << elemenet << " ";
        }
        cout << endl;
    }
};

#endif