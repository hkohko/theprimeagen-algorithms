pub fn linear_search_main() {
    let haystack = [3,4,5,6,7];
    let needle = 5;
    let res = linear_search(&haystack, needle);
    dbg!(&res);
}
fn linear_search(haystack: &[i32], needle: i32) -> bool {
    for n in haystack.iter() {
        if n == &needle {
            return true
        } 
    }
    false
    
}