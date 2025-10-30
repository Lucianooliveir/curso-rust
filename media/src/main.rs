#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Movie { title, director } = self {
        //     format!("Titulo do filme: {}, Diretor: {}", title, director)
        // } else if let Media::Book { title, author } = self {
        //     format!("Titulo do livro: {}, Autor: {}", title, author)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Nome do audiobook: {}", title)
        // } else {
        //     String::from("Mídia não encontrada")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Titulo do livro: {}, Autor: {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Nome do filme: {}, Diretor {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Nome do audiobook: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Numero do episodio do podcast: {}", episode_number)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> PodeTerUmValor {
        if self.items.len() > index {
            PodeTerUmValor::TemUmValor(&self.items[index])
        } else {
            PodeTerUmValor::TemNao
        }
    }
}

enum PodeTerUmValor<'a> {
    TemUmValor(&'a Media),
    TemNao,
}

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    let mut catalog = Catalog::new();
    let audiobook = Media::Audiobook {
        title: String::from("Um audiobook"),
    };
    let movie = Media::Movie {
        title: String::from("Filme bom"),
        director: String::from("Diretor bom"),
    };
    let book = Media::Book {
        title: String::from("Livro ruim"),
        author: String::from("JK Rowling"),
    };
    let podcast = Media::Podcast(3);
    let placeholder = Media::Placeholder;
    println!("{}", audiobook.description());
    println!("{}", movie.description());
    println!("{}", book.description());
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.get_by_index(40) {
        PodeTerUmValor::TemUmValor(valor) => {
            println!("Item: {:#?}", valor);
        }
        PodeTerUmValor::TemNao => {
            println!("Nenhum valor encontrado nesse indice");
        }
    }
}
