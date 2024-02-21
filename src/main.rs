use std::error::Error;
use std::env;
use csv::Reader;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use enigo::*;

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut path: String = String::from(env::current_dir()?.to_string_lossy());
    path = path.replace("/src","/Assets/words.csv");
    let mut rdr = Reader::from_path(path)?;
    let mut temp: String;
    let mut v: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        for field in &record.get(1){
            let str_field: String = field.to_string();
            temp = str_field.trim().to_string();
            v.push(temp);
        }
    }

    println!("Carregamento concluído!");
    seed(&v);
    Ok(())
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro: nenhum valor digitado.");
    let cleaned_input = input.trim().to_string();
    cleaned_input
}

fn seed(v: &[String]) {
    let enigo = Enigo::new();
    let mut x: i32 = 0;
    let mut words: i32;
    let mut result: i64;
    let mut rng;
    let mut last_cursor_location: (i32,i32);
    let mut cursor_location: (i32,i32);
    
    words = 0;
    println!();
    while words == 0 {
        println!("Digite [1] para gerar uma seed frase de 24 palavras");
        println!("Digite [2] para gerar uma seed frase de 12 palavras");
        let input = read_string();
    
        if input == "1" {
            words = 24;
            println!();
            println!("Opção [1] seed frase de 24 palavras selecionada!");
            println!();
        } else if input == "2" {
            words = 12;
            println!();
            println!("Opção [2] seed frase de 12 palavras selecionada!");
            println!();
        } else {
            println!();
            println!("Opção inválida: digite 1 ou 2");
            println!();
        }
    }

    println!("Gerando sua frase seed ...");
    println!();
    println!("**Atenção: movimente seu mouse durante o processo para gerar aleatoriedade para a sua seed**");
    println!();
    print!("Frase seed: ");

    loop {
        x+=1;
        cursor_location = enigo.mouse_location();
        result = i64::from(cursor_location.0) * i64::from(cursor_location.1);

        if x > (11*words){
            break;
        }
        
        if x%11==0{
            rng = ChaCha8Rng::seed_from_u64(result as u64);
            print!("{} ", v[rng.gen_range(0..2047)]);
        }
        
        loop {
            last_cursor_location = enigo.mouse_location();
            if last_cursor_location != cursor_location{
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!();
    println!("Iniciando programa:");
    println!("Carregando dados...");
    let _ = read_csv();
    println!();
    println!();
    println!("Processo concluído!");
    println!();
    Ok(())
}