//! # pythagorean-triplet
//!
//! 'A Pythagorean triple' consists of three positive integers a, b, and c,
//! such that a2 + b2 = c2. Such a triple is commonly written (a, b, c), and
//! awell-known example is (3, 4, 5). If (a, b, c) is a Pythagorean triple,
//! then so is (ka, kb, kc) for any positive integer k.

/// find () returns option <u32>
///
///   # Examples
///  ```
///  let target: u32 = 1000;
///  for a in 1..target/2 {
///   for b in 1..target/2 {
///    if b >= a {
///    let c = target - (a+b);
///     if a.pow(2)+b.pow(2) == c.pow(2){
///  ...
///  ```
/// This app initialises the target value which in this case is 1000.
///
/// Then two for loops which are optized to use the same target value but divided by 2.
/// This reduces the number of duplicate iterations within the loop i.e a 60 one way is
/// the same as b 60 the other for loop.
/// The rest of the app logic should make sence otherwise.
///
pub fn find() -> Option<u32> {

    let target: u32 = 1000; // target value of a+b+c=1000
    for a in 1..target/2 {
        for b in 1..target/2 {
        if b >= a {
            let c = target - (a+b);
            println!("[Testing] a: {}, b: {}, c:{}",a,b,c);
                if a.pow(2)+b.pow(2) == c.pow(2){
                    println!("[FOUND] ----> a: {} + b: {} + c: {} = {}",a,b,c,a+b+c);
                    println!("[FOUND] ----> a**2: {} + b**2: {} = c**2: {}",a.pow(2),b.pow(2),c.pow(2));
                    println!("[FOUND] ----> result: a**2 * b**2 * c**2 = {}",a*b*c);
                    return Some(a*b*c);
                }
            }
        }
    }
    //If we get here it's an error result
    return None
}
