pub fn get_random_word() -> String {
    let response = reqwest::blocking::get(
        "https://random-word-api.herokuapp.com/word"
    )
    .expect("Failed to get word")
    .error_for_status()
    .expect("Request failed");

    let words: Vec<String> = response
        .json()
        .expect("Failed to parse JSON");

    words.into_iter()
        .next()
        .expect("API returned no words")
}
