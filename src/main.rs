fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn summing_up() {
        // let numbers = vec![1.0, 5.0, 2.0, 0.5, 1.5];
        let mut numbers: Vec<f64> = Vec::new();
        numbers.push(1.0);
        numbers.push(5.0);
        numbers.push(4.0);

        let result: f64 = numbers.iter().sum();

        assert_eq!(result, 10.0)
    }
}
