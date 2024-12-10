// Leetcode Problem 4

using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace easy;

public class Solution_FindMedianSortedArrays {
    public double FindMedianSortedArrays(int[] nums1, int[] nums2) {
        int[] result = nums1.Concat(nums2).ToArray();
        Array.Sort(result);

        if (result.Length % 2 == 0) {
            return (result[result.Length / 2] + result[result.Length / 2 - 1]) / 2.0;
        } else {
            return result[result.Length / 2];
        }
    }

    [TestClass]
    public class TestFindMedianSortedArrays {
        private Solution_FindMedianSortedArrays solution;

        [TestInitialize]
        public void Setup() {
            solution = new Solution_FindMedianSortedArrays();
        }

        [TestMethod]
        public void TestFindMedianSortedArrays_SimpleTestCase1() {
            int[] nums1 = [1, 3];
            int[] nums2 = [2];
            double expected = 2.0;

            Assert.AreEqual(expected, solution.FindMedianSortedArrays(nums1, nums2));
        }

        [TestMethod]
        public void TestFindMedianSortedArrays_SimpleTestCase2() {
            int[] nums1 = [1, 2];
            int[] nums2 = [3, 4];
            double expected = 2.5;

            Assert.AreEqual(expected, solution.FindMedianSortedArrays(nums1, nums2));
        }
    }
}