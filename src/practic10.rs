pub fn rotate(s: &str, n: isize) {
    let len = s.len() as isize;
    if len == 0 {
        println!("{}", s);
        return;
    }

    let shift = ((n % len) + len) % len;
    let split = len as usize - shift as usize;

    println!("{}{}", &s[split..], &s[..split]);
}