pub fn quick_sort<T>(input: &[T]) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let mut result = input[..].to_owned();
    sort_part(&mut result);
    result
}

pub fn sort_part<T>(input: &mut [T])
where
    T: PartialOrd,
{
    if input.len().lt(&1) {
        return;
    }
    let pivot = partition(input);
    sort_part(&mut input[..pivot]);
    sort_part(&mut input[pivot + 1..]);
}

pub fn partition<T>(input: &mut [T]) -> usize
where
    T: PartialOrd,
{
    let mut i = 0;
    let right = input.len() - 1;
    for j in 0..right {
        if input[j] <= input[right] {
            input.swap(j, i);
            i += 1;
        }
    }
    input.swap(i, right);
    i
}

#[cfg(test)]
mod test_qs {
    use super::*;

    #[test]
    fn it_should_work() {
        let input = vec![9, 1, 5, 22, 180, 5, 23];
        let expected = vec![1, 5, 5, 9, 22, 23, 180];
        let result = quick_sort(&input);
        assert_eq!(expected, result);
    }
}
