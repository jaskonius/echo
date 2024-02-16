/// Returns all tracks in the library.
///
/// Format: [title, artist, album, duration]
pub fn get_tracks() -> Vec<[String; 4]> {
    vec![
        [
            String::from("Heart of Courage"),
            String::from("Two Steps from Hell"),
            String::from("Invincible"),
            String::from("3:12"),
        ],
        [
            String::from("Time"),
            String::from("Hans Zimmer"),
            String::from("Inception (Original Motion Picture Soundtrack)"),
            String::from("4:35"),
        ],
        [
            String::from("Test Drive"),
            String::from("John Powell"),
            String::from("How to Train Your Dragon (Music from the Motion Picture)"),
            String::from("3:15"),
        ],
        [
            String::from("Victory"),
            String::from("Two Steps from Hell"),
            String::from("Archangel"),
            String::from("5:20"),
        ],
        [
            String::from("Forbidden Friendship"),
            String::from("John Powell"),
            String::from("The Forbidden"),
            String::from("4:25"),
        ],
    ]
}

/// Returns all albums in the library.
///
/// Format: [Artist, Album, Year, Duration]
pub fn get_albums() -> Vec<[String; 4]> {
    vec![
        [
            String::from("Two Steps from Hell"),
            String::from("Dragon"),
            String::from("2019"),
            String::from("01:21:48"),
        ],
        [
            String::from("Two Steps from Hell"),
            String::from("Myth"),
            String::from("2022"),
            String::from("01:26:58"),
        ],
    ]
}

pub fn get_artists() -> Vec<String> {
    todo!()
}
