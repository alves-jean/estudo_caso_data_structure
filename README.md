# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

# Descrição:
Sistema de recomendação de produtos que utiliza grafos. A ideia é que os produtos sejam os nós do grafo e as recomendações sejam as arestas. Com entrada de dados feita de forma manual e de quantidade ilimitada, sendo que a escalabilidade é interrompida quando o usuário insere o termo de parada.

# Objetivos do Sistema:
1- Gerenciamento de produtos e recomendações: Adição de produtos e recomendações, sendo armazenado as relações entre eles;
2- Buscar recomendações: De acordo com o produto pesquisado, é informado quais recomendações estão associadas a ele;
3- Sistema interativo: Fornecimento ilimitado de dados, sendo o fornecimento interrompido com a digitação de "Sair".

# Funcionalidade:
1- Adição de Produtos e Recomendações:
- O usuário pode adicionar produtos e recomendações inerentes a este produto, sendo armazenados em grafos representados como HashMap.

2- Consulta de recomendações:
- Após adição de produtos e recomendações, o usuário pode consultar as recomendações de um dado produto. A pesquisa é feita no grafo, retornando as recomendações associadas ao produto.

3- Entrada de dados de forma interativa:
- O sistema permite entrada ilimitada de dados, tendo assim escalabilidade, e para interrupção da entrada de dados basta inserir a informação "Sair"

4- Exibição de Recomendações cadastradas:
- Ao final da inserção de dados, sendo feita a interrupção com o código "Sair", o sistema exibe os produtos e recomendações cadastradas.

# Tecnologias utilizadas:
1- Linguagem utilizada:
- A linguagem rust foi a única utilizada focando na segurança, desempenho e concorrência. A rust é conhecida pelo seu gerenciamento de memória sem necessidade de coletor de lixo.

2- Crates utilizadas:
- HashMap: Utilizado por sua eficência em aramazenar a relação entre produtos e recomendações, auxiliando em buscas rápidas e adição dinâmica de novas reocmendações;
- IO: Utilizado para entrada manual de dados, através de interação do usuário com o terminal.

3- Ferramentas de teste:
- Cargo Test: O "Cargo" é uma ferramenta de construção do Rust, na qual possui sistemas de testes embutidos. O "Cargo test" permite escrever testes unitários, integração e teste de aceitação no próprio código;
- Assert: Valida as condições durante o desenvolvimento para garantir que o código está funcionando como o esperado.

# Instruções:
1- Teste
cd testes
cargo test

2- Principal
cd ..
cd estudo_caso
cargo run
inserir produto
inserir recomendação
se quiser sair da inserção de dados, digite "Sair"". Caso não, insira produto e recomendação.
Inserindo o código: "Sair"
Aparecerá os produtos e recomedações adicionados
E será solicitado a pesquisa de algum produto
Digite o produto que queira pesquisar.

# Arquitetura
A arquitetura projetada de maneira simples e eficiente, utiliza o Rust para gerenciar a interaçã entre os produtos e suas recomendações.
1- Estrutura principal (Graph):
- A estrutura principal desta arquitetura é o "Graph" que é responsável por representar o grafo que armazena a relação entre recomendações e produtos.
- Os métodos:
    - new() que inicializa um HashMap, estrutura esta responsável por mapear cada produto a uma lista de recomendações;
    - add_edge(produto, recomendacao): ao adicionar uma nova recomendação no grafo, associando um produto a outro através da recomendação;
    - get_recommendations(produto): Retorna as recomendações associadas a um produto em especifico.

2- Módulo de entrada de dados (Função main):
- Responsável por gerenciar a interação com o usuário e o controle de fluxo de dados.
- Componentes:
    - Entrada de dados: Utiliza a biblioteca "std::io" para capturar a entrada de dados via terminal;
    - Controle de fluxo: Utiliza um loop que permite a entrada de dados de forma ilimitada até que o usuário digite "sair";
    - Consulta de recomendações: O usuário tem permissão para consulta de recomendação de um produto especifico.

# Fluxo
1- Inicialização:
    - O programa inicia com a criação de um novo grafo, onde as recomendações serão armazenadas.

2- Entrada de dados:
    - O sistema solicita ao usuário a entrada de um produto e a recomendação relacionada a ele;
    - Os dados vão sendo inseridos até que o usuário digite "sair";
    - Através do método "add_edge()" os produtos e suas recomendações são inseridas no grafo.

3- Exibição de recomendação:
    - Após a entrada dos dados, o sistema exibe todos os produtos e recomendações;
    - É permitido ao usuário a pesquisa de um produto especifico e suas recomendações.

4- Finalização:
    - O sistema termina com a exibição de todas as recomendações cadastradas.

# Algoritmos e Estruturas de dados utilizados
1- Estrutura de dados - HashMap: 
- O principal componenetes de dados responsável pela implementação de uma tabela de dispersão (hash table) que armazera par de chave e valor. Ele aramzena a relação entre produtos e recomendações.

2- Algoritmos de inserção de dados - add_edge:
- O método add_edge é responsável por inserir um novo produto e sua recomendação no grafo representado pelo HashMap. Este método recebe dois parâmetros, sendo um com o produto e o outro com sua recomendação, ao inserir a chave do produto é verificado a existência do mesmo no grafo, caso exista a recomendação é adicionada ao vetor correspondente ao produto, já se não existir o produto na lista é feita a criação de uma nova chave.

3- Algoritmo de busca de recomendação - get_recommendations:
- Este método é responsável por fazer a pesquisa das recomendações associadas a um produto especifico, ele utiliza o HashMap para buscar rapidamente as recomedações, ao passar a chave produto para o método get do HashMap, que procura encontrar as recomendações associadas a aquele produto.

# Desempenho e Escalabilidade

1- Estrutura de dados e Complexibilidade:
- A escolha de HashMap<String, Vec<String>> para armazenamento implica em certas complexibilidades computacionais:
    - Adição de recomendações (add_edge):
        - Complexibilidade O(1) para inserção no HashMap;
        - A inserção em um vetor já existente Vec<String> também possui complexibilidade O(1);
        - No pior caso a complexibilidade pode ser O(n) caso seja necesário alocação dinâmica do vetor.
    - Busca de recomendações (get_recommendations):
        - A busca é eficiente por apresentar complexibilidade constando com O(1);
        - Já a iteração sobre o vetor de recomendações pode apresentar complexibilidade O(m), onde m representa o número de recomendações.

2- Uso de memória:
- O uso de uma HashMap<String, Vec<String>> cada produto possui uma chave alocada em uma tabela hash e um vetor de recomendações associado e quantidade de memória utilizada cresce proporcionalmente ao número de produtos e recomendações inseridas.

3- Escalabilidade:
- Quando a entrada de dados aumenta é necessário a reestruturação da tabela interna do HashMap, porém isto tem o custo de O(n).

