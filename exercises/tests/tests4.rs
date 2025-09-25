// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        //正常创建，检查字段
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        //检查负宽度触发 panic
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        //检查负高度触发 panic
        let _rect = Rectangle::new(10, -10);
    }
}
