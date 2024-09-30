using LeetCode.Easy.Solutions;

namespace LeetCode.Easy.Tests.Solutions;

public class TwoSumSolutionTests
{
    [Theory]
    [InlineData(new [] { 2,7,11,15 }, 9, 0, 1)]
    [InlineData(new [] { 3,2,4 }, 6, 1, 2)]
    [InlineData(new [] { 3,3 }, 6, 0, 1)]
    public void Test(int[] nums, int target, int firstIndex, int secondIndex)
    {
        var indicies = TwoSumSolution.TwoSum(nums, target);

        int[] expected = [firstIndex, secondIndex];
        indicies.Should().BeEquivalentTo(expected);
    }
}