// Leetcode Problem 3152

using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace medium;

public class Solution_SpecialArray2 {
    public bool[] IsArraySpecial(int[] nums, int[][] queries) {
        Dictionary<int, int> positions = [];

        int section = 0;
        bool wasLasEven = nums[0] % 2 == 0;

        positions.Add(0, 0);

        for (int i = 1; i < nums.Length; i++) {
            if ((wasLasEven && nums[i] % 2 != 0) || (!wasLasEven && nums[i] % 2 == 0)) {
                positions.Add(i, section);
            } else {
                section++;
                positions.Add(i, section);
            }
            wasLasEven = nums[i] % 2 == 0;
        }

        bool[] solution = new bool[queries.Length];
        for (int i = 0; i < queries.Length; i++) {
            if (positions[queries[i][0]] != positions[queries[i][^1]]) {
                solution[i] = false;
            } else {
                solution[i] = true;
            }
        }

        return solution;
    }

    [TestClass]
    public class TestSpecialArray2 {
        private Solution_SpecialArray2 solution;

        [TestInitialize]
        public void Setup() {
            solution = new Solution_SpecialArray2();
        }

        [TestMethod]
        public void TestSpecialArray2_SimpleTestCase1() {
            int[] nums = [3, 4, 1, 2, 6];
            int[][] queries = [[0, 4]];
            bool[] expected = [false];

            CollectionAssert.AreEqual(expected, solution.IsArraySpecial(nums, queries));
        }

        [TestMethod]
        public void TestSpecialArray2_SimpleTestCase2() {
            int[] nums = [4, 3, 1, 6];
            int[][] queries = [[0, 2], [2, 3]];
            bool[] expected = [false, true];

            CollectionAssert.AreEqual(expected, solution.IsArraySpecial(nums, queries));
        }
    }
}