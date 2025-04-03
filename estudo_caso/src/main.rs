use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Graph {
    listas_adjacentes: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            listas_adjacentes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, produto: &str, recomendacao: &str) {
        self.listas_adjacentes
            .entry(produto.to_string())
            .or_insert_with(Vec::new)
            .push(recomendacao.to_string());
    }

    fn get_recommendations(&self, produto: &str) -> Option<&Vec<String>> {
        self.listas_adjacentes.get(produto)
    }
}

fn main() {
    let mut produto_grafo = Graph::new();
    let mut entrada = String::new();
    let mut running = true;

    println!("Bem-vindo ao Sistema de Recomendação de Produtos utilizando Grafos!");
    println!("Digite 'sair' a qualquer momento para finalizar.\n");

    // Entrada manual e ilimitada de dados
    while running {
        println!("Digite o nome do produto:");
        entrada.clear();
        io::stdin().read_line(&mut entrada).unwrap();
        let produto = entrada.trim().to_string();

        if produto.eq_ignore_ascii_case("sair") {
            running = false;
            break;
        }

        println!("Digite uma recomendação para '{}':", produto);
        entrada.clear();
        io::stdin().read_line(&mut entrada).unwrap();
        let recomendacao = entrada.trim().to_string();

        if recomendacao.eq_ignore_ascii_case("sair") {
            running = false;
            break;
        }

        produto_grafo.add_edge(&produto, &recomendacao);
        println!("Adicionado: '{}' -> '{}'\n", produto, recomendacao);
    }

    // Exibição das recomendações
    println!("\nProdutos e suas recomendações cadastradas:");
    for (produto, recomendacoes) in &produto_grafo.listas_adjacentes {
        println!("Produto: {}", produto);
        for rec in recomendacoes {
            println!("- {}", rec);
        }
    }

    // Testando busca de recomendações
    println!("\nDigite o nome de um produto para visualizar suas recomendações:");
    entrada.clear();
    io::stdin().read_line(&mut entrada).unwrap();
    let product_to_check = entrada.trim().to_string();

    println!("Recomendações para '{}':", product_to_check);
    if let Some(recomendacoes) = produto_grafo.get_recommendations(&product_to_check) {
        for rec in recomendacoes {
            println!("- {}", rec);
        }
    } else {
        println!("Nenhuma recomendação encontrada.");
    }
}
