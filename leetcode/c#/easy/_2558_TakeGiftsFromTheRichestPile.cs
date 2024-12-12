// Leetcode Problem 2558

using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace easy;

public class Solution_TakeGiftsFromTheRichestPile {
    // This is not my solution but a more efficient approach
    public long PickGifts(int[] gifts, int k) {
        for (int i = 0; i < k; i++) {
            int max = 0;
            int index = -1;
            for (int j = 0; j < gifts.Length; j++) {
                if (gifts[j] > max) {
                    max = gifts[j];
                    index = j;
                }
            }

            gifts[index] = (int) Math.Sqrt(gifts[index]);
        }

        return gifts.Aggregate(0L, (a,b) => (long)a + (long)b);
    }

    // This is my solution but it is not very efficient
    public long PickGiftsInefficient(int[] gifts, int k) {
        PriorityQueue<int, int> pq = new PriorityQueue<int, int>(
            gifts.Select(x => (x, -x))
        );

        while (k > 0) {
            int gift = pq.Dequeue();

            // find root with binary search
            int left = 1;
            int right = Math.Min(gift, 46340);
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

            pq.Enqueue(ans, -ans);
            k--;
        }

        long sum = 0;
        while (pq.Count > 0) {
            sum += pq.Dequeue();
        }

        return sum;
    }

    [TestClass]
    public class TestTakeGiftsFromTheRichtestPile {
        private Solution_TakeGiftsFromTheRichestPile solution;

        [TestInitialize]
        public void Setup() {
            solution = new Solution_TakeGiftsFromTheRichestPile();
        }

        [TestMethod]
        public void SimpleTestCase1() {
            int[] gifts = [25, 64, 9, 4, 100];

            Assert.AreEqual(29, solution.PickGifts(gifts, 4));
        }

        [TestMethod]
        public void SimpleTestCase2() {
            int[] gifts = [1, 1, 1, 1];

            Assert.AreEqual(4, solution.PickGifts(gifts, 4));
        }
    }
}