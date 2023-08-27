trait ToBase {
    fn to_base(&self, base: i32) -> String;
}

impl ToBase for i32 {
    fn to_base(&self, base: i32) -> String {
        let charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
        let mut new_num = String::new();
        let mut n = *self;
        while n > 0 {
            let (q, r) = (n / base, n % base);
            new_num.insert(0, charset[r as usize] as char);
            n = q;
        }
        new_num
    }
}

fn main() {
    let number = 123123;
    println!("{} in base 2 is {}", number, number.to_base(100));
}
