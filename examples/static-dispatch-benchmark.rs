// based on https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
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

    let backend = Box::new(NegativeBackend);

    let mut res= 0 as i64;

    let start_time = SystemTime::now();

    let total = 20_000_000 as i64;

    for i in 0 .. total {
        res += backend.compute(i) + res;
    }

    println!("Result: {}",res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());

}
