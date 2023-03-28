enum Dow {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Sarurday,
    Sunday,
}

impl Dow {
    fn get_kanji_strokes(&self) -> isize {
        match self {
            Dow::Monday => 4,
            Dow::Tuesday => 4,
            Dow::Wednesday => 4,
            Dow::Thursday => 4,
            Dow::Friday => 8,
            Dow::Sarurday => 3,
            Dow::Sunday => 4,
        }
    }
}

fn main() {
    unimplemented!();
}

# [test]
fn test_get_kanji_strokes() {
    assert_eq!(Dow::Monday.get_kanji_strokes(), 4);
    assert_eq!(Dow::Tuesday.get_kanji_strokes(), 4);
    assert_eq!(Dow::Wednesday.get_kanji_strokes(), 4);
    assert_eq!(Dow::Thursday.get_kanji_strokes(), 4);
    assert_eq!(Dow::Friday.get_kanji_strokes(), 8);
    assert_eq!(Dow::Sarurday.get_kanji_strokes(), 3);
    assert_eq!(Dow::Sunday.get_kanji_strokes(), 4);
}
