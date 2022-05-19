pub fn build_proverb_1(list: &[&str]) -> String {
    match list.len() {
        0 => "".to_owned(),
        _ => {
            let mut story = list
                .windows(2)
                .map(|x| format!("For want of a {} the {} was lost.", x[0], x[1],))
                .collect::<Vec<String>>();
            story.push(format!("And all for the want of a {}.", list[0]));
            story.join("\n")
        }
    }
}

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::from(""),
        Some(wanted) => list
            .windows(2)
            .map(|x| format!("For want of a {} the {} was lost.", x[0], x[1],))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                wanted
            )))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

fn main() {
    let a = vec!["nail", "shoe", "horse"];
    println!("{}", build_proverb(&a));
}
