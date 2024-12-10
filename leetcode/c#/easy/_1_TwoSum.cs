// Leetcode Problem 1

using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace easy;

public class Solution_TwoSum {
    public int[] TwoSum(int[] nums, int target) {
        Dictionary<int, int> map = [];

        for (int i = 0; i < nums.Length; i++) {
            int complement = target - nums[i];

            if (map.ContainsKey(complement)) {
                return [map[complement], i];
            }
            map[nums[i]] = i;
        }

        return [];
    }

    [TestClass]
    public class TestTwoSum {
        private Solution_TwoSum solution;

        [TestInitialize]
        public void Setup() {
            solution = new Solution_TwoSum();
        }

        [TestMethod]
        public void TestTwoSum_SimpleTest() {
            int[] nums = [2, 7, 11, 15];
            int[] expected = [0, 1];

            CollectionAssert.AreEqual(expected, solution.TwoSum(nums, 9));
        }

        [TestMethod]
        public void TestTwoSum_SimpleTest2() {
            int[] nums = [3, 2, 4];
            int[] expected = [1, 2];

            CollectionAssert.AreEqual(expected, solution.TwoSum(nums, 6));
        }

        [TestMethod]
        public void TestTwoSum_SimplgeTest3() {
            int[] nums = [3, 3];
            int[] expected = [0, 1];

            CollectionAssert.AreEqual(expected, solution.TwoSum(nums, 6));
        }
    }
}
