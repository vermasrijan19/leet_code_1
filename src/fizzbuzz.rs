pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for x in 1..n
    {
        if x % 3 == 0 && x % 5 == 0
        {
            ans.push("FizzBuzz".to_string());
        }
        else if x % 3 == 0
        {
            ans.push("Fizz".to_string());
        }
        else if x % 5 == 0
        {
            ans.push("Buzz".to_string());
        }
        else
        {
            ans.push(x.to_string());
        }
    }
    return ans;
}