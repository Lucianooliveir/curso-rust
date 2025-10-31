#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
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
