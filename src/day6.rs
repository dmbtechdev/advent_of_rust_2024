// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let length_1 = s1.trim().chars().count();
    let length_2 = s2.trim().chars().count();
    if length_1 > length_2 {
        Some(s1)
    } else if length_1 == length_2 {
        None
    } else {
        Some(s2)
    }
}
