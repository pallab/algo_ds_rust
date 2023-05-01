pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut index = -1;

    let mut left_i = 0;
    let mut right_i  = nums.len() as i32  -1;
    while left_i <= right_i {
        let mid = (right_i+left_i)/2 ;

        println!("mid is {}", mid);

        if nums[mid as usize] > target {
            right_i = mid-1;
        } else if nums[mid as usize] < target {
            left_i = mid+1;
        } else{
            index = mid;
            break;
        }
    }
    index
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn first() {
        assert_eq!(0, search(vec![5], -5));
    }
}