use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::time::{Duration, Instant};

use clap::Parser;
use futures::future;
use serde::Serialize; // Importa o trait 'Serialize'
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Define os argumentos da nossa CLI v3
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Caminho para o arquivo de texto com a lista de servidores.
    #[arg(short, long)]
    file_path: String,

    /// Porta TCP para verificar a conexão.
    #[arg(short, long, default_value_t = 80)]
    port: u16,

    /// Tempo limite em segundos para cada verificação.
    #[arg(short, long, default_value_t = 2)]
    timeout_secs: u64,
}

/// Enum para representar o status de forma estruturada.
#[derive(Serialize, Debug)]
enum Status {
    Online,
    Offline,
}

/// Struct para guardar o resultado de cada verificação.
/// A anotação #[derive(Serialize)] ensina o compilador a converter esta struct para JSON.
#[derive(Serialize, Debug)]
struct CheckResult {
    server: String,
    status: Status,
    response_time: String,
    details: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let timeout_duration = Duration::from_secs(args.timeout_secs);

    let servers = match read_servers_from_file(&args.file_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Erro fatal ao ler o arquivo: {}", e);
            return;
        }
    };

    println!("Iniciando verificação em {} servidores na porta {} com timeout de {}s...", 
             servers.len(), args.port, args.timeout_secs);
    
    // Executa as verificações e coleta os resultados
    let results = check_servers_asynchronously(servers, args.port, timeout_duration).await;

    // Converte o vetor de resultados para uma string JSON formatada.
    let json_output = serde_json::to_string_pretty(&results).expect("Falha ao serializar resultados para JSON.");

    println!("\n--- Resultado Final (JSON) ---");
    println!("{}", json_output);
}

// A função de leitura permanece a mesma
fn read_servers_from_file<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let reader = BufReader::new(File::open(filename)?);
    reader.lines().collect()
}

// A função de verificação agora retorna um Vetor com os resultados estruturados
async fn check_servers_asynchronously(
    server_list: Vec<String>,
    port: u16,
    timeout_duration: Duration,
) -> Vec<CheckResult> {
    let checks = server_list
        .into_iter()
        .map(|server| check_port_status(server, port, timeout_duration));
    
    future::join_all(checks).await
}

// A função de verificação agora tenta uma conexão TCP real!
async fn check_port_status(server: String, port: u16, timeout_duration: Duration) -> CheckResult {
    let address = format!("{}:{}", server, port);
    let start_time = Instant::now();

    // Tenta conectar, mas com um tempo limite.
    match timeout(timeout_duration, TcpStream::connect(&address)).await {
        // Sucesso dentro do timeout
        Ok(Ok(_)) => {
            let response_time = start_time.elapsed();
            CheckResult {
                server,
                status: Status::Online,
                response_time: format!("{:?}", response_time),
                details: format!("Conexão na porta {} bem-sucedida.", port),
            }
        }
        // Erro na conexão (ex: recusada, host não encontrado)
        Ok(Err(e)) => CheckResult {
            server,
            status: Status::Offline,
            response_time: "N/A".to_string(),
            details: e.to_string(),
        },
        // A operação excedeu o tempo limite
        Err(_) => CheckResult {
            server,
            status: Status::Offline,
            response_time: "N/A".to_string(),
            details: "Timeout atingido.".to_string(),
        },
    }
}