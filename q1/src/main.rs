use std::io::stdin;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res:Vec<i32>=vec![0,0];
    for i in 0..nums.len(){
        for j in i+1..nums.len(){
            if nums[i]+nums[j] == target{
                res[0]=i as i32;
                res[1]=j as i32;
                break
            }
        }
    }
    return res;
}

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    stdin().read_line(&mut s1).expect("unable to read line");
    let nums: Vec<i32> = s1
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    stdin().read_line(&mut s2).expect("unable to read line");
    let target: i32 = s2.trim().parse().expect("can not parse to int");
    let res = two_sum(nums, target);
    println!("{}\n{}",res[0],res[1]);
}
