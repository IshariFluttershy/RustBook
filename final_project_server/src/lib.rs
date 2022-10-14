use std::thread;

pub struct GroupeTaches {
    taches: Vec<thread::JoinHandle<()>>,
}

pub enum ErreurGroupeTache {
    SizeIsZero,
    Other,
}

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

        let mut taches = Vec::with_capacity(size);

        for _ in 0..size {
            taches.push(thread::spawn(|| { }));
        }

        GroupeTaches { taches }
    }

    pub fn executer<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        
    }
}