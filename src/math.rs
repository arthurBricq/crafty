
pub fn gcd(mut a:u32, mut b:u32) -> u32{
    if a==b { return a; }
    if b > a { let temp = a; a = b; b = temp; }
    while b>0 { let temp = a; a = b; b = temp%b; }
    return a;
}

pub fn lcm(a:u32, b:u32) -> u32{
    return a*(b/gcd(a,b));
}