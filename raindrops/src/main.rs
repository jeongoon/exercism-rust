pub fn raindrops_1(n: u32) -> String {
    let divisible_by = |k| -> bool { n % k == 0 };

    let sound = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .map(|(k, sound)| if divisible_by(k as u32) { sound } else { "" })
        .join("");

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}

pub fn raindrops_1_1(n: u32) -> String {
    let divisible_by = |k| -> bool { n % k == 0 };

    let sound = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|(k, sound)| {
            if divisible_by(*k as u32) {
                Some(sound.to_string())
            } else {
                None
            }
        })
        .collect::<String>();

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}

struct FacToSound(u32, &'static str);

impl FacToSound {
    pub fn get_factor(&self) -> u32 {
        self.0
    }

    pub fn get_sound(&self) -> &str {
        self.1
    }
}

const RAIN_DROP_SOUND_MAP: [FacToSound; 3] = [
    FacToSound(3, "Pling"),
    FacToSound(5, "Plang"),
    FacToSound(7, "Plong"),
];

pub fn raindrops_2(n: u32) -> String {
    let divisible_by = |k| -> bool { n % k == 0 };
    let fold_helper = |acc: Option<String>, fac_sound: &FacToSound| {
        if divisible_by(fac_sound.get_factor()) {
            let new_sound = fac_sound.get_sound();
            acc.map(|s| s + new_sound)
                .or_else(|| Some(String::from(new_sound)))
        } else {
            acc
        }
    };

    RAIN_DROP_SOUND_MAP
        .iter()
        .fold(None, fold_helper)
        .unwrap_or_else(|| n.to_string())
}

pub fn raindrops(n: u32) -> String {
    let divisible_by = |k| -> bool { n % k == 0 };
    let fold_helper = |acc: Option<String>, fac_sound: &FacToSound| {
        if divisible_by(fac_sound.get_factor()) {
            let new_sound = fac_sound.get_sound();
            Some(match acc {
                Some(sound_so_far) => sound_so_far + new_sound,
                None => String::from(new_sound),
            })
        } else {
            acc
        }
    };

    RAIN_DROP_SOUND_MAP
        .iter()
        .fold(None, fold_helper)
        .unwrap_or_else(|| n.to_string())
}

fn main() {
    println!("{}", raindrops_1_1(50));
}
