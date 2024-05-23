// based on https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
use std::env;
use std::time::SystemTime;

trait Backend{
    fn compute(&self,number: i64) -> i64;
}

struct PositiveBackend;
struct NegativeBackend;

impl Backend for PositiveBackend{
    #[inline(never)]
    fn compute(&self,number: i64) -> i64{
        number+1
    }
}

impl Backend for NegativeBackend{
    #[inline(never)]
    fn compute(&self,number: i64) -> i64{
        number-1
    }
}


fn main() {

    // decide which backend to use based on a user-set program argument
    let backend = match env::args().skip(1).next() {
        Some(_x) => Box::new(PositiveBackend ) as Box<dyn Backend>,
        _ => Box::new(NegativeBackend) as Box<dyn Backend>
    };

    let mut res= 0i64;

    let start_time = SystemTime::now();

    let total = 20_000_000i64;

    for i in 0 .. total {
        res += backend.compute(i) + res;
    }

    println!("Result: {}",res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());

}
