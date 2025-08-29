use std::thread;
use std::time::Duration;

// Função principal, onde o programa começa.
fn main() {
    // 1. O 'main' é o DONO (owner) deste vetor de Strings.
    let servers = vec![
        String::from("server-alpha.com"),
        String::from("server-beta.com"),
        String::from("server-down.com"), // Este vai falhar de propósito.
        String::from("server-gamma.com"),
    ];

    println!("Iniciando verificação de servidores em paralelo...");

    // Chamamos a função para verificar os servidores, "emprestando" os dados a ela.
    check_servers_concurrently(&servers);

    // Mesmo depois da função ter sido chamada, 'servers' ainda é válido aqui.
    // O 'main' nunca perdeu a posse dos dados.
    println!("\nVerificação concluída. O vetor 'servers' ainda existe e tem {} itens.", servers.len());
}

// Função que verifica uma lista de servidores em paralelo.
// Ela "pega emprestado" (borrows) a lista de servidores, indicada pelo '&'.
fn check_servers_concurrently(server_list: &[String]) {
    // 3. CRIA UM ESCOPO SEGURO PARA THREADS.
    // O Rust garante que as threads não viverão mais que os dados que elas estão usando ('server_list').
    thread::scope(|s| {
        for server in server_list {
            // Cria e inicia uma nova thread para cada servidor.
            // O 'move' aqui move a propriedade de 'server' para dentro da thread.
            s.spawn(move || {
                let status = check_status(server); // Verifica o status do servidor.
                
                // 2. USA 'MATCH' PARA TRATAR O RESULTADO DE FORMA SEGURA.
                // O compilador nos força a lidar com ambos os casos: sucesso (Ok) e erro (Err).
                match status {
                    Ok(success_message) => println!("[ONLINE ✅] {}", success_message),
                    Err(error_message) => println!("[OFFLINE ❌] {}", error_message),
                }
            });
        }
    });
}

// Função que simula a verificação de status de um único servidor.
// Ela retorna um 'Result', que pode ser um sucesso (Ok) ou um erro (Err).
fn check_status(server_name: &str) -> Result<String, String> {
    // Simula um tempo de resposta da rede.
    thread::sleep(Duration::from_millis(500));

    if server_name.contains("down") {
        // Se o nome contém "down", simulamos um erro.
        Err(format!("Falha ao conectar com {}", server_name))
    } else {
        // Se não, simulamos um sucesso.
        Ok(format!("{} respondeu com sucesso.", server_name))
    }
}