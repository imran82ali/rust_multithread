//Here are going to make a multithreaded app
// we are getting system average load and system different temprature on every second interval
  

//for reading system info
use sysinfo::{System,SystemExt};
//for asynchronous channel, returning the sender/receiver halves.
use std::sync::mpsc::channel;
//crate used for multi thread
use std::thread;

//enum for matching requests
enum Requests {
    LoadAvg,
    Temp
}

fn main() {
    //transmission and response channels
    let (req_tx, req_rx) = channel();
    let (resp_tx, resp_rx) = channel();

    // we use "new_all" to ensure that all lists of CPUs and processes are filled!
    let mut sys = System::new_all();
    
    //This call will create first thread 
    let _first = thread::spawn(move|| [

        //iterating requests
        for req in req_rx.iter() {
            sys.refresh_all();

            let data = match req {
                //matching requests
                Requests::LoadAvg => {
                    let load_avg= sys.load_average();
                    format!(
                        "Load at one minute : {}%,  five minute : {}%, fifteen minute : {}%",
                        load_avg.one,
                        load_avg.five,
                        load_avg.fifteen,
                    )

                },
                Requests::Temp => {
                    let mut temp_compo = String::new();
                    for component in sys.components() {
                        temp_compo = temp_compo+ (&format!("Temperature : {component:?}"));
                    }
                    temp_compo
                }
            };
            
            //sending response
            resp_tx.send(data);

        }
    ]);


    //reading response in threads
    let _screen = thread::spawn(move ||[
        for msg in resp_rx.iter(){
            println!("\nFirst Thread {:?}",msg)
        }
    ]);

    //sample second thread
    let _second = thread::spawn(|| [
        loop {
            println!("\n hi second thread thread! \n");
            // to pause the execution of the current thread.
            thread::sleep(std::time::Duration::from_secs(5));
        }
    ]);

    //sending multiple requests
    loop{
        req_tx.send(Requests::LoadAvg);
        req_tx.send(Requests::Temp);
        // to pause the execution of the current thread.
        thread::sleep(std::time::Duration::from_secs(2));
    }

}
