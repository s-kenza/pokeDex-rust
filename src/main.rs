use std::io;
use prettytable::{Table, row};
use rand::Rng;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Feu,
    Eau,
    Plante,
    Electrik,
    Roche,
    Psy,
    Vol,
    Insecte,
    Normal,
    Combat,
    Poison,
    Spectre,
    Dragon,
    Glace,
}

#[derive(Debug, Clone, PartialEq)]
enum Genre {
    Male,
    Femelle,
}

#[derive(Debug, Clone)]
struct Pokemon {
    nom: String,
    niveau: u8,
    type_: Type,
    experience: u32,
    genre: Genre,
}

fn generer_pokemons() -> Vec<Pokemon> {
    vec![
        Pokemon { nom: "Pikachu".to_string(), niveau: 12, type_: Type::Electrik, experience: 230, genre: Genre::Male },
        Pokemon { nom: "Salamèche".to_string(), niveau: 8, type_: Type::Feu, experience: 160, genre: Genre::Femelle },
        Pokemon { nom: "Carapuce".to_string(), niveau: 10, type_: Type::Eau, experience: 180, genre: Genre::Male },
        Pokemon { nom: "Bulbizarre".to_string(), niveau: 9, type_: Type::Plante, experience: 120, genre: Genre::Femelle },
        Pokemon { nom: "Rondoudou".to_string(), niveau: 6, type_: Type::Normal, experience: 80, genre: Genre::Femelle },
        Pokemon { nom: "Nosferapti".to_string(), niveau: 7, type_: Type::Poison, experience: 90, genre: Genre::Male },
        Pokemon { nom: "Caninos".to_string(), niveau: 11, type_: Type::Feu, experience: 210, genre: Genre::Male },
        Pokemon { nom: "Magicarpe".to_string(), niveau: 5, type_: Type::Eau, experience: 60, genre: Genre::Femelle },
        Pokemon { nom: "Racaillou".to_string(), niveau: 13, type_: Type::Roche, experience: 270, genre: Genre::Male },
        Pokemon { nom: "Psykokwak".to_string(), niveau: 9, type_: Type::Psy, experience: 150, genre: Genre::Femelle },
        Pokemon { nom: "Roucool".to_string(), niveau: 8, type_: Type::Vol, experience: 130, genre: Genre::Male },
        Pokemon { nom: "Chenipan".to_string(), niveau: 3, type_: Type::Insecte, experience: 20, genre: Genre::Femelle },
        Pokemon { nom: "Machoc".to_string(), niveau: 14, type_: Type::Combat, experience: 290, genre: Genre::Male },
        Pokemon { nom: "Spectrum".to_string(), niveau: 16, type_: Type::Spectre, experience: 320, genre: Genre::Femelle },
        Pokemon { nom: "Dracolosse".to_string(), niveau: 45, type_: Type::Dragon, experience: 800, genre: Genre::Male },
        Pokemon { nom: "Lokhlass".to_string(), niveau: 30, type_: Type::Glace, experience: 600, genre: Genre::Femelle },
        Pokemon { nom: "Évoli".to_string(), niveau: 6, type_: Type::Normal, experience: 75, genre: Genre::Femelle },
        Pokemon { nom: "Miaouss".to_string(), niveau: 7, type_: Type::Normal, experience: 85, genre: Genre::Male },
        Pokemon { nom: "Voltorbe".to_string(), niveau: 9, type_: Type::Electrik, experience: 110, genre: Genre::Femelle },
        Pokemon { nom: "Akwakwak".to_string(), niveau: 18, type_: Type::Eau, experience: 350, genre: Genre::Male },
    ]
}

fn train_pokemon(pokemons: &mut Vec<Pokemon>, xp: u32, nom_fichier: &str) {
    pokemons.iter_mut().for_each(|p| {
        p.experience += xp;
        if p.experience >= 100 {
            p.niveau += 1;
            p.experience = 0;
        }
    });
    println!("Tous les Pokemon ont été entrainés avec {} XP.", xp);
    
    // Sauvegarde automatique après l'entraînement
    if let Err(e) = sauvegarder_pokemons(pokemons, nom_fichier) {
        println!("Erreur lors de la sauvegarde après entraînement: {}", e);
    }
}

fn display_pokemon(pokemons: &Vec<Pokemon>) {
    let mut table = Table::new();
    table.add_row(row!["Nom", "Niveau", "Type", "Experience", "Genre"]);

    for p in pokemons.iter() {
        table.add_row(row![
            p.nom.clone(),
            p.niveau.to_string(),
            format!("{:?}", p.type_),
            p.experience.to_string(),
            format!("{:?}", p.genre),
        ]);
    }

    table.printstd();
}

fn is_reproduction(pokemon1: &Pokemon, pokemon2: &Pokemon) -> bool {
    if pokemon1.type_ == pokemon2.type_ && pokemon1.genre != pokemon2.genre && pokemon1.niveau >= 10 && pokemon2.niveau >= 10 {
        println!("Reproduction possible entre {} et {}", pokemon1.nom, pokemon2.nom);
        return true;
    }
    false
}

fn reproduce_pokemon(pokemon1: &Pokemon, pokemon2: &Pokemon, liste: &mut Vec<Pokemon>, nom_fichier: &str) {
    let reproductible = is_reproduction(pokemon1, pokemon2);

    if reproductible {
        let mut enfant = pokemon1.clone();
        let mut rng = rand::rng();
        let gender = if rng.random_bool(0.5) {
            Genre::Femelle
        } else {
            Genre::Male
        };

        enfant.nom = "Mystère".to_string(); // Nom temporaire pour l'enfant
        enfant.type_ = pokemon1.type_.clone(); // Type de l'enfant hérité du parent
        enfant.niveau = 1;
        enfant.experience = 0;
        enfant.genre = gender;
        println!("Un nouveau Pokémon est né: {:?}", enfant);
        liste.push(enfant);
        
        // Sauvegarde automatique après la reproduction
        if let Err(e) = sauvegarder_pokemons(liste, nom_fichier) {
            println!("Erreur lors de la sauvegarde après reproduction: {}", e);
        }
    } else {
        println!("Reproduction impossible entre {} et {}", pokemon1.nom, pokemon2.nom);
        println!("\nAssurez-vous que les deux Pokémon sont de types compatibles et de genres opposés.");
        println!("Niveau minimum requis pour la reproduction: 10");
        println!("Niveau de {}: {}, Niveau de {}: {}", pokemon1.nom, pokemon1.niveau, pokemon2.nom, pokemon2.niveau);
        println!("Genres: {} et {}", format!("{:?}", pokemon1.genre), format!("{:?}", pokemon2.genre));
        println!("Types: {:?} et {:?}", pokemon1.type_, pokemon2.type_);
        println!("Veuillez essayer avec d'autres Pokémon.");
    }
}

fn ajouter_pokemon(pokemons: &mut Vec<Pokemon>, nom_fichier: &str) {
    let mut nom = String::new();
    let mut niveau = String::new();
    let mut type_str = String::new();
    let mut experience = String::new();
    let mut genre_str = String::new();

    println!("Nom du Pokémon : ");
    io::stdin().read_line(&mut nom).unwrap();

    println!("Niveau (nombre) : ");
    io::stdin().read_line(&mut niveau).unwrap();

    println!("Type (Feu, Eau, Plante, Electrik, Roche, Psy, Vol, Insecte, Normal, Combat, Poison, Spectre, Dragon, Glace) : ");
    io::stdin().read_line(&mut type_str).unwrap();

    println!("Experience (nombre) : ");
    io::stdin().read_line(&mut experience).unwrap();

    println!("Genre (Male ou Femelle) : ");
    io::stdin().read_line(&mut genre_str).unwrap();

    let type_ = match type_str.trim() {
        "Feu" => Type::Feu,
        "Eau" => Type::Eau,
        "Plante" => Type::Plante,
        "Electrik" => Type::Electrik,
        "Roche" => Type::Roche,
        "Psy" => Type::Psy,
        "Vol" => Type::Vol,
        "Insecte" => Type::Insecte,
        "Normal" => Type::Normal,
        "Combat" => Type::Combat,
        "Poison" => Type::Poison,
        "Spectre" => Type::Spectre,
        "Dragon" => Type::Dragon,
        "Glace" => Type::Glace,
        _ => {
            println!("Type inconnu. Abandon.");
            return;
        }
    };

    let genre = match genre_str.trim() {
        "Male" => Genre::Male,
        "Femelle" => Genre::Femelle,
        _ => {
            println!("Genre invalide.");
            return;
        }
    };

    let pokemon = Pokemon {
        nom: nom.trim().to_string(),
        niveau: niveau.trim().parse().unwrap_or(1),
        type_,
        experience: experience.trim().parse().unwrap_or(0),
        genre,
    };

    pokemons.push(pokemon);
    println!("Nouveau Pokémon ajouté !");
    
    // Sauvegarde automatique après l'ajout
    if let Err(e) = sauvegarder_pokemons(pokemons, nom_fichier) {
        println!("Erreur lors de la sauvegarde après ajout: {}", e);
    }
}

fn sort_pokemons(pokemons: &mut Vec<Pokemon>, critere: &str, nom_fichier: &str) {
    match critere {
        "niveau" => pokemons.sort_by(|a, b| a.niveau.cmp(&b.niveau)),
        "type" => pokemons.sort_by(|a, b| format!("{:?}", a.type_).cmp(&format!("{:?}", b.type_))),
        _ => println!("Critère de tri inconnu."),
    }
    display_pokemon(pokemons);
    
    println!("\nPokémons triés par {}", critere);
    
    // Sauvegarde automatique après le tri
    if let Err(e) = sauvegarder_pokemons(pokemons, nom_fichier) {
        println!("Erreur lors de la sauvegarde après tri: {}", e);
    }
}

fn sauvegarder_pokemons(pokemons: &Vec<Pokemon>, nom_fichier: &str) -> io::Result<()> {
    let mut fichier = File::create(nom_fichier)?;
    
    for pokemon in pokemons {
        let genre_str = match pokemon.genre {
            Genre::Male => "Male",
            Genre::Femelle => "Femelle",
        };
        
        let type_str = match pokemon.type_ {
            Type::Feu => "Feu",
            Type::Eau => "Eau",
            Type::Plante => "Plante",
            Type::Electrik => "Electrik",
            Type::Roche => "Roche",
            Type::Psy => "Psy",
            Type::Vol => "Vol",
            Type::Insecte => "Insecte",
            Type::Normal => "Normal",
            Type::Combat => "Combat",
            Type::Poison => "Poison",
            Type::Spectre => "Spectre",
            Type::Dragon => "Dragon",
            Type::Glace => "Glace",
        };
        
        let ligne = format!("{}|{}|{}|{}|{}\n", 
            pokemon.nom,
            pokemon.niveau,
            type_str,
            pokemon.experience,
            genre_str
        );
        
        fichier.write_all(ligne.as_bytes())?;
    }
    
    Ok(())
}

fn charger_pokemons(nom_fichier: &str) -> io::Result<Vec<Pokemon>> {
    let mut pokemons = Vec::new();
    
    // Vérifier si le fichier existe
    if !Path::new(nom_fichier).exists() {
        println!("Fichier {} non trouvé. Génération d'une nouvelle liste de Pokémon.", nom_fichier);
        return Ok(generer_pokemons());
    }
    
    let fichier = File::open(nom_fichier)?;
    let reader = BufReader::new(fichier);
    
    for ligne in reader.lines() {
        let ligne = ligne?;
        let parties: Vec<&str> = ligne.split('|').collect();
        
        if parties.len() != 5 {
            println!("Format incorrect: {}", ligne);
            continue;
        }
        
        let nom = parties[0].to_string();
        let niveau = parties[1].parse().unwrap_or(1);
        
        let type_ = match parties[2] {
            "Feu" => Type::Feu,
            "Eau" => Type::Eau,
            "Plante" => Type::Plante,
            "Electrik" => Type::Electrik,
            "Roche" => Type::Roche,
            "Psy" => Type::Psy,
            "Vol" => Type::Vol,
            "Insecte" => Type::Insecte,
            "Normal" => Type::Normal,
            "Combat" => Type::Combat,
            "Poison" => Type::Poison,
            "Spectre" => Type::Spectre,
            "Dragon" => Type::Dragon,
            "Glace" => Type::Glace,
            _ => {
                println!("Type inconnu dans le fichier: {}", parties[2]);
                Type::Normal // Type par défaut
            }
        };
        
        let experience = parties[3].parse().unwrap_or(0);
        
        let genre = match parties[4] {
            "Male" => Genre::Male,
            "Femelle" => Genre::Femelle,
            _ => {
                println!("Genre invalide dans le fichier: {}", parties[4]);
                Genre::Male // Genre par défaut
            }
        };
        
        pokemons.push(Pokemon {
            nom,
            niveau,
            type_,
            experience,
            genre,
        });
    }
    
    println!("{} Pokémons chargés depuis {}", pokemons.len(), nom_fichier);
    Ok(pokemons)
}

fn generer_nom_pokemon() -> String {
    let mut rng = rand::rng();
    
    let prefixes = vec!["Pika", "Sala", "Bulbi", "Cara", "Dra", "Noc", "Fero", "Magi", "Ger", "Psy"];
    let suffixes = vec!["chu", "monde", "zarre", "puce", "feu", "tali", "claw", "carpe", "bi", "duck"];
    
    let prefix = prefixes[rng.random_range(0..prefixes.len())];
    let suffix = suffixes[rng.random_range(0..suffixes.len())];
    
    format!("{}{}", prefix, suffix)
}

fn generer_genre_pokemon() -> Genre {
    let mut rng = rand::rng();
    if rng.random_bool(0.5) {
        Genre::Male
    } else {
        Genre::Femelle
    }
}

fn main() {
    let nom_fichier = "pokemons.txt";
    
    // Charger les Pokémon depuis le fichier ou générer une nouvelle liste
    let mut pokemons = match charger_pokemons(nom_fichier) {
        Ok(p) => p,
        Err(e) => {
            println!("Erreur lors du chargement: {}. Génération d'une nouvelle liste.", e);
            generer_pokemons()
        }
    };

    // Si le fichier n'existait pas, le créer maintenant avec la liste générée
    if !Path::new(nom_fichier).exists() {
        if let Err(e) = sauvegarder_pokemons(&pokemons, nom_fichier) {
            println!("Erreur lors de la création du fichier initial: {}", e);
        } else {
            println!("Nouveau fichier de sauvegarde créé: {}", nom_fichier);
        }
    }

    loop {
        // Affichage du menu
        println!("******** Gestion de l'élevage Pokémon ********");
        println!("1. Ajouter un Pokemon");
        println!("2. Afficher tous les Pokemon");
        println!("3. Entrainer tous les Pokemon (gain d'XP)");
        println!("4. Tenter une reproduction entre deux Pokemon");
        println!("5. Générer aléatoirement le genre ou le nom du nouveau Pokemon");
        println!("6. Trier les Pokemon par niveau ou type");
        println!("7. Quitter");
        println!("Choisissez une option (1 à 7) :");

        // Lire le choix de l'utilisateur
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        // Enlever les espaces / retour à la ligne
        let choix = choix.trim();

        // Vérifier quel choix a été fait
        match choix {
            "1" => {
                ajouter_pokemon(&mut pokemons, nom_fichier);
            },
            "2" => {
                display_pokemon(&pokemons);
            },
            "3" => {
                train_pokemon(&mut pokemons, 20, nom_fichier); // Gain d'XP de 20 pour chaque Pokemon
            },
            "4" => {
                display_pokemon(&pokemons);

                println!("Sélectionnez deux Pokemon pour la reproduction :");

                println!("Pokemon 1 (nom) : ");
                let mut nom1 = String::new();
                io::stdin().read_line(&mut nom1).expect("Erreur de lecture");

                println!("Pokemon 2 (nom) : ");
                let mut nom2 = String::new();
                io::stdin().read_line(&mut nom2).expect("Erreur de lecture");

                if let Some(pokemon1) = pokemons.iter().find(|p| p.nom == nom1.trim()).cloned() {
                    if let Some(pokemon2) = pokemons.iter().find(|p| p.nom == nom2.trim()).cloned() {
                        reproduce_pokemon(&pokemon1, &pokemon2, &mut pokemons, nom_fichier);
                    } else {
                        println!("Pokemon {} non trouvé", nom2.trim());
                    }
                } else {
                    println!("Pokemon {} non trouvé", nom1.trim());
                }
            },
            "5" => {
                println!("Que souhaitez-vous générer ? (1: Nom, 2: Genre, 3: Les deux)");
                let mut option = String::new();
                io::stdin().read_line(&mut option).expect("Erreur de lecture");
                let option = option.trim();
                
                match option {
                    "1" => {
                        let nom = generer_nom_pokemon();
                        println!("Nom généré: {}", nom);
                    },
                    "2" => {
                        let genre = generer_genre_pokemon();
                        println!("Genre généré: {:?}", genre);
                    },
                    "3" => {
                        let nom = generer_nom_pokemon();
                        let genre = generer_genre_pokemon();
                        println!("Nom généré: {}, Genre généré: {:?}", nom, genre);
                        
                        let mut type_str = String::new();
                        println!("Type du Pokemon (Feu, Eau, Plante, Electrik, Roche, Psy, Vol, Insecte, Normal, Combat, Poison, Spectre, Dragon, Glace) : ");
                        io::stdin().read_line(&mut type_str).expect("Erreur de lecture");
                        
                        // Convertir la saisie en minuscules pour ignorer la casse
                        let type_ = match type_str.trim().to_lowercase().as_str() {
                            "feu" => Type::Feu,
                            "eau" => Type::Eau,
                            "plante" => Type::Plante,
                            "electrik" => Type::Electrik,
                            "roche" => Type::Roche,
                            "psy" => Type::Psy,
                            "vol" => Type::Vol,
                            "insecte" => Type::Insecte,
                            "normal" => Type::Normal,
                            "combat" => Type::Combat,
                            "poison" => Type::Poison,
                            "spectre" => Type::Spectre,
                            "dragon" => Type::Dragon,
                            "glace" => Type::Glace,
                            _ => {
                                println!("Type inconnu, utilisation du type Normal par défaut.");
                                Type::Normal
                            }
                        };
                        
                        let pokemon = Pokemon {
                            nom,
                            niveau: 1,
                            type_,
                            experience: 0,
                            genre,
                        };
                        println!("Nouveau Pokemon généré et ajouté à votre collection: {:?}", pokemon);
                        pokemons.push(pokemon);

                        // Sauvegarde automatique après l'ajout du Pokémon généré
                        if let Err(e) = sauvegarder_pokemons(&pokemons, nom_fichier) {
                            println!("Erreur lors de la sauvegarde après génération: {}", e);
                        }
                        
                    },
                    _ => println!("Option invalide."),
                }
            },
            "6" => {
                println!("Choisissez un critère de tri (niveau ou type) : ");
                let mut critere = String::new();
                io::stdin().read_line(&mut critere).expect("Erreur de lecture");
                let critere = critere.trim();
                sort_pokemons(&mut pokemons, critere, nom_fichier);
            },
            "7" => {
                println!("Au revoir!");
                break;
            },
            _ => {
                println!("Choix invalide. Veuillez réessayer.");
            }
        }

        println!(""); // Saut de ligne pour aérer
    }
}