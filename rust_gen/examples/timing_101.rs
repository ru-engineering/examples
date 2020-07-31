use std::time::{Duration,Instant};
use std::thread;

fn main(){
    //Basic timing is just waiting for stuff, for example wait for 5 seconds
    let mut start = Instant::now();
    println!("Hello there the time is now {:?}",start.elapsed());

    thread::sleep(Duration::from_millis(2000)); //sleeps for 2 seconds
    print!("\n{:?} --> ",start.elapsed());
    //we can also predefine the time or use a function
    let time1 = Duration::from_secs(2); //2 seconds
    thread::sleep(time1);
    print!("{:?} --> ",start.elapsed());
    my_sleep(1000);//sleeps for one second
    println!("{:?}",start.elapsed());
    println!("\n\n");
    //sometimes we want to have a function do something for x amount of time
    //and once it is done continue. This can also be used as a timeout as it is a simple
    //boolean that drives it
    start = Instant::now();
    let mut i = 0;
    while start.elapsed() < Duration::from_secs(3){
        i=i+1;
        print!("{} ",i);
        my_sleep(500);
    }
    println!("!");
    println!("went 6 times for 0.5s ")
    //this should result in it counting to 6
}


fn my_sleep(time : u64){
    //blocks and sleeps
    thread::sleep(Duration::from_millis(time));
}
