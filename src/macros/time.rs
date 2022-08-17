use std::time::Instant;
macro_rules! calc_time {
    ($fn:ident) => {
        let timer = Instant::now();
        $fn();
        println!("time = {:?}", timer.elapsed());
    };
}
