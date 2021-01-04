

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 1 {
        return vec![nums];
    }
    let pers = permute(nums[1..].to_vec());
    let mut vec = Vec::new();
    for per in pers {
        for i in 0..per.len()+1 {
            let mut v = Vec::new();
            v.extend(&per[..i]);
            v.push(nums[0]);
            v.extend(&per[i..]);
            vec.push(v);
        }
    }
    return vec;
}