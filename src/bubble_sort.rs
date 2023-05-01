pub fn bubble_sort(nums: &mut Vec<i32>) {
    let mut last_i = nums.len() -1;
    while last_i >0 {
        for i in 0..last_i {
            if nums[i] > nums[i+1] {
                (nums[i+1], nums[i]) = (nums[i], nums[i+1])
            }
        }
        last_i -=1;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    pub fn sort() {
        let mut v = vec![3,1,2,7,5,4,9,6,8];
        bubble_sort(&mut v);

        assert_eq!(vec![1,2,3,4,5,6,7,8,9], v);
    }
}