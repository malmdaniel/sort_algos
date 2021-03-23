
pub fn insertion_sort<T: std::cmp::PartialOrd>(arr: &mut [T] ) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1]{
            arr.swap(j, j -1);
            j -= 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let mut names = ["bert", "adam","cedric"];
        insertion_sort(&mut names);
        assert_eq!(["adam","bert","cedric"], names);
    }

    #[test]
    fn letters() {
        let mut letters = ['b', 'k', 'A', 'p'];
        insertion_sort(&mut letters);
        assert_eq!(['A','b','k','p'], letters);
    }

    #[test]
    fn int_numbers() {
        let mut integers_list = [3,2,6,9,1,0];
        insertion_sort(&mut integers_list);
        assert_eq!([0,1,2,3,6,9], integers_list);
    }
    #[test]
    fn float_numbers() {
        let mut float_list = [3.9, 3.7, 5.1, 0.1];
        insertion_sort(&mut float_list);
        assert_eq!([0.1,3.7,3.9,5.1], float_list);
    }

    #[test]
    fn negative_values() {
        let mut negative_values = [-2,-7,-100,-54];
        insertion_sort(&mut negative_values);
        assert_eq!([-100,-54,-7,-2], negative_values);
    }
    #[test]
    fn same_numbers() {
        let mut same = [3,3,3,3,3];
        insertion_sort(&mut same);
        assert_eq!([3,3,3,3,3], same);
    }

}