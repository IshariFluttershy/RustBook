use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let valeurs = vec![
            String::from("Salutations"),
            String::from("a partir"),
            String::from("de la"),
            String::from("nouvelle tache"),
        ];

        for valeur in valeurs {
            tx1.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let valeurs = vec![
            String::from("Encore plus"),
            String::from("de messages"),
            String::from("pour"),
            String::from("vous"),
        ];

        for valeur in valeurs {
            tx.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recu in rx {
        println!("On a recu : {}", recu);
    }
}
