pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max=0;

    for x in accounts{
        let mut sum=0;
        for y in x{
            sum+=y;
        }
        if sum>max{
            max=sum;
        }
    }
    return max;
}