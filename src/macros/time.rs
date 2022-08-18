#[macro_export]
macro_rules! calc_time {
    ($fn:ident) => {
        let timer = std::time::Instant::now();
        $fn();
        println!("time = {:?}", timer.elapsed());
    };
}
