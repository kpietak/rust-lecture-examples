use std::fs;
use anyhow::{anyhow, bail, Result};


fn main() {

}

#[derive(Debug,Clone)]
struct Rectangle {
    width : f64,
    height : f64
}

const DEFAULT_WIDTH: f64 = 1.0;
const DEFAULT_HEIGHT: f64 = 1.0;
impl Rectangle {
    fn new(width : f64, height : f64) -> Result<Rectangle> {
        if width < 0.0 || height < 0.0 {
            Err(anyhow!("Rectangle cannot have negative width or height"))
        } else {
            Ok(Rectangle { width, height })
        }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn convert_to_int(&self) -> Result<(u32, u32)> {

        let w = self.width;
        let h = self.height;

        if w == (w as u32) as f64 && h == (h as u32) as f64 {
            Ok((w as u32, h as u32))
        } else {
            bail!("Cannot convert to int")
        }
    }

    fn read_from_file(path : &str) -> Result<Rectangle> {
        let s = fs::read_to_string(path)?;
        let mut iter = s.split_whitespace();
        let width : f64 = iter.next().ok_or(anyhow!("Cannot convert string to width"))?.parse()?;
        let height : f64 = iter.next().ok_or(anyhow!("Cannot convert string to height"))?.parse()?;

        Ok(Rectangle::new(width, height)?)

    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle{ width : DEFAULT_WIDTH, height : DEFAULT_HEIGHT }
    }
}

fn bigger(w1: f64, h1 : f64, w2 : f64, h2 : f64) -> Result<Rectangle> {
    let r1 = Rectangle::new(w1, h1)?;
    let r2 = Rectangle::new(w2, h2)?;

    if r1.area() > r2.area() {
        Ok(r1)
    } else {
        Ok(r2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rectangle() {
        // given
        let width = 4.5;
        let height = 5.7;

        // when
        let r = Rectangle::new(width, height).unwrap();

        // then
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);

    }

    #[test]
    fn test_new_rectangle_with_negative() {
        let r = Rectangle::new(-1.0, 1.0);
        match r {
            Err(e) => assert_eq!("Rectangle cannot have negative width or height", e.to_string()),
            Ok(_) => panic!() // the result shouldn't be Ok
        }
    }


    #[test]
    #[should_panic]
    fn test_unwrap() {
        // that is ok
        let r = Rectangle::new(1.0, 2.0);
        let _rec = r.unwrap(); // consumes the value

        // that generates panic
        let r = Rectangle::new(-1.0, 2.0);
        let _rec = r.unwrap(); // consumes the value

    }

    #[test]
    #[should_panic]
    fn test_expect() {
        // that is ok
        let r = Rectangle::new(1.0, 2.0);
        let _rec = r.expect("This should be always a proper rectangle");

        // that generates panic
        let r = Rectangle::new(1.0, -2.0);
        let _rec = r.expect("Don't do that!");
    }

    #[test]
    fn test_is_ok() {
        let r = Rectangle::new(1.0, 2.0);
        assert!(r.is_ok());
        assert!(!r.is_err());
    }

    #[test]
    fn test_unwrap_or_else() {
        let r = Rectangle::new(-1.0, -2.0);
        let rec = r.unwrap_or_else(|_| Rectangle::new(DEFAULT_WIDTH, DEFAULT_HEIGHT).unwrap());

        assert!((rec.width - DEFAULT_WIDTH).abs() < f64::EPSILON && (rec.height - DEFAULT_HEIGHT).abs() < f64::EPSILON);
    }

    #[test]
    fn test_unwrap_or_default() {
        let r = Rectangle::new(-1.0, -2.0);
        let rec = r.unwrap_or_default();

        assert!((rec.width - DEFAULT_WIDTH).abs() < f64::EPSILON && (rec.height - DEFAULT_HEIGHT).abs() < f64::EPSILON);

        let r = Rectangle::new(1.0, 2.0);
        let rec = r.unwrap_or_default();
        assert!((rec.width - 1.0).abs() < f64::EPSILON && (rec.height - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bigger() {
        // given
        let w1 = 1.0;
        let h1 = 2.0;
        let w2 = 3.0;
        let h2 = 4.0;

        // when
        let r = bigger(w1, h1, w2, h2);

        // then
        assert!((r.unwrap().area() - (w2 * h2)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bigger_err() {
        // given
        let w1 = 1.0;
        let h1 = -2.0; // wrong width
        let w2 = 3.0;
        let h2 = 4.0;

        // when
        let r = bigger(w1, h1, w2, h2);

        // then
        assert!(r.is_err());
    }

    #[test]
    fn test_read_from_not_existing_file() {
        // given
        let path = "not-existing-file.txt";

        // when
        let r = Rectangle::read_from_file(path);

        // then
        match r {
            Err(ref e) if e.is::<std::io::Error>() => println!("ok"),
            _ => panic!()
        }
    }

}
