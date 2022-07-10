pub fn number_of_steps(num : i32) -> i32 {
    let mut count = 0;
    num += 1;
    loop {
        if num == 0 {
            break;
        }
        if num %2==0{
            num/=2;
        }
        else{
            num-=1;
        }
        count+=1;
    }
    return count;
}