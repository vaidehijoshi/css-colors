#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    // red between 0-255
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,
}
