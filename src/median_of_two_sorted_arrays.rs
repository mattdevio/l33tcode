/// # Median of two sorted arrays
///
/// Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively.
/// Return *the median* of the two sorted arrays
/// The overall run time complexity should be O(log (m+n))

pub mod solution {

    pub fn find_median_softed_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (small, big) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        println!("SMALL -> {:?}", small);
        println!("BIG   -> {:?}", big);


        let mut lo = 0;
        let mut hi = small.len();
        let combined_len = small.len() + big.len();

        while lo <= hi {
            let partition_x = (lo + hi) / 2;
            let partition_y = (combined_len + 1) / 2 - partition_x;


            let max_x = get_max(&small, partition_x);
            let min_x = get_min(&small, partition_x);
            
            let max_y = get_max(&big, partition_y);
            let min_y = get_min(&big, partition_y);
            
            if max_x <= min_y && max_y <= min_x {
                if combined_len % 2 == 0 {
                    return (max_x.max(max_y) + min_x.min(min_y)) as f64 / 2.0;
                }
                return max_x.max(max_y) as f64;
            } else if max_x > min_y {
                hi = partition_x - 1;
            } else {
                lo = partition_x + 1;
            }

        };
        
        // Should never reach this with sorted arrays
        unreachable!()

    }


    pub fn get_max(nums: &Vec<i32>, partition: usize) -> i32 {
        if partition == 0 {
            i32::MIN
        } else {
            nums[partition - 1]
        }
    }

    pub fn get_min(nums: &Vec<i32>, partition: usize) -> i32 {
         if partition == nums.len() {
            i32::MAX
        } else {
            nums[partition]
        }
    }

}


#[cfg(test)]
mod test_4 {
    use super::solution::find_median_softed_arrays;

    #[test]
    fn test_a() {
        // Arrange
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let answer = 2.0;
        // Act
        let result = find_median_softed_arrays(nums1, nums2);
        // Assert
        assert_eq!(answer, result);
    }

    #[test]
    fn test_b() {
        // Arrange
        let nums1 = vec![3, 4, 5, 8, 9];
        let nums2 = vec![1, 2, 3, 5];
        let answer = 4.0;
        // Act
        let result = find_median_softed_arrays(nums1, nums2);
        // Assert
        assert_eq!(answer, result);
    }

    #[test]
    fn test_c() {
        // Arrange
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let answer = 2.5;
        // Act
        let result = find_median_softed_arrays(nums1, nums2);
        // Assert
        assert_eq!(answer, result);
    }
}
