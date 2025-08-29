use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::thread;
use std::time::Duration;

use clap::Parser; // Importa o parser de argumentos

/// Struct que define os argumentos de linha de comando esperados.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// O caminho para o arquivo de texto contendo a lista de servidores.
    #[arg(short, long)]
    file_path: String,
}

// Função principal, agora um pouco mais inteligente.
fn main() {
    let args = Args::parse(); // Lê os argumentos da linha de comando.

    println!("Lendo servidores do arquivo: {}", args.file_path);

    // Lê os servidores do arquivo e lida com um possível erro.
    let servers = match read_servers_from_file(args.file_path) {
        Ok(server_list) => server_list,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo de servidores: {}", e);
            return; // Encerra o programa se não conseguir ler o arquivo.
        }
    };
    
    if servers.is_empty() {
        println!("Nenhum servidor encontrado no arquivo.");
        return;
    }

    println!("\nIniciando verificação de {} servidores em paralelo...", servers.len());
    check_servers_concurrently(&servers);
    println!("\nVerificação concluída.");
}

// NOVA FUNÇÃO: Lê um arquivo linha por linha e retorna um vetor de Strings.
fn read_servers_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    // Coleta cada linha do arquivo em um vetor, tratando possíveis erros.
    reader.lines().collect()
}


// A função de verificação concorrente permanece a mesma.
fn check_servers_concurrently(server_list: &[String]) {
    thread::scope(|s| {
        for server in server_list {
            s.spawn(move || {
                let status = check_status(server);
                match status {
                    Ok(success_message) => println!("[ONLINE ✅] {}", success_message),
                    Err(error_message) => println!("[OFFLINE ❌] {}", error_message),
                }
            });
        }
    });
}

// A função de verificação de status também permanece a mesma.
fn check_status(server_name: &str) -> Result<String, String> {
    thread::sleep(Duration::from_millis(500));
    if server_name.contains("down") {
        Err(format!("Falha ao conectar com {}", server_name))
    } else {
        Ok(format!("{} respondeu com sucesso.", server_name))
    }
}