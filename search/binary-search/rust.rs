fn binary_search(v: &Vec<i32>, target: i32) -> Option<i32> {
    let mut min: usize = 0;
    let mut max: usize = v.len() - 1;
    let mut found: Option<i32> = None;

    while min <= max {
        let mid: usize = min + (max - min) / 2;

        if target == v[mid] {
            found = Some(target);
            break;
        }

        if target > v[mid] {
            min = mid + 1;
        }

        if target < v[mid] {
            max = mid - 1;
        }
    }

    found
}

#[test]
fn test_search() {
    let v: Vec<i32> = vec![1,2,3,4];

    assert_eq!(binary_search(&v, 2), Some(2));
    assert_eq!(binary_search(&v, 5), None);
    assert_eq!(binary_search(&v, 4), Some(4));
}
