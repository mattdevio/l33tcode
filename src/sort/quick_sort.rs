#[allow(dead_code)]
// Quick sort implementation
// copyright matzxrr
// ----------------------------------------

pub fn sort(input: &[i32]) -> Vec<i32> {
    let mut array = input.to_owned();
    quick_sort(&mut array);
    array
}

fn quick_sort(array: &mut [i32]) {
    if array.len() <= 1 {
        return;
    }
    let pivot = partition(array);
    quick_sort(&mut array[..pivot]);
    quick_sort(&mut array[pivot + 1..]);
}
fn partition(array: &mut [i32]) -> usize {
    let mut i = 0;
    let right = array.len() - 1;

    for j in 0..right {
        if array[j] <= array[right] {
            array.swap(j, i);
            i += 1;
        }
    }
    array.swap(i, right);
    i
}

#[cfg(test)]
mod quick_sort_test {
    use super::*;

    #[test]
    fn a() {
        let input = vec![1, 9, 3, 2, 10, 4, 12];
        let expected = vec![1, 2, 3, 4, 9, 10, 12];
        let result = sort(&input);

        assert_eq!(expected, result);
    }
}
