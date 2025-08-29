# Verificador de Servidores em Rust (Server Checker)

Este é um projeto de linha de comando (CLI) desenvolvido em Rust para verificar o status de uma lista de servidores de forma concorrente e assíncrona. O projeto foi criado como um exercício prático para explorar os principais conceitos e pontos fortes da linguagem Rust.

## ✨ Funcionalidades

* **Leitura de Arquivo:** Lê uma lista de nomes de servidores a partir de um arquivo de texto.
* **Verificação Concorrente:** Utiliza o runtime assíncrono **Tokio** e o padrão `async/await` para verificar múltiplos servidores em paralelo, de forma altamente eficiente e sem bloquear threads do sistema.
* **Interface de Linha de Comando:** Utiliza a biblioteca (`crate`) **`clap`** para um parsing robusto de argumentos de linha de comando.
* **Tratamento de Erros:** Emprega o sistema de `Result` e `match` do Rust para lidar com falhas de forma explícita e segura (ex: arquivo não encontrado, servidor offline).

## 🚀 Conceitos de Rust Demonstrados

Este projeto serve como um exemplo prático dos seguintes "superpoderes" do Rust:

1.  **Segurança de Memória:** Através do sistema de *Ownership* (Dono) e *Borrowing* (Empréstimo), garantindo que não haja vazamentos de memória ou acessos indevidos.
2.  **Concorrência sem Medo:** Uso de `async/await` com Tokio para executar tarefas de rede em paralelo com a garantia do compilador contra *data races*.
3.  **Abstrações de Custo Zero:** Uso de funcionalidades de alto nível (como `iterators` e `futures`) que são compiladas para um código de máquina extremamente performático.
4.  **Ecossistema e Ferramentas (Cargo):** Demonstração do uso do `Cargo` para gerenciar dependências de bibliotecas externas (`crates`) como `clap`, `tokio` e `futures`.

## 🛠️ Pré-requisitos

Para compilar и executar este projeto, você precisará ter o seguinte ambiente configurado:

1.  **Rust Toolchain:** Instalado via `rustup` (inclui `rustc` e `cargo`).
2.  **Microsoft C++ Build Tools:** Necessário para a etapa de "linkagem" no Windows (conforme as notas de instalação do `rustup`).

## ⚙️ Como Compilar e Executar

1.  **Clone o repositório (se estiver no GitHub):**
    ```bash
    git clone <url-do-seu-repositorio>
    cd server_checker_v3
    ```

2.  **Crie um arquivo de servidores:**
    Crie um arquivo de texto (ex: `servidores.txt`) na raiz do projeto e adicione os nomes dos servidores, um por linha.
    ```
    server-alpha.com
    server-beta.com
    server-down.com
    server-gamma.com
    server-delta.com
    ```

3.  **Execute em modo de desenvolvimento:**
    Use o `cargo run`. Note o `--` para separar os argumentos do Cargo dos argumentos do nosso programa.
    ```bash
    cargo run -- --file-path servers.txt
    ```

4.  **Compile para produção (otimizado):**
    Para criar um executável otimizado, use o comando `build --release`.
    ```bash
    cargo build --release
    ```
    O executável estará em `target/release/server_checker.exe`. Você pode então executá-lo diretamente:
    ```bash
    .\target\release\server_checker.exe --file-path servidores.txt
    ```