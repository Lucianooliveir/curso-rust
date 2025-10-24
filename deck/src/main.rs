use rand::{prelude::ThreadRng, rng, seq::SliceRandom};

/*
    Define atributos (mais algumas instruções) para a struct Deck
    derive fala que queremos adicionar mais algumas funções para essa struct, no caso a Debug
*/
#[derive(Debug)]
struct Deck {
    cartas: Vec<String>,
}

// Ingerent Implementation = adicionando função para uma struct
impl Deck {
    // Associated function = função ligada a definição da struct, é chamada com ::, ex: Deck::new()
    fn new() -> Self {
        let naipes: [&'static str; 4] = ["Espadas", "Paus", "Copas", "Ouros"];
        let valores: [&'static str; 13] = [
            "Ás", "Dois", "Três", "Quatro", "Cinco", "Seis", "Sete", "Oito", "Nove", "Dez",
            "Valete", "Dama", "Rei",
        ];
        // precisa ser mutavel já que não é possivel adicionar no vetor imutavel
        let mut cartas: Vec<String> = vec![];

        for naipe in naipes {
            for valor in valores {
                let carta: String = format!("{} de {}", valor, naipe);
                cartas.push(carta);
            }
        }
        // struct literal: criando uma instancia de uma struct (Deck) e inicializando cards com um vetor vazio
        // vec![] == Vec::new()
        Deck { cartas } //retorno implicito, tirar return e ponto e virgula
    }

    // Metodos = Quando vamos usar/mudar algum campo no struct
    fn shuffle(&mut self) {
        let mut rng: ThreadRng = rng();
        self.cartas.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cartas: usize) -> Vec<String> {
        self.cartas.split_off(self.cartas.len() - num_cartas)
    }
}

fn main() {
    let mut deck: Deck = Deck::new();

    deck.shuffle();

    let mao = deck.deal(3);

    println!("Aqui esta sua mão: {:#?}", mao);
    println!("Aqui esta o baralho: {:#?}", deck);
}
