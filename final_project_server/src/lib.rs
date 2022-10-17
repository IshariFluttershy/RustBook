use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct GroupeTaches {
    operateurs: Vec<Operateur>,
    envoi: mpsc::Sender<Message>,
}

pub enum Message {
    NouvelleMission(Mission),
    Extinction,
}

pub enum ErreurGroupeTache {
    SizeIsZero,
    Other,
}

pub struct Operateur {
    id: usize,
    tache: Option<thread::JoinHandle<()>>,
}

type Mission = Box<dyn FnOnce() + Send + 'static>;

impl GroupeTaches {
    /// Crée un nouveau GroupeTaches.
    ///
    /// La taille est le nom de tâches présentes dans le groupe.
    ///
    /// # Panics
    ///
    /// La fonction `new` devrait paniquer si la taille vaut zéro.
     
    /*pub fn new(size: usize) -> Result<GroupeTaches, ErreurGroupeTache> {
        if size <= 0 {
            Err(ErreurGroupeTache::SizeIsZero)
        } else {
            Ok(GroupeTaches)
        }

    }*/
    pub fn new(size: usize) -> GroupeTaches{
        assert!(size > 0);

        let (envoi, reception) = mpsc::channel();

        let reception = Arc::new(Mutex::new(reception));
        let mut operateurs = Vec::with_capacity(size);

        for id in 0..size {
            operateurs.push(Operateur::new(id, Arc::clone(&reception)));
        }

        GroupeTaches { operateurs, envoi }
    }

    pub fn executer<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let mission = Box::new(f);
        self.envoi.send(Message::NouvelleMission(mission)).unwrap();
    }
}

impl Drop for GroupeTaches {
    fn drop(&mut self) {
        println!("envoi du message d'extinction a tous les operateurs");

        for _ in &mut self.operateurs {
            self.envoi.send(Message::Extinction).unwrap();
        }

        println!("Arret de tous les operateurs.");

        for operateur in &mut self.operateurs {
            println!("Arret de l'opérateur {}", operateur.id);

            if let Some(tache) = operateur.tache.take() {
                tache.join().unwrap();
            }
        }
    }
}

impl Operateur {
    pub fn new(id: usize, reception: Arc<Mutex<mpsc::Receiver<Message>>>) -> Operateur{
        Operateur {
            id: id,
            tache: Some(thread::spawn(move || loop { 
                let message = reception.lock().unwrap().recv().unwrap();

                match message {
                    Message::NouvelleMission(mission) => {
                    println!("L'opérateur {} a obtenu une mission ; il l'exécute.", id);

                    mission();
                    }
                    Message::Extinction => {
                        println!("L'opérateur {} a recu instruction d'arret", id);

                        break;
                    }
                }

            })),
        }
    }
}