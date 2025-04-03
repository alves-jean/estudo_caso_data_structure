use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut grafo = Graph::new();
        grafo.add_edge("Notebook", "Mouse");
        grafo.add_edge("Notebook", "Teclado");
        grafo.add_edge("Mouse", "Mousepad");

        assert!(grafo.listas_adjacentes.contains_key("Notebook"));
        assert_eq!(grafo.listas_adjacentes["Notebook"], vec!["Mouse", "Teclado"]);
        assert_eq!(grafo.listas_adjacentes["Mouse"], vec!["Mousepad"]);
    }

    #[test]
    fn test_get_recommendations_existing() {
        let mut grafo = Graph::new();
        grafo.add_edge("Notebook", "Mouse");
        grafo.add_edge("Notebook", "Teclado");

        let recomendacoes = graph.get_recommendations("Notebook");
        assert!(recomendacoes.is_some());
        assert_eq!(recomendacoes.unwrap().len(), 2);
        assert!(recomendacoes.unwrap().contains(&"Mouse".to_string()));
        assert!(recomendacoes.unwrap().contains(&"Teclado".to_string()));
    }

    #[test]
    fn test_get_recommendations_nonexistent() {
        let grafo = Graph::new();
        let recomendacoes = grafo.get_recommendations("Smartphone");
        assert!(recomendacoes.is_none());
    }

    #[test]
    fn test_manual_input_simulation() {
        let mut grafo = Graph::new();
        let entradas = vec![
            ("Notebook", "Teclado"),
            ("Notebook", "Mouse"),
            ("Mouse", "Mousepad"),
        ];

        for (produto, recomendacao) in entradas {
            grafo.add_edge(produto, recomendacao);
        }

        assert_eq!(grafo.listas_adjacentes["Notebook"], vec!["Teclado", "Mouse"]);
        assert_eq!(grafo.listas_adjacentes["Mouse"], vec!["Mousepad"]);
    }
}
