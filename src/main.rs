fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn simple_test() {
        assert_eq!(2+2, 4);
    }
}