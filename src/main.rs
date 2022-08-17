fn unary(n: u64) -> String {
    return "0".repeat(n as usize) + "1";
}

fn binary(n: u64) -> String {
    return format!("{n:b}");
}

fn reduced_binary(n: u64) -> String {
    return String::from(binary(n + 1).strip_prefix("1").unwrap());
}

fn gamma(n: u64) -> String {
    let red = reduced_binary(n);
    return unary(red.len() as u64) + &red;
}

fn delta(n: u64) -> String {
    let red = reduced_binary(n);
    return gamma(red.len() as u64) + &red;
}

fn minimal_binary(n: u64, k: u64) -> String {
    let s = (k as f64).log2().ceil() as usize;
    let m = 2u64.pow(s as u32) - k;

    if n < m {
        return format!("{:0>t$b}", n, t = s - 1)
    }
    return  format!("{:0>t$b}", n + m, t = s)
}

fn golomb(n: u64, b: u64) -> String {
    return unary(n / b) + &minimal_binary(n % b, b);
}

fn main() {
    for x in 0..6 {
        println!("{:?} => {}", x, golomb(x, 3));
    }
}