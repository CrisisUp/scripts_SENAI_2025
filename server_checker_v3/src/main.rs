use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

use clap::Parser;
use futures::future; // Importa funcionalidades da crate 'futures'

/// Struct para os argumentos (permanece a mesma)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// O caminho para o arquivo de texto contendo a lista de servidores.
    #[arg(short, long)]
    file_path: String,
}

// A anotação #[tokio::main] transforma nossa função 'main' em um ponto de entrada assíncrono.
#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("Lendo servidores do arquivo: {}", args.file_path);

    let servers = match read_servers_from_file(args.file_path) {
        Ok(server_list) => server_list,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo de servidores: {}", e);
            return;
        }
    };
    
    if servers.is_empty() {
        println!("Nenhum servidor encontrado no arquivo.");
        return;
    }

    println!("\nIniciando verificação de {} servidores com async/await...", servers.len());
    check_servers_asynchronously(&servers).await; // Note o '.await' aqui!
    println!("\nVerificação concluída.");
}

// Função de leitura de arquivo (permanece a mesma, não precisa ser async)
fn read_servers_from_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

// Esta função agora é 'async'
async fn check_servers_asynchronously(server_list: &[String]) {
    // Cria uma lista de "tarefas futuras". Cada chamada a check_status agora retorna um "Future".
    let checks = server_list.iter().map(|server| check_status(server));

    // Executa todas as "tarefas futuras" concorrentemente e aguarda a conclusão de todas.
    let results = future::join_all(checks).await;

    // Itera sobre os resultados depois que todos terminaram.
    for status in results {
        match status {
            Ok(success_message) => println!("[ONLINE ✅] {}", success_message),
            Err(error_message) => println!("[OFFLINE ❌] {}", error_message),
        }
    }
}

// Esta função também se torna 'async'
async fn check_status(server_name: &str) -> Result<String, String> {
    // Usa o 'sleep' do Tokio, que não bloqueia a thread do sistema operacional.
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    if server_name.contains("down") {
        Err(format!("Falha ao conectar com {}", server_name))
    } else {
        Ok(format!("{} respondeu com sucesso.", server_name))
    }
}