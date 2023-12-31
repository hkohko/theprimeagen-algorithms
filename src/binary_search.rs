pub fn binary_search_main() {
    let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let needle = 10;
    let res = binary_search(&haystack, &needle);
    dbg!(&res);
}
fn binary_search(haystack: &[i32], needle: &i32) -> bool {
    //haystack MUST be sorted
    let mut lo = 0;
    let mut hi = i32::try_from(haystack.len()).expect("");
    while lo < hi {
        //midpoint formula + NO SCANNING (no walking through the array) makes this algo O(logN)
        let midpoint = usize::try_from(lo + (hi - lo) / 2).expect("");
        let value = haystack[midpoint];
        if &value == needle {
            return true;
        } else if &value > needle {
            hi = i32::try_from(midpoint).expect("");
        } else {
            lo = i32::try_from(midpoint + 1).expect("");
        }
    }
    false
}
#[test]
fn passing_tests() {
    let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for n in 1..=10 {
        println!("{}", &n);
        assert_eq!(binary_search(&haystack, &n), true);
    }
}
#[test]
fn failing_tests() {
    let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for n in 11..100 {
        assert_eq!(binary_search(&haystack, &n), false);
    }
}
