# Array Plus One

Given a non-negative number represented as an array of digits, add 1 to the number (increment the number represented by the digits). The digits are stored such that the most significant digit is the first element of the array.

**Approach**: To add one to the number represented by digits, follow the below steps : 

- Parse the given array from the end as we do in school addition
- If the last elements are 9, make it 0 and carry = 1.
- For the next iteration check carry and if it adds to 10, do the same as step 2.
- After adding carry, make carry = 0 for the next iteration.
- If the vectors add and increase the vector size, append 1 in the beginning.

```
pub fn array_plus_one(nums: &mut std::collections::VecDeque<usize>) {
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
```
