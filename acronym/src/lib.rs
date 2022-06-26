pub fn abbreviate(phrase: &str) -> String {
    let cons_as_uppercase = |(mut chars, some_char): (Vec<char>, Option<char>)| {
        if let Some(ch) = some_char {
            chars.insert(ch.to_ascii_uppercase());
        }
        chars
    };

    cons_as_uppercase(
        phrase
            .chars()
            .rev()
            .fold((vec![], None), |(mut abbr, mb_candi), c| {
                if c.is_ascii_alphabetic() {
                    if !c.is_ascii_uppercase()
                        || mb_candi.map_or(true, |candi| candi.is_ascii_uppercase())
                    {
                        // ^ new character is lowercase
                        //   OR new character is uppercase
                        //      and current candidate is None
                        //          or current candidate is uppercase as well
                        (abbr, Some(c))
                        // ^ change to new candidate only
                    } else {
                        // note: c is already in upper case
                        (
                            {
                                abbr.push(c);
                                abbr
                            },
                            None,
                        )
                    }
                } else {
                    if c == '\'' {
                        (abbr, mb_candi)
                    } else {
                        (cons_as_uppercase((abbr, mb_candi)), None)
                    }
                }
            }),
    )
    .into_iter()
    .rev()
    .collect::<String>()
}
