mod blog;

use crate::blog::Billet;

fn main() {
    let mut billet = Billet::new();

    billet.ajouter_texte("J'ai mangé une salade au déjeuner aujourd'hui");
    assert_eq!("", billet.contenu());
    assert_eq!("Brouillon", billet.etat());

    billet.demander_relecture();
    assert_eq!("", billet.contenu());
    assert_eq!("EnRelecture", billet.etat());
    
    billet.rejeter();
    billet.ajouter_texte("J'ai mangé des lentilles au diner aujourd'hui");
    assert_eq!("", billet.contenu());
    assert_eq!("Brouillon", billet.etat());
    
    billet.demander_relecture();
    billet.ajouter_texte("J'ai mangé une soupe au souper aujourd'hui");
    assert_eq!("", billet.contenu());
    assert_eq!("EnRelecture", billet.etat());

    billet.approuver();
    assert_eq!("", billet.contenu());
    assert_eq!("EnRelecture", billet.etat());

    billet.approuver();
    assert_eq!("J'ai mangé des lentilles au diner aujourd'hui", billet.contenu());
    assert_eq!("Publier", billet.etat());
}