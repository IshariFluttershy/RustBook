use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur: HashMap<u32, Option<u32>>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32
{
    fn new(calcul: T) -> Cache<T> {
        Cache {
            calcul,
            valeur: HashMap::new(),
        }
    }

    fn valeur(&mut self, arg: u32) -> u32 {
        match self.valeur.get(&arg) {
            Some(v) => v.unwrap(),
            None => {
                let v = (self.calcul)(arg);
                self.valeur.insert(arg, Some(v));
                v
            },
        }
    }
}


fn main() {
    let valeur_utilisateur_simule = 10;
    let nombre_aleatoire_simule = 7;

    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);
}

fn generer_exercices(intensite: u32, nombre_aleatoire: u32) {
    let mut resultat_lent = Cache::new(|nombre| {
        println!("calcul très lent ...");
        thread::sleep(Duration::from_secs(2));
        nombre
    });

    if intensite < 25 {
        println!("Aujourd'hui, faire {} pompes !", resultat_lent.valeur(intensite));
        println!("Ensuite, faire {} abdominaux !", resultat_lent.valeur(intensite));
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
        } else {
            println!(
                "Aujourd'hui, courrez pendant {} minutes !",
                resultat_lent.valeur(intensite)
            );
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Cache;
    #[test]
    fn appel_avec_differentes_valeurs() {
        let mut c = Cache::new(|a| a);

        let v1 = c.valeur(1);
        let v2 = c.valeur(2);
        let v3 = c.valeur(1);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        assert_eq!(v3, 1);
    }
}