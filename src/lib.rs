use std::{
    net::{Ipv4Addr,SocketAddr,TcpStream},
    thread,
    sync::{mpsc,Arc,Mutex},
    time::Duration
};
#[derive(Debug)]
pub struct Config{
    ip         : Ipv4Addr,
    begin_p    : u16,
    end_p      : u16,
    thread_num : usize,
}


impl Config{
    pub fn build(args:Vec<String>) -> Config{
        let ip =args[1].parse().expect("无效的 IP 地址格式");
        
        let begin_p = args[2].parse().unwrap();
        let end_p = args[3].parse().unwrap();
        let thread_num = args[4].parse().unwrap();

        Config { ip, begin_p, end_p, thread_num }
    }
    pub fn get_size(&self) -> usize{
        self.thread_num
    }
    pub fn get_start(&self) -> u16{
        self.begin_p
    }
    pub fn get_end(&self) -> u16{
        self.end_p
    }
    pub fn get_ip(&self) -> Ipv4Addr{
        self.ip
    }
}

pub struct ThreadPool {
    sender : Option<mpsc::Sender<WorkPackage>>,
    workers: Vec<Worker>,
}
impl ThreadPool{
    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);
        assert!(size < 101);
        
        let (sender , receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers : Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool { sender : Some(sender), workers }
    }
    pub fn execute(&self,workpackage : WorkPackage){
        self.sender.as_ref().unwrap().send(workpackage).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("Shutdown Worker{}",worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

pub struct WorkPackage{
    addr : SocketAddr,
}
impl WorkPackage {
    pub fn new(ip:Ipv4Addr,port :u16) -> WorkPackage{
        WorkPackage { addr: SocketAddr::from((ip,port)) }
    }
}


struct Worker{
    id : usize,
    thread : Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id:usize,receiver : Arc<Mutex<mpsc::Receiver<WorkPackage>>>) -> Worker{
        let thread = thread::spawn(move|| loop{
            let workpackage = receiver.lock().unwrap().recv();

            match workpackage{
                Ok(workpackage) => {
                    
                    println!("Worker {id} , Receiver addr {}  Working",workpackage.addr);
                    let timeout = Duration::from_secs(3);

                    match TcpStream::connect_timeout(&workpackage.addr, timeout){
                        Ok(_stream) => {
                            println!("Port {} connected successfully",workpackage.addr.port());
                        }
                        Err(e) => {
                            println!("Port {} connected Failed for reason {}",workpackage.addr.port(),e);
                        }
                    }
                }
                Err(_) => {
                    println!("睡觉睡觉睡");
                    break;
                }
            }
        });
        Worker { 
            id,
            thread: Some(thread) 
        }
    }
}