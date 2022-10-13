pub struct Billet {
    etat: Option<Box<dyn Etat>>,
    contenu: String,
}

impl Billet {
    pub fn new() -> Billet {
        Billet {
            etat: Some(Box::new(Brouillon {})),
            contenu: String::new(),
        }
    }

    pub fn ajouter_texte(&mut self, text : &str) {
        self.contenu = String::from(self.etat.as_ref().unwrap().ajouter_texte(self, text));
    }

    pub fn contenu(&self) -> &str {
        self.etat.as_ref().unwrap().contenu(self)
    }

    pub fn demander_relecture(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.demander_relecture());
        }
    }

    pub fn approuver(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.approuver());
        }
    }

    pub fn rejeter(&mut self) {
        if let Some(s) = self.etat.take() {
            self.etat = Some(s.rejeter());
        }
    }

    pub fn etat(&self) -> &str{
        self.etat.as_ref().unwrap().etat()
    }
}

trait Etat {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat>;
    fn approuver(self: Box<Self>) -> Box<dyn Etat>;
    fn rejeter(self: Box<Self>) -> Box<dyn Etat>;
    fn contenu<'a>(&self, _billet: &'a Billet) -> &'a str {
        ""
    }
    fn etat(&self) -> &str;
    fn ajouter_texte<'a>(&self, billet: &'a Billet, _text: &'a str) -> &'a str {
        &billet.contenu
    }
}

struct Brouillon {}
impl Etat for Brouillon {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(EnRelecture { readed_once: false})
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn rejeter(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn etat(&self) -> &str {
        "Brouillon"
    }

    fn ajouter_texte<'a>(&self, _billet: &'a Billet, text: &'a str) -> &'a str {
        text
    }
}

struct EnRelecture {
    readed_once: bool,
}
impl Etat for EnRelecture {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        if self.readed_once == false {
            Box::new(EnRelecture { readed_once: true})
        }
        else {
            Box::new(Publier {})
        }
    } 

    fn rejeter(self: Box<Self>) -> Box<dyn Etat> {
        Box::new(Brouillon {})
    }

    fn etat(&self) -> &str {
        "EnRelecture"
    }
}

struct Publier {}
impl Etat for Publier {
    fn demander_relecture(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn approuver(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn rejeter(self: Box<Self>) -> Box<dyn Etat> {
        self
    }

    fn contenu<'a>(&self, billet: &'a Billet) -> &'a str {
        &billet.contenu
    }

    fn etat(&self) -> &str {
        "Publier"
    }
}
