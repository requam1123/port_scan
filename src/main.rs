use std::{env};
use port_scan::{Config, ThreadPool, WorkPackage};

fn main(){

    let args : Vec<String> = env::args().collect();
    let config = Config::build(args);
    let pool = ThreadPool::new(config.get_size());

    for port in config.get_start()..=config.get_end()  {
        let workpackage : WorkPackage = WorkPackage::new(config.get_ip(), port);
        pool.execute(workpackage);
    }
}

