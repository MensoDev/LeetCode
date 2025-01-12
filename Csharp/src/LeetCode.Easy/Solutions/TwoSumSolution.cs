namespace LeetCode.Easy.Solutions;

public static class TwoSumSolution
{
    public static int[] TwoSum(int[] nums, int target)
    {
        Dictionary<int, int> indexMap = [];
        
        for (var index = 0; index < nums.Length; index++)
        {
            var current = target - nums[index];

            if (indexMap.TryGetValue(current, out var value))
            {
                return [value, index];
            }
            
            indexMap[nums[index]] = index;
        }

        return [];
    }
}