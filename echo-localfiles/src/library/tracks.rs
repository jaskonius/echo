/// Returns all tracks in the library.
///
/// Format: [title, artist, album, duration]
pub fn get_tracks() -> Vec<Vec<String>> {
    vec![
        vec![
            String::from("Heart of Courage"),
            String::from("Two Steps from Hell"),
            String::from("Invincible"),
            String::from("3:12"),
        ],
        vec![
            String::from("Time"),
            String::from("Hans Zimmer"),
            String::from("Inception (Original Motion Picture Soundtrack)"),
            String::from("4:35"),
        ],
        vec![
            String::from("Test Drive"),
            String::from("John Powell"),
            String::from("How to Train Your Dragon (Music from the Motion Picture)"),
            String::from("3:15"),
        ],
        vec![
            String::from("Victory"),
            String::from("Two Steps from Hell"),
            String::from("Archangel"),
            String::from("5:20"),
        ],
        vec![
            String::from("Forbidden Friendship"),
            String::from("John Powell"),
            String::from("The Forbidden"),
            String::from("4:25"),
        ],
    ]
}
