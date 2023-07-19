use std::io;
use std::io::prelude::*;
mod env;
mod metadata_loader;
use mysql::*;
//use mysql::prelude::Queryable;
use std::fs;


mod metadata;
mod tables_functions;

use metadata::TableDef;
use tables_functions::create_tables;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let env_vars = env::load_env();

    let database_url = env_vars.get("DB_URL")
        .expect("La variable DB_URL n'existe pas.");
    let pool = Pool::new(database_url.as_str()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    loop {
        print!("Choisissez une option :\n1. Créer des tables à partir de metadata.txt\n2. Quitter\nVotre choix : ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let content = fs::read_to_string("metadata.txt")
                    .expect("Le fichier metadata.txt est inexistant");
                let lines = content.split('\n');
                let tables: Vec<TableDef> = lines.map(|line| TableDef::parse(line)).collect();
                create_tables(tables, &mut conn);
                println!("Les tables ont étaient créés");
                break;
            },
            "2" => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
    pause();
}