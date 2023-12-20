pub fn binary_search_main() {
    let haystack = [1,2,3,4,5,6,7,8,9,10];
    let needle= 1;
    let res = binary_search(&haystack, &needle);
    dbg!(&res);
}
fn binary_search(haystack: &[i32], needle: &i32) -> bool {
    //haystack MUST be sorted
    let mut lo = 0;
    let mut hi = i32::try_from(haystack.len()).expect("");
    while lo < hi {
        let midpoint = lo + (hi - lo) / 2;
        if &midpoint == needle {
            return true
        } else if &midpoint > needle {
            hi = midpoint;
        } else {
            lo = midpoint + 1;
        }
    }
    false
}
#[test]
fn passing_tests() {
    let haystack = [1,2,3,4,5,6,7,8,9,10];
    for n in 0..10 {
        println!("{}", &n);
        assert_eq!(binary_search(&haystack, &n), true);
    }
}
#[test]
fn failing_tests() {
    let haystack = [1,2,3,4,5,6,7,8,9,10];
    for n in 10..100 {
        assert_eq!(binary_search(&haystack, &n), false)
    }
}