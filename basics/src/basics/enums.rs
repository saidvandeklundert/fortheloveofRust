///
/// Does each thing you are modeling have the same methods? Use an Enum
///
/// Do the things you are modeling have some of the same methods, but also
/// some different methods, use structs.
///
///
///
#[derive(Debug)]
enum Media {
    // variants:
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    Audiobook {
        title: String,
    },
    /// Podcast { episode_number: u32}, on the next line, this declaration is shortened
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn descripion(&self) -> String {
        match self {
            // if self is a Book, then give me access to
            // 'title' and 'author':
            Media::Book { title, author } => {
                format!("Book {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast episode {}", id)
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
    fn new() -> Catalog {
        Catalog { items: vec![] }
    }
    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Option::Some(&self.items[index])
            // Some(&self.items[index]) is possible as well
        } else {
            Option::None
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}
pub fn enum_examples() {
    println!("Some examples with Rust Enums");
    let book = Media::Book {
        title: "Dune".to_string(),
        author: "Herbert".to_string(),
    };
    let audio_book = Media::Audiobook {
        title: String::from("Said van de Klundert"),
    };
    print_media(book);

    audio_book.descripion();
    let podcast = Media::Podcast(0909);
    let placeholder = Media::Placeholder;
    let mut catalog = Catalog::new();
    catalog.add(audio_book);
    catalog.add(podcast);
    catalog.add(placeholder);
    println!("{:#?}", catalog);
    // match arm on method that returns the `Option` enum:
    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing here");
        }
    }
    // same, but shorthad:
    match catalog.items.get(010000) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing here");
        }
    }
    if let Some(value) = catalog.get_by_index(133123123) {
        println!("Got an item: {:#?}", value);
    } else {
        println!("Got None",);
    }
    // crashes if the result of the Option enum is not Some:
    let unwrapped_value = catalog.get_by_index(1).unwrap();
    println!("Got an unwrapped_value: {:#?}", unwrapped_value);
    // crashes if the result of the Option enum is not Some but has error message:
    let expected_value = catalog.get_by_index(1).expect("informative error message");
    println!("Got an expected_value: {:#?}", expected_value);
    // unwrap_or allows us to provide placeholders:
    let placeholder = Media::Placeholder;
    let unwrap_or = catalog.get_by_index(12312414).unwrap_or(&placeholder);
    println!("unwrap_or content: {:#?}", unwrap_or);
}
