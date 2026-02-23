use std::{env, sync::mpsc};
use port_scan::{Config, ThreadPool, WorkPackage};

fn main(){

    let args : Vec<String> = env::args().collect();
    let config = Config::build(args);

    let (result_sender , result_receiver) = mpsc::channel();
     
    let pool = ThreadPool::new(config.get_size(),result_sender);

    for port in config.get_start()..=config.get_end()  {
        let workpackage : WorkPackage = WorkPackage::new(config.get_ip(), port);
        pool.execute(workpackage);
    }

    drop(pool);

    let mut open_ports : Vec<u16> = Vec::new();

    for i in result_receiver{
        open_ports.push(i);
    }

    open_ports.sort();
    for port in open_ports {
        println!("port {} is open",port);
    }
}

