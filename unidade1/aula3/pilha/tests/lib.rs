// Definição da estrutura de Pilha (Stack)
pub struct Stack<T> {
    // Vetor que armazena os elementos da pilha
    elementos: Vec<T>,
    // Capacidade máxima da pilha (opcional)
    capacidade_maxima: Option<usize>,
}

// Implementação dos métodos para a estrutura Stack
impl<T> Stack<T> {
    // Método para criar uma nova pilha sem limite de capacidade
    pub fn nova() -> Self {
        Stack {
            elementos: Vec::new(),
            capacidade_maxima: None,
        }
    }

    // Método para criar uma nova pilha com capacidade máxima
    pub fn nova_com_capacidade(capacidade: usize) -> Self {
        Stack {
            elementos: Vec::with_capacity(capacidade),
            capacidade_maxima: Some(capacidade),
        }
    }

    // Método para empilhar um elemento (push)
    pub fn empilhar(&mut self, elemento: T) -> Result<(), &'static str> {
        // Verifica se a pilha atingiu sua capacidade máxima
        if let Some(capacidade) = self.capacidade_maxima {
            if self.elementos.len() >= capacidade {
                return Err("Erro: A pilha está cheia");
            }
        }

        // Adiciona o elemento à pilha
        self.elementos.push(elemento);
        Ok(())
    }

    // Método para desempilhar um elemento (pop)
    pub fn desempilhar(&mut self) -> Option<T> {
        self.elementos.pop()
    }

    // Método para visualizar o elemento do topo sem removê-lo (peek)
    pub fn topo(&self) -> Option<&T> {
        self.elementos.last()
    }

    // Método para verificar se a pilha está vazia
    pub fn esta_vazia(&self) -> bool {
        self.elementos.is_empty()
    }

    // Método para verificar se a pilha está cheia
    pub fn esta_cheia(&self) -> bool {
        if let Some(capacidade) = self.capacidade_maxima {
            return self.elementos.len() >= capacidade;
        }
        false
    }

    // Método para obter o tamanho atual da pilha
    pub fn tamanho(&self) -> usize {
        self.elementos.len()
    }

    // Método para limpar a pilha (remover todos os elementos)
    pub fn limpar(&mut self) {
        self.elementos.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nova_pilha_esta_vazia() {
        let pilha: Stack<i32> = Stack::nova();
        assert!(pilha.esta_vazia());
    }

    #[test]
    fn nova_pilha_com_capacidade() {
        let pilha: Stack<i32> = Stack::nova_com_capacidade(5);
        assert!(pilha.esta_vazia());
        assert_eq!(pilha.tamanho(), 0);
    }

    #[test]
    fn empilhar_aumenta_tamanho() {
        let mut pilha = Stack::nova();
        pilha.empilhar(42).unwrap();
        assert_eq!(pilha.tamanho(), 1);
        assert!(!pilha.esta_vazia());
    }

    #[test]
    fn topo_retorna_ultimo_elemento() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1).unwrap();
        pilha.empilhar(2).unwrap();
        pilha.empilhar(3).unwrap();
        assert_eq!(*pilha.topo().unwrap(), 3);
        assert_eq!(pilha.tamanho(), 3); // Verificar que o topo não remove o elemento
    }

    #[test]
    fn desempilhar_retorna_ultimo_elemento() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1).unwrap();
        pilha.empilhar(2).unwrap();
        let valor = pilha.desempilhar();
        assert_eq!(valor, Some(2));
        assert_eq!(pilha.tamanho(), 1);
    }

    #[test]
    fn desempilhar_pilha_vazia_retorna_none() {
        let mut pilha: Stack<i32> = Stack::nova();
        let valor = pilha.desempilhar();
        assert_eq!(valor, None);
    }

    #[test]
    fn topo_pilha_vazia_retorna_none() {
        let pilha: Stack<i32> = Stack::nova();
        let valor = pilha.topo();
        assert_eq!(valor, None);
    }

    #[test]
    fn pilha_limitada_rejeita_elementos_alem_da_capacidade() {
        let mut pilha = Stack::nova_com_capacidade(2);
        assert!(pilha.empilhar(1).is_ok());
        assert!(pilha.empilhar(2).is_ok());
        assert!(pilha.empilhar(3).is_err()); // Deve falhar
        assert_eq!(pilha.tamanho(), 2);
    }

    #[test]
    fn pilha_ilimitada_aceita_muitos_elementos() {
        let mut pilha = Stack::nova();
        for i in 0..1000 {
            assert!(pilha.empilhar(i).is_ok());
        }
        assert_eq!(pilha.tamanho(), 1000);
    }

    #[test]
    fn limpar_remove_todos_elementos() {
        let mut pilha = Stack::nova();
        pilha.empilhar(1).unwrap();
        pilha.empilhar(2).unwrap();
        pilha.empilhar(3).unwrap();
        pilha.limpar();
        assert!(pilha.esta_vazia());
        assert_eq!(pilha.tamanho(), 0);
    }

    #[test]
    fn esta_cheia_retorna_correto() {
        let mut pilha = Stack::nova_com_capacidade(2);
        assert!(!pilha.esta_cheia());
        pilha.empilhar(1).unwrap();
        assert!(!pilha.esta_cheia());
        pilha.empilhar(2).unwrap();
        assert!(pilha.esta_cheia());
    }

    #[test]
    fn pilha_ilimitada_nunca_esta_cheia() {
        let mut pilha = Stack::nova();
        for i in 0..100 {
            pilha.empilhar(i).unwrap();
            assert!(!pilha.esta_cheia());
        }
    }
}