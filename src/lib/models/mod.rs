mod narrator_model;
pub use self::narrator_model::{Narrator, NewNarrator, narrator};

mod part_model;
pub use self::part_model::{Part, NewPart, part};

mod series_model;
pub use self::series_model::{Series, NewSeries, series};

mod author_model;
pub use self::author_model::{Author, NewAuthor, author};

mod audiobook_model;
pub use self::audiobook_model::{AudioBook, NewAudioBook, audiobook};

allow_tables_to_appear_in_same_query!(
    audiobook,
    author,
    part,
    series,
    narrator,
);
