mod content;
use content::catalog::Catalog;
use content::media::Media;

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
        Some(valor) => {
            println!("Item: {:#?}", valor);
        }
        None => {
            println!("Nenhum valor encontrado nesse indice");
        }
    }
}
