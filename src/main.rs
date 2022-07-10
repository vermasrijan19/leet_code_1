fn main() {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for x in 1..nums.len(){
            nums[x] += nums[x-1];
        }
    return nums;
    }
}
