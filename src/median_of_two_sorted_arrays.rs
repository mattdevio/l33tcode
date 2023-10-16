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


        let sl = small.len();
        let bl = big.len();

        let mut lo = 0;
        let mut hi = sl;
        let combined_len = sl + bl;

        while lo <= hi {
            let partition_x = (lo + hi) / 2;
            let partition_y = (combined_len + 1) / 2 - partition_x;


            let max_x = if partition_x == 0 { i32::MIN } else { small[partition_x - 1] };
            let min_x = if partition_x == sl { i32::MAX } else { small[partition_x] };

            let max_y = if partition_y == 0 { i32::MIN } else { big[partition_y - 1] };
            let min_y = if partition_y == bl { i32::MAX } else { big[partition_y] };

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
