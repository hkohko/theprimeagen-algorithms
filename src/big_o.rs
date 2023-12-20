pub fn still_On() {
    //for loops are O(n) (linear time)

    //O(N)l
    let mut sum = 0;
    for num in 0..=10 {
        sum += num;
    }
    //function still_0n() is still O(N) with the addition of this for loop
    //O(2N) <- drop constants :D
    for num in 1..=10 {
        sum *= num;
    }
    dbg!(&sum);
}
pub fn O_N_squared() {
    let v = vec!["hello".to_string(), "world".to_string()];
    //Nested for loops -> O(N^2)
    for element in v.iter() {
        for c in element.chars() {
            println!("{c}");
        }
    }
}
