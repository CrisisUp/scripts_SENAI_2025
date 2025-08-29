# Verificador de Servidores Assíncrono em Rust (v3)

Este é um projeto de linha de comando (CLI) de alta performance, desenvolvido em Rust, para verificar o status de uma lista de servidores através de conexões TCP em uma porta específica. O projeto foi evoluído para utilizar programação assíncrona, tornando-o uma ferramenta de diagnóstico de rede eficiente e prática.

## ✨ Funcionalidades

* **Verificação de Rede Real:** Tenta estabelecer uma conexão TCP real em uma porta configurável para determinar se um serviço está online.
* **Altamente Concorrente:** Utiliza o runtime assíncrono **Tokio** e o padrão `async/await` para verificar centenas de servidores em paralelo com um número mínimo de threads do sistema.
* **Saída Estruturada em JSON:** Coleta os resultados de todas as verificações e os exibe em um formato JSON limpo e legível por máquina, ideal para integração com outros scripts e ferramentas de automação.
* **Interface de Linha de Comando Configurável:** Utiliza a biblioteca (`crate`) **`clap`** para permitir que o usuário especifique o arquivo de entrada, a porta a ser verificada e o tempo limite (`timeout`) via argumentos.
* **Tratamento de Erros Detalhado:** Captura e reporta diferentes tipos de falhas de rede, como falhas de resolução de DNS, conexões recusadas e timeouts.

## 🚀 Conceitos de Rust Demonstrados

Este projeto serve como um exemplo prático dos seguintes conceitos avançados do Rust:

1.  **Programação Assíncrona:** Uso intensivo de `async/await` e do runtime `tokio` para operações de I/O de rede não-bloqueantes.
2.  **Modelagem de Dados e Serialização:** Uso de `structs` e `enums` para modelar os dados de resultado e a biblioteca `serde` para serializar esses dados para o formato JSON.
3.  **Gerenciamento de Erros em I/O:** Tratamento de `io::Result` e outros tipos de erro provenientes de operações de rede e de arquivo.
4.  **Ecossistema e Ferramentas (Cargo):** Gerenciamento de dependências externas (`crates`) como `clap`, `tokio`, `futures` e `serde`.
5.  **Controle de Tempo (`tokio::time`):** Implementação de timeouts para operações de rede, uma prática essencial para ferramentas robustas.

## 🛠️ Pré-requisitos

Para compilar e executar este projeto, você precisará ter o seguinte ambiente configurado:

1.  **Rust Toolchain:** Instalado via `rustup` (inclui `rustc` e `cargo`).
2.  **Microsoft C++ Build Tools:** Necessário para a etapa de "linkagem" no Windows.

## ⚙️ Como Compilar e Executar

1.  **Clone o repositório (se estiver no GitHub):**
    ```bash
    git clone <url-do-seu-repositorio>
    cd nome-da-pasta-do-projeto
    ```

2.  **Crie um arquivo de servidores:**
    Crie um arquivo de texto (ex: `servidores.txt`) na raiz do projeto e adicione os nomes dos servidores, um por linha.
    ```
    google.com
    github.com
    um-servidor-que-nao-existe.com
    ```

3.  **Compile para produção (otimizado):**
    Para criar um executável otimizado, use o comando `build --release`.
    ```bash
    cargo build --release
    ```

4.  **Execute a ferramenta:**
    O executável estará em `target/release/`. Você pode então executá-lo diretamente, passando os argumentos necessários.
    ```bash
    .\target\release\server_checker.exe --file-path servidores.txt --port 443 --timeout-secs 1
    ```

## 📋 Exemplo de Uso e Saída

### Comando de Exemplo

Verifica a porta 443 (HTTPS) com um timeout de 1 segundo para cada servidor listado em `servidores.txt`.

```bash
cargo run --release -- --file-path servidores.txt --port 443 --timeout-secs 1