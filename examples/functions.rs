fn main() {}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::process_int;

    #[test]
    fn functions_def() {
        let increment_v1 = |x: u32| -> u32 { x + 1 };
        let increment_v2 = |x| { x + 1 };
        let increment_v3 = |x| x + 1;

        assert_eq!(increment_v2(4), 5);
        assert_eq!(increment_v3(4), 5);
    }

    #[test]
    fn higher_order_fun_with_parameters() {
        let res = process_int(1, 2, |x, y| x + y);
        assert_eq!(res, 3);

        let mul = |x, y| x * y;
        let res = process_int(3, 4, mul);
        assert_eq!(res, 12);
    }

    #[test]
    fn clousure_intro() {
        let mut x = 5;
        let mul_5 = |a| a * x;

        assert_eq!(mul_5(4), 20);
    }

    #[test]
    fn clousure_fnmut() {
        let mut x = 5;
        let mut add_to_x = |a| x += a;

        add_to_x(5);
        assert_eq!(x, 10);

    }

    #[test]
    fn clousure_fnonce() {
        let mut x = vec![1, 2, 3];
        let equal_to_x = move |y| y == x;

        // println!("can't use x here: {:?}", x);

        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));

    }

    #[test]
    fn iter_next() {
        let v = vec![1, 2, 3];
        let mut iter = v.iter(); // must be mutable

        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn consuming_iters() {
        let v = vec![1, 2, 3];
        let iter = v.iter();
        assert_eq!(6, iter.sum()); // consumes the iterator

        let v = vec![1, 2, 3];
        assert_eq!(3, v.iter().count()); // consumes the iterator

        let a = [1, 2, 3];
        let v = a.iter().collect::<Vec<_>>(); // turbofish syntax
        assert_eq!(&1, *v.get(0).unwrap()); // :>
    }

    #[test]
    fn transforming_iter() {
        let v = vec![1, 2, 3];
        // transforming each element
        let res = v.iter().map(|x| x * 2).collect::<Vec<i32>>();
        assert_eq!(vec![2, 4, 6], res);

        // filter elements
        let res = v.iter().filter(|&x| *x % 2 == 0).map(|x| *x).collect::<Vec<i32>>();
        assert_eq!(vec![2], res);

    }

    #[test]
    fn using_iter_mut() {
        let mut v = vec![1, 2, 3];

        v.iter_mut().map(|x| *x +=1).collect::<Vec<_>>();
        assert_eq!(vec![2, 3, 4], v);
    }

    #[test]
    fn using_iter_into() {
        let v = vec![1, 2, 3];
        let v2 = v.into_iter().map(|x | x * 2).collect::<Vec<_>>();

        // v.get(0); // cannot use v anymore - it's moved

        assert_eq!(vec![2, 4, 6], v2);

    }
}

fn process_int<F>(a: i32, b: i32, f: F) -> i32
    where F: Fn(i32, i32) -> i32 {
    f(a, b) //applies the function
}