use std::fs;
use std::io;
use std::process;

fn checkProcessID() {
   let currentProcessID = process::id();
   println!("...current process is: {}", currentProcessID);
   if currentProcessID == 1 {
      println!("rusty is on PID 1!");
   } else {
     println!("rusty is not running on PID 1...... continuing for now....");
   }
}

fn importConfiguration() {
   println!("Do Later");
}

fn importRC() {
   println!("also do later");
}

fn main() {
    println!("...starting rusty");
    checkProcessID();
}
