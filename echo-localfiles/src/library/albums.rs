/// Returns all albums in the library.
///
/// Format: [Artist, Album, Year, Duration]
pub fn get_albums() -> Vec<Vec<String>> {
    vec![
        vec![
            String::from("Two Steps from Hell"),
            String::from("Dragon"),
            String::from("2019"),
            String::from("01:21:48"),
        ],
        vec![
            String::from("Two Steps from Hell"),
            String::from("Myth"),
            String::from("2022"),
            String::from("01:26:58"),
        ],
    ]
}
