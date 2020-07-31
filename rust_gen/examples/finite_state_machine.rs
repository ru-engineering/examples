/*
    Author, Kjartan Bjarmi Arnason Klein
    Date 08.2020
    Purpose:
    This example should help the reader understand finite state machines
    There are 2 main types the "random number" type and the "binary encoded" type
    This example is for the random number type.

    Why call it random number type? and what does it do?
    Simply each code should have a couple of states by default,
    fx error state and initializer state.
    In the random number type you would give theese staes a number.
    in our example you would give the initializer state '0' and the error state '-1'

    Typicaly the states of the code would go from 0 -> 1 ->2 ...->n
    you could also have function states so when the code coes into function() it would get state x
    etc. The problem with it is that the numbers end up becoming ectrmly complicated.

    in the binary encoded type this is a bit diffrent, see the other example

    This example will use an elevator as an example.
*/
use std::time::Duration;
use std::thread;

fn main(){
let mut state :i16 = 0;
let mut current_floor: i8 = 2; //this would be done with a sensor but just play prented here
let mut runner_floor: i8 = 127;
println!("code started, random number FSM state is {} ",state );
while state != -1{
    match state {
    0 => {
        //method one of enforcing the state system. and a good one as it can have otputs
        print!("state is {} powering on",state);
        match power_on(){
            Ok(_) =>     {state  =   state + 1}, //using state = state+1 is typical, but can fail
            Err(_) =>    {state =   -1}, //-1 is the error state
        }
        println!("\t ==> {}", state);
        runner_floor = 2;   //just setting this now so it has some value. there are some ways elevators do this
    }

    1 => {
            print!("state is {} getting current floor",state);
            //this is just as valid but if you have more than one error state this becomes more difficult
            if is_floor_legit(current_floor,runner_floor) {
                current_floor = runner_floor;   //set current flooor
                state = 2;
                runner_floor = 127;//knowing my code this is an unvalid floor thus if something fails i can see it
            }
            else{
                state = -1; //set to error
            }
            println!("\t ==> {}",state);
    }

    2 => {
        print!("state is {} getting new floor",state);
        match get_floor(current_floor){ //should just add 1 to the runner floor.
            Ok(k) => {
                runner_floor = k;   //if ok then continue
                state = state+1;
            },
            Err(_) => {
                state = -1; //else fail
            },
        }

    }

    3 => {
        print!("state is {} getting new floor",state);
        runner_floor = goto_floor(current_floor); //somethimes it is a prcoess that cant be checked
        //then it is ok to not have an error state, however this is bad behaviour
        state = 1;
        println!(" --->  Sucessfull! going back to state 1")
    }

    -1 => {
        panic!("Something went wrong check log");
    }

    _ => {
        panic!("unkown state reached {}",state);
    }
    }
}
}

fn power_on() -> Result<(),()>{
    //empty function with a delay so the code doesnt just spam the screen
    println!("powering on");
    thread::sleep(Duration::from_millis(1000));
    Ok(())
}
fn is_floor_legit(old:i8,new:i8) -> bool{
    old > -5 && old < 5 && new > -5 && new < 5
    //checking if legit, the idea here is that the building has only +-5 floors
}

fn goto_floor(goto:i8)->i8{
    //basicly a delay function so the code is not rapid
    print!("   -ELEVATOR MUSIC-    ");
    thread::sleep(Duration::from_millis(3000));
    goto
}

fn get_floor(old:i8) -> Result<i8,()> {
    //this woudl be getting from a pannel or something, play along as this will give an error eventialy
    if is_floor_legit(old , old+1){
        return Ok(old+1 as i8)
    }else{

    Err(())
    }
}
