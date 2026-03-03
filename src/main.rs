use std::fmt::format;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use colored::Colorize;



struct Passanger {
    passanger_id: u32,
    name: String,
    destination: String,
}

struct FlightPlan {
    destination: String,
    gate_number: u32,
}

struct Baggage {
    baggage_id: u32,
    destination: String,

}

struct Terminal {
    terminal_id: u32,
    destination: String,
}


fn create_passanger () -> Passanger {
    let mut rng = rand::thread_rng();
    let destination =  vec!["Denmark", "France", "Iceland", "Türkiye"];
    let first_name = vec!["Thomas", "Mikkel", "Kim", "Lars", "Per", "Jacob", "Emil", "Peter", "Niels"];
    let last_name = vec!["Jensen", "Nielsen", "Hansen", "Pedersen", "Andersen", "Christensen", "Larsen", "Sørensen", "Rasmussen"];
    let first = first_name[rng.gen_range(0..first_name.len())].to_string();
    let last = last_name[rng.gen_range(0..last_name.len())].to_string();
    let passanger = Passanger {
        passanger_id: rng.gen_range(0..=10000),
        name: format!("{} {}", first, last),
        destination: destination[rng.gen_range(0..destination.len())].to_string(),
    };

    thread::sleep(Duration::from_secs(1));
    let output = format!(
        "{} has arrived, they are traveling to {} | ID: {}",
        passanger.name.yellow(),
        passanger.destination.cyan(),
        passanger.passanger_id.to_string().green(),
    );

    println!("{}", output.blue().bold().underline());

    passanger

}

fn create_counter(i: u32, rx: Arc<Mutex<mpsc::Receiver<Passanger>>>) {
    //giver vores skrank et navn
    let counter_name = format!("Counter {}", i);

    //laver vores tråd
    thread::Builder::new()
        .name(counter_name)
        .spawn(move || {
            //who even knows
            let mut rng = rand::thread_rng();
            loop {
                //henter passanger ned med recieve
                let passanger = match rx.lock().unwrap().recv() {
                    Ok(p) => p,
                    //breaker, hvis der sker en fejl
                    Err(_) => break,
                };
                //sætter thread navn ind i my_name
                let my_name = thread::current().name().unwrap().to_string();
                // println!("Passanger {} has arrived at {}", passanger.name.yellow(), my_name);
                let output2 = format!(
                    "Passanger {} has arrived at {}", passanger.name.yellow(), my_name.bold().underline(),
                );
                println!("{}", output2.blue());

                //opretter baggage ud fra hentet passanger
                let baggage = Baggage {
                    baggage_id: rng.gen_range(1000..9999),
                    //får passanger destionation
                    destination: passanger.destination.clone(),
                };
                //sover 1 sec inden næste output
                thread::sleep(Duration::from_secs(1));
                // println!(
                //     "Passanger {} with Id: {} has checked in baggage: {} with destination: {}",
                //     passanger.name.yellow(), passanger.passanger_id.to_string().green(), baggage.baggage_id, baggage.destination
                // );
                let output3 = format!(
                    "Passanger {} with Id: {} has checked in baggage: {} with destination: {}",
                    passanger.name.yellow(), passanger.passanger_id.to_string().green(), baggage.baggage_id.to_string().green(), baggage.destination.cyan(),
                );
                println!("{}", output3.blue());

                //tråden randmly sover mellem 3-6s før skranken bliver ledig igen
                let sleep_time = Duration::from_millis(rng.gen_range(3000..=6000));
                thread::sleep(sleep_time);
                let output2 = format!(
                    "--- {} er nu ledig igen ---", my_name
                );
                println!("{}", output2.bright_white().on_green());
            }
        })
        .unwrap();
}

fn create_terminal (id: u32, destination: String, rx: mpsc::Receiver<Baggage> ) {
    let terminal_name = format!("Terminal {}", id);

    println!("{} created with destination {}", terminal_name, destination);

    thread::Builder::new()
        .name(terminal_name)
        .spawn(move || {
            let terminal = Terminal {
                terminal_id: id,
                destination: destination.clone(),
            };

            loop {
                let baggage = match rx.recv() {
                    Ok(b) => b,
                    Err(_) => break,
                };

                println!("{}", format!(
                    "Terminal {} to {} recived baggage number {}",
                    terminal.terminal_id,
                    terminal.destination,
                    baggage.baggage_id,
                ));
            }
    })
    .unwrap();
}


fn main() {
    let (tx, rx) = mpsc::channel::<Passanger>();
    let rx = Arc::new(Mutex::new(rx));

    let destinations = vec![
        (1, "Denmark"),
        (2, "France"),
        (3, "Iceland"),
        (4, "Türkiye"),
    ];

    //opretter 3 skranke
    for i in 1..=3 {
        create_counter(i, Arc::clone(&rx));
    }

    for (id, dest) in destinations {
        let (tx, rx) = mpsc::channel::<Baggage>();
        create_terminal(id, dest.to_string(), rx);
    }

    //opretter x passangere
    for _ in 0..5 {
        let new_passanger = create_passanger();
        tx.send(new_passanger).unwrap();
    }
    //giver tid til at skranke kører passangere igennem
    thread::sleep(Duration::from_secs(20));

}




