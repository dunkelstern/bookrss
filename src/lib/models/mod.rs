pub mod author;
pub mod speaker;
pub mod part;
pub mod audiobook;
pub mod series;

use self::audiobook::audiobook as audiobook_table;
use self::author::author as author_table;
use self::part::part as part_table;
use self::series::series as series_table;
use self::speaker::speaker as speaker_table;

allow_tables_to_appear_in_same_query!(
    audiobook_table,
    author_table,
    part_table,
    series_table,
    speaker_table,
);
