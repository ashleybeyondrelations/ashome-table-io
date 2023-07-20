#![allow(unused_parens)]
extern crate daemonize;


use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs::File;
use std::{thread, time};
use std::str;
use std::time::{Instant,Duration};
use std::collections::HashMap;

use daemonize::Daemonize;
//use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};

fn main() {
    let stdout = File::create("/usr/local/ashux/log/event_handler.out").unwrap();
    let stderr = File::create("/usr/local/ashux/log/event_handler.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/usr/local/ashux/event_handler/daemon.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/usr/local/ashux/event_handler") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
//        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => scan(),
        Err(e) => eprintln!("Error, {}", e),
    }
}
fn test()
{
	let mut instring = "x";
                println!("{}: {:?}", instring, instring.as_bytes());
	instring = "u";
                println!("{}: {:?}", instring, instring.as_bytes());
	instring = "d";
                println!("{}: {:?}", instring, instring.as_bytes());
	instring = "p";
                println!("{}: {:?}", instring, instring.as_bytes());
	
}
fn scan()
{

//	Numrow1Key.
	    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
	    
//	    listener.set_nonblocking(true).expect("Cannot set non-blocking");
	    
	    let mut keep_alive = true;
    // accept connections and process them, spawning a new thread for each one
    inputbot::KeybdKey::AKey.bind(|| println!("Pressed a") );
	inputbot::KeybdKey::OtherKey(71).bind(|| println!("Pressed F5") );
	inputbot::KeybdKey::OtherKey(124).bind(|| println!("Pressed power") );
	println!("watching keys");
	inputbot::handle_input_events();

	println!("Server listening on port 3333");
	for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                
                keep_alive = establish_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
        if !keep_alive
        {
			println!("kill seen!");
			break;
		}
    }
    // close the socket server
    drop(listener);
}

pub struct GestureData {
//    list: Vec<i32>,
    actions: Vec<GestureEvent>,
    //can be derivedfrom the time of the first key
//    startTime: Instant,
//can be derived from if there are any actions
//    active: bool,
}

static long_hold_duration: Duration = time::Duration::from_millis(1500);

static short_gap_duration: Duration = time::Duration::from_millis(500);



impl GestureData{
	//the intent is to test the event after 
	//the last released key was released longer ago than the minimum between presses 
	//AND (the last key held has been held longer than required for a long press OR all the actions are released )
	fn evaluate(&mut self)
	{
		if (self.actions[0].state)
		{
			//sleep for long press
			thread::sleep(long_hold_duration);
		}
		if (self.actions.len() == 0 )
		{
			return;
		}

//		println!("looping through all pressed");
		
//		for cur_action in &self.getAllPressed()
//		{
//			println!("{}",cur_action.key);
//		}

		let mut shortest_since_release = short_gap_duration;

		let now = Instant::now(); 
		
		for cur_action in &self.getAllReleased()
		{
			if ( now.duration_since(cur_action.registered_at) < shortest_since_release )
			{
				shortest_since_release = now.duration_since(cur_action.registered_at);
			}
		}

		let mut shortest_since_press = long_hold_duration;
		
		for cur_action in &self.getAllPressed()
		{
			if ( now.duration_since(cur_action.registered_at) < shortest_since_press )
			{
				shortest_since_press = now.duration_since(cur_action.registered_at);
			}
		}
		
		if ( shortest_since_press >= long_hold_duration && shortest_since_release >= short_gap_duration )
		{
			//weve met the requirements; now we clear and process!
			let mut gesture_actions = self.actions.clone();
//					gesture_actions.sort_by(|a, b| a.registered_at < b.registered_at)
							gesture_actions.sort_by(|a, b| b.registered_at.cmp(&a.registered_at));

			&self.actions.clear();

				println!("evaluating combo");			
			
			for index in 0..gesture_actions.len() 
			{
				println!("key - {} : state : {}",gesture_actions[index].key,gesture_actions[index].state);
				
			}
		}
		
	}
	
	
	fn getAllPressed(&self)->Vec<GestureEvent>
	{
		let mut retval : Vec<GestureEvent> = self.actions.clone();
//							retval.sort_by(|a, b| a.registered_at.cmp(&b.registered_at))

		retval.retain(|pressed_event| 
			pressed_event.state 
			&& 
			!self.actions.iter().any(|relafter| !relafter.state && relafter.registered_at > pressed_event.registered_at && pressed_event.key == relafter.key) );

//		let released_keys : Vec<String> = Vec::new();

		//if n.iter().any(|&i| i=="-i")
		
		return retval;
//		for cur_action in &self.actions {
	//		if !cur_action.state && !released_keys.contains(cur_action.key)
		//	{
			//		released_keys.push(cur_action.key);
			//}
		//	else if cur_action.state && released_keys.contains(cur_action.key)
			//{
				//released_keys.retain(|&x| x != cur_action.key );				
			//}
			//println!("{}", i);
		//}

	}

	fn getAllReleased(&self)->Vec<GestureEvent>
	{
		let mut retval : Vec<GestureEvent> = self.actions.clone();
//							retval.sort_by(|a, b| a.registered_at.cmp(&b.registered_at))
		retval.retain(|released_event| 
			!released_event.state 
			&& 
			!self.actions.iter().any(|pressafter| pressafter.state && pressafter.registered_at > released_event.registered_at && released_event.key == pressafter.key) );
		return retval;
	}

	
	fn register(&mut self, key : String , state :bool) 
	{
		if !state && self.actions.len() == 0 
		{
			return;
		}
			
		let registered_at = Instant::now(); 
		
		let mut millis_from_start = 0; 
		if (self.actions.len() > 0)
		{
			millis_from_start=registered_at.duration_since(self.actions[self.actions.len()-1].registered_at).as_millis();
		}

//if it was down, we do a little cleanup by setting as having gone up before recording the new one		
		if state && self.getAllPressed().iter().any(|anypressed| key == anypressed.key)
		{
			
			let curkey = GestureEvent{
								key:key.to_string(),
								state:false,
								registered_at:registered_at - Duration::from_millis(1),
								millis_from_start:millis_from_start-1,
								};
			self.actions.push(curkey);
		}
		
		let curkey = GestureEvent{
								key,
								state,
								registered_at,
								millis_from_start,
								};
		self.actions.push(curkey);
	}
}

#[derive(Clone)]
pub struct GestureEvent {
    key: String,
    state: bool,
    registered_at : Instant,
    millis_from_start: u128,
}
pub struct ActionGesture{
    actions: Vec<ActionInput>,
}
pub struct ActionInput{
    key: String,
    state: bool,
}
 fn registeredGestures() -> Vec<ActionGesture> 
{
	
	let retval = vec![
ActionGesture{
	actions:vec![
		ActionInput{
		key:"XF86AudioRaiseVolume".to_string(),
		state:true,
		},
	]
	},
ActionGesture{
	actions:vec![
		ActionInput{
		key:"XF86AudioRaiseVolume".to_string(),
		state:true,
		},
		ActionInput{
		key:"XF86AudioRaiseVolume".to_string(),
		state:false,
		},
	]
	},	
];
return retval;
}

static mut CURRENT_EVENT: GestureData = GestureData{ 
	actions:Vec::new(),
//	startTime: Instant::now(),
	 };
//                    handle_client(listener,stream)

fn is_exit_signal(  stream:&mut TcpStream, signal : String) -> bool
{
		let exit_signal = "cmd:exit";

				if signal.len()>=8 && exit_signal==&signal[0..8]
				{
					println!("rec shutdown");

					let msg = b"shutting down";
					stream.write(msg).unwrap();
                        

					return true;
				}          
				else 
				{
					return false;
				}

}

fn resolve_signal(  stream:&mut TcpStream, signal : String) {

            // echo everything!
//					println!("sig : {}",signal);
//            stream.write(signal.as_bytes()).unwrap();

					let key_signal = "key:";
					if key_signal==&signal[0..4]
					{
						let msg = format!("rec {}",signal[6..].to_string());
						println!("rec {}",signal[4..].to_string());
						stream.write(msg.as_bytes()).unwrap();
						unsafe {
						let state_value = matches!(&signal[4..5], "true" | "t" | "1");
						CURRENT_EVENT.register(
							signal[6..].to_string(),
							state_value,			
							);				
							CURRENT_EVENT.evaluate();
						}
//						CURRENT_EVENT.actions.push(
//							GestureEvent{
//								key:signal[6..7].to_string(),
//								state:state_value,
//								millis_from_start:10,
//								});
//						let state=&signal[4..5];
	//					let msg=format!("state: {}", state);
//						stream.write(msg.as_bytes()).unwrap();
						//now we add a hook to evalute the events after 1.5 secs
					}

					            stream.shutdown(Shutdown::Both).unwrap();
//			return true;
                
}

fn establish_client( mut stream: TcpStream) -> bool
{
    let mut data = [0 as u8; 50]; // using 50 byte buffer

    match stream.read(&mut data)
    {
        Ok(_size) => {

				let rec_signal = str::from_utf8(&data).unwrap().to_string();
//                println!("rec:{}:", rec_signal);
				
				if !is_exit_signal(&mut stream, rec_signal.to_string())
				{

					thread::spawn(move|| {
						resolve_signal(&mut stream, rec_signal.to_string());
                    // connection succeeded
                    handle_client(stream);
                });
                                    return true;

				}
				else
				{
					                    return false;
				}

        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
				return true;
        }
	}
}

fn  handle_client( mut stream: TcpStream) {
//run on background
//since we are using this to determine if shutdown is seen, we check if it is shut down then send others to a background	
//	                thread::spawn(move|| {
  //                  // connection succeeded
    //                handle_client(stream);
      //          });

    let mut data = [0 as u8; 50]; // using 50 byte buffer
    
    while match stream.read(&mut data) {
        Ok(_size) => {
		
				let rec_signal = str::from_utf8(&data).unwrap();

				resolve_signal(&mut stream,rec_signal.to_string());
				true
				
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }


    } 
    {				 }
}

fn time_loop()
{
	    let mut count = 0u32;


    // Infinite loop
    loop {
        count += 1;



        println!("{}", count);



        if count == 50 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
        
        let ten_millis = time::Duration::from_millis(1000);
let _now = Instant::now();

thread::sleep(ten_millis);

    }
}
