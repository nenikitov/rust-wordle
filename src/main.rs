mod words;


fn main() {
    let words = words::read_from("res/words_alpha.txt");
    let words = words::filter(words);
    words::write_into("res/words_filtered.txt", &words);
}
