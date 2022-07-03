use std::collections::VecDeque;

pub fn array_plus_one(nums: &mut VecDeque<usize>) {
    let arr_len = nums.len();
    for i in 0..arr_len  {
        let index = arr_len - 1 - i;
        if nums[index] + 1 <= 9 {
            nums[index] += 1;
            break;
        } else {
            nums[index] = 0;
            if i == arr_len - 1 {
                nums.push_front(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::array_plus_one;

    #[test]
    fn check_simple() {
        let mut nums: VecDeque<usize> = VecDeque::new();
        nums.push_back(6);
        nums.push_back(8);
        nums.push_back(3);

        array_plus_one(&mut nums);

        assert_eq!(nums, vec![6, 8, 4]);
    }

    #[test]
    fn check_simple_neq() {
        let mut nums: VecDeque<usize> = VecDeque::new();
        nums.push_back(6);
        nums.push_back(8);
        nums.push_back(3);
        
        array_plus_one(&mut nums);

        assert_ne!(nums, vec![6, 8, 3]);
    }

    #[test]
    fn check_nine() {
        let mut nums: VecDeque<usize> = VecDeque::new();
        nums.push_back(3);
        nums.push_back(1);
        nums.push_back(2);
        nums.push_back(3);
        nums.push_back(9);
        nums.push_back(9);
        
        array_plus_one(&mut nums);

        assert_eq!(nums, vec![3, 1, 2, 4, 0, 0]);
    }

    #[test]
    fn check_length_increase() {
        let mut nums: VecDeque<usize> = VecDeque::new();
        nums.push_back(9);
        nums.push_back(9);
        nums.push_back(9);
        nums.push_back(9);
        
        array_plus_one(&mut nums);

        assert_eq!(nums, vec![1, 0, 0, 0, 0]);
    }
}
