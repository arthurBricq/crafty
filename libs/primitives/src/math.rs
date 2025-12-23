pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

pub fn lcm(a: u32, b: u32) -> u32 {
    return a * (b / gcd(a, b));
}

#[cfg(test)]
mod tests {
    use crate::math::{gcd, lcm};

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(3, 2), 6);
        assert_eq!(lcm(1, 5), 5);
        assert_eq!(lcm(3, 5), 15);
        assert_eq!(lcm(7, 7), 7);
        assert_eq!(lcm(15, 10), 30);
        assert_eq!(lcm(15, 30), 30);
        assert_eq!(lcm(6, 35), 210);
    }

    #[test]
    fn test_gcm() {
        assert_eq!(gcd(3, 2), 1);
        assert_eq!(gcd(3, 6), 3);
        assert_eq!(gcd(8, 8), 8);
        assert_eq!(gcd(1, 9), 1);
        assert_eq!(gcd(10, 15), 5);
    }
}
