use std::time::{Duration};
use std::thread;
use std::sync::mpsc;

fn main(){
//other ways to do timing is to use threads, this sometimes ignores all timing constraints anyways

let child1 = thread::spawn(||{
    println!("working hard...");
    thread::sleep(Duration::from_millis(5000));
    println!("..or hardly working!");//thread stops here
    return 25
});
//this thread should never stop
let mut i :u8 = 0;
let _child2 = thread::spawn(move||loop{
    i=i+1;
    println!("{}",i);
    thread::sleep(Duration::from_millis(1000));
});

let (killswitch, death) = mpsc::channel();
let new_thread = my_thread_starter(death);

//wait  for child 1 to rejoin
let child_mess = child1.join().expect("child1 has somehow died, please call an ambulance");
println!("child 1 returned {}",child_mess);
killswitch.send(1u8).unwrap();
match new_thread.join(){
    Ok(x) => println!("thread exited with {:?}", x),
    Err(mut x) => println!("thread paniced with {:?}",x.downcast_mut::<&str>()),
    }
}


fn my_thread_starter(signal: mpsc::Receiver<u8>)-> thread::JoinHandle<()>{
let mut i  = 0;
    thread::spawn(move ||{
        println!("function thread started");
        loop{
            i=i+1;
            thread::sleep(Duration::from_millis(1000));
            match signal.try_recv(){
                Ok(_) => {
                println!("Got kill singal i is at {}",i);
                panic!("I am dead now");
                }
                Err(_) => {}
            };
        }
    })
}
