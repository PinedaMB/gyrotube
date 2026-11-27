use crate::models::SyncedLyricLine;

const HANGUL_INITIALS: [&str; 19] = [
    "g", "kk", "n", "d", "tt", "r", "m", "b", "pp", "s", "ss", "", "j", "jj", "ch", "k", "t", "p",
    "h",
];

const HANGUL_VOWELS: [&str; 21] = [
    "a", "ae", "ya", "yae", "eo", "e", "yeo", "ye", "o", "wa", "wae", "oe", "yo", "u", "wo", "we",
    "wi", "yu", "eu", "ui", "i",
];

const HANGUL_FINALS: [&str; 28] = [
    "", "k", "k", "k", "n", "n", "n", "t", "l", "k", "m", "p", "l", "t", "p", "l", "m", "p", "p",
    "t", "t", "ng", "t", "t", "k", "t", "p", "t",
];

fn is_non_latin_romanizable(ch: char) -> bool {
    let cp = ch as u32;
    (0xAC00..=0xD7A3).contains(&cp) // Hangul Syllables
        || (0x3040..=0x309F).contains(&cp) // Hiragana
        || (0x30A0..=0x30FF).contains(&cp) // Katakana
        || (0x0400..=0x04FF).contains(&cp) // Cyrillic
        || (0x0370..=0x03FF).contains(&cp) // Greek
}

fn to_hiragana(ch: char) -> char {
    let cp = ch as u32;
    if (0x30A1..=0x30F6).contains(&cp) {
        char::from_u32(cp - 0x60).unwrap_or(ch)
    } else {
        ch
    }
}

fn kana_digraph(a: char, b: char) -> Option<&'static str> {
    match (a, b) {
        ('き', 'ゃ') => Some("kya"),
        ('き', 'ゅ') => Some("kyu"),
        ('き', 'ょ') => Some("kyo"),
        ('し', 'ゃ') => Some("sha"),
        ('し', 'ゅ') => Some("shu"),
        ('し', 'ょ') => Some("sho"),
        ('ち', 'ゃ') => Some("cha"),
        ('ち', 'ゅ') => Some("chu"),
        ('ち', 'ょ') => Some("cho"),
        ('に', 'ゃ') => Some("nya"),
        ('に', 'ゅ') => Some("nyu"),
        ('に', 'ょ') => Some("nyo"),
        ('ひ', 'ゃ') => Some("hya"),
        ('ひ', 'ゅ') => Some("hyu"),
        ('ひ', 'ょ') => Some("hyo"),
        ('み', 'ゃ') => Some("mya"),
        ('み', 'ゅ') => Some("myu"),
        ('み', 'ょ') => Some("myo"),
        ('り', 'ゃ') => Some("rya"),
        ('り', 'ゅ') => Some("ryu"),
        ('り', 'ょ') => Some("ryo"),
        ('ぎ', 'ゃ') => Some("gya"),
        ('ぎ', 'ゅ') => Some("gyu"),
        ('ぎ', 'ょ') => Some("gyo"),
        ('じ', 'ゃ') => Some("ja"),
        ('じ', 'ゅ') => Some("ju"),
        ('じ', 'ょ') => Some("jo"),
        ('び', 'ゃ') => Some("bya"),
        ('び', 'ゅ') => Some("byu"),
        ('び', 'ょ') => Some("byo"),
        ('ぴ', 'ゃ') => Some("pya"),
        ('ぴ', 'ゅ') => Some("pyu"),
        ('ぴ', 'ょ') => Some("pyo"),
        ('ふ', 'ぁ') => Some("fa"),
        ('ふ', 'ぃ') => Some("fi"),
        ('ふ', 'ぇ') => Some("fe"),
        ('ふ', 'ぉ') => Some("fo"),
        ('て', 'ぃ') => Some("ti"),
        ('で', 'ぃ') => Some("di"),
        _ => None,
    }
}

fn kana_single(ch: char) -> Option<&'static str> {
    match ch {
        'あ' | 'ぁ' => Some("a"),
        'い' | 'ぃ' => Some("i"),
        'う' | 'ぅ' => Some("u"),
        'え' | 'ぇ' => Some("e"),
        'お' | 'ぉ' => Some("o"),
        'か' => Some("ka"),
        'き' => Some("ki"),
        'く' => Some("ku"),
        'け' => Some("ke"),
        'こ' => Some("ko"),
        'さ' => Some("sa"),
        'し' => Some("shi"),
        'す' => Some("su"),
        'せ' => Some("se"),
        'そ' => Some("so"),
        'た' => Some("ta"),
        'ち' => Some("chi"),
        'つ' => Some("tsu"),
        'て' => Some("te"),
        'と' => Some("to"),
        'な' => Some("na"),
        'に' => Some("ni"),
        'ぬ' => Some("nu"),
        'ね' => Some("ne"),
        'の' => Some("no"),
        'は' => Some("ha"),
        'ひ' => Some("hi"),
        'ふ' => Some("fu"),
        'へ' => Some("he"),
        'ほ' => Some("ho"),
        'ま' => Some("ma"),
        'み' => Some("mi"),
        'む' => Some("mu"),
        'め' => Some("me"),
        'も' => Some("mo"),
        'や' | 'ゃ' => Some("ya"),
        'ゆ' | 'ゅ' => Some("yu"),
        'よ' | 'ょ' => Some("yo"),
        'ら' => Some("ra"),
        'り' => Some("ri"),
        'る' => Some("ru"),
        'れ' => Some("re"),
        'ろ' => Some("ro"),
        'わ' => Some("wa"),
        'を' => Some("wo"),
        'ん' => Some("n"),
        'が' => Some("ga"),
        'ぎ' => Some("gi"),
        'ぐ' => Some("gu"),
        'げ' => Some("ge"),
        'ご' => Some("go"),
        'ざ' => Some("za"),
        'じ' => Some("ji"),
        'ず' => Some("zu"),
        'ぜ' => Some("ze"),
        'ぞ' => Some("zo"),
        'だ' => Some("da"),
        'ぢ' => Some("ji"),
        'づ' => Some("zu"),
        'で' => Some("de"),
        'ど' => Some("do"),
        'ば' => Some("ba"),
        'び' => Some("bi"),
        'ぶ' => Some("bu"),
        'べ' => Some("be"),
        'ぼ' => Some("bo"),
        'ぱ' => Some("pa"),
        'ぴ' => Some("pi"),
        'ぷ' => Some("pu"),
        'ぺ' => Some("pe"),
        'ぽ' => Some("po"),
        'ゔ' => Some("vu"),
        _ => None,
    }
}

fn romanize_cyrillic(ch: char) -> Option<&'static str> {
    match ch {
        'А' => Some("A"),
        'а' => Some("a"),
        'Б' => Some("B"),
        'б' => Some("b"),
        'В' => Some("V"),
        'в' => Some("v"),
        'Г' => Some("G"),
        'г' => Some("g"),
        'Д' => Some("D"),
        'д' => Some("d"),
        'Е' => Some("E"),
        'е' => Some("e"),
        'Ё' => Some("Yo"),
        'ё' => Some("yo"),
        'Ж' => Some("Zh"),
        'ж' => Some("zh"),
        'З' => Some("Z"),
        'з' => Some("z"),
        'И' | 'І' => Some("I"),
        'и' | 'і' => Some("i"),
        'Й' => Some("Y"),
        'й' => Some("y"),
        'К' => Some("K"),
        'к' => Some("k"),
        'Л' => Some("L"),
        'л' => Some("l"),
        'М' => Some("M"),
        'м' => Some("m"),
        'Н' => Some("N"),
        'н' => Some("n"),
        'О' => Some("O"),
        'о' => Some("o"),
        'П' => Some("P"),
        'п' => Some("p"),
        'Р' => Some("R"),
        'р' => Some("r"),
        'С' => Some("S"),
        'с' => Some("s"),
        'Т' => Some("T"),
        'т' => Some("t"),
        'У' => Some("U"),
        'у' => Some("u"),
        'Ф' => Some("F"),
        'ф' => Some("f"),
        'Х' => Some("Kh"),
        'х' => Some("kh"),
        'Ц' => Some("Ts"),
        'ц' => Some("ts"),
        'Ч' => Some("Ch"),
        'ч' => Some("ch"),
        'Ш' => Some("Sh"),
        'ш' => Some("sh"),
        'Щ' => Some("Shch"),
        'щ' => Some("shch"),
        'Ы' => Some("Y"),
        'ы' => Some("y"),
        'Э' => Some("E"),
        'э' => Some("e"),
        'Ю' => Some("Yu"),
        'ю' => Some("yu"),
        'Я' => Some("Ya"),
        'я' => Some("ya"),
        'Ъ' | 'ъ' | 'Ь' | 'ь' => Some(""),
        'Є' => Some("Ye"),
        'є' => Some("ye"),
        'Ї' => Some("Yi"),
        'ї' => Some("yi"),
        _ => None,
    }
}

fn romanize_greek(ch: char) -> Option<&'static str> {
    match ch {
        'Α' | 'Ά' => Some("A"),
        'α' | 'ά' => Some("a"),
        'Β' => Some("V"),
        'β' => Some("v"),
        'Γ' => Some("G"),
        'γ' => Some("g"),
        'Δ' => Some("D"),
        'δ' => Some("d"),
        'Ε' | 'Έ' => Some("E"),
        'ε' | 'έ' => Some("e"),
        'Ζ' => Some("Z"),
        'ζ' => Some("z"),
        'Η' | 'Ή' => Some("I"),
        'η' | 'ή' => Some("i"),
        'Θ' => Some("Th"),
        'θ' => Some("th"),
        'Ι' | 'Ί' | 'Ϊ' => Some("I"),
        'ι' | 'ί' | 'ϊ' | 'ΐ' => Some("i"),
        'Κ' => Some("K"),
        'κ' => Some("k"),
        'Λ' => Some("L"),
        'λ' => Some("l"),
        'Μ' => Some("M"),
        'μ' => Some("m"),
        'Ν' => Some("N"),
        'ν' => Some("n"),
        'Ξ' => Some("X"),
        'ξ' => Some("x"),
        'Ο' | 'Ό' => Some("O"),
        'ο' | 'ό' => Some("o"),
        'Π' => Some("P"),
        'π' => Some("p"),
        'Ρ' => Some("R"),
        'ρ' => Some("r"),
        'Σ' => Some("S"),
        'σ' | 'ς' => Some("s"),
        'Τ' => Some("T"),
        'τ' => Some("t"),
        'Υ' | 'Ύ' | 'Ϋ' => Some("Y"),
        'υ' | 'ύ' | 'ϋ' | 'ΰ' => Some("y"),
        'Φ' => Some("F"),
        'φ' => Some("f"),
        'Χ' => Some("Ch"),
        'χ' => Some("ch"),
        'Ψ' => Some("Ps"),
        'ψ' => Some("ps"),
        'Ω' | 'Ώ' => Some("O"),
        'ω' | 'ώ' => Some("o"),
        _ => None,
    }
}

pub(crate) fn romanize_line(text: &str) -> Option<String> {
    if !text.chars().any(is_non_latin_romanizable) {
        return None;
    }

    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len() * 2);
    let mut i = 0;
    let mut double_next_consonant = false;

    while i < chars.len() {
        let ch = chars[i];
        let cp = ch as u32;

        // Hangul syllable decomposition
        if (0xAC00..=0xD7A3).contains(&cp) {
            let s_idx = (cp - 0xAC00) as usize;
            let initial = s_idx / 588;
            let vowel = (s_idx % 588) / 28;
            let final_c = s_idx % 28;
            out.push_str(HANGUL_INITIALS[initial]);
            out.push_str(HANGUL_VOWELS[vowel]);
            out.push_str(HANGUL_FINALS[final_c]);
            i += 1;
            continue;
        }

        // Japanese prolonged sound mark (ー)
        if ch == 'ー' {
            if let Some(last) = out.chars().last()
                && matches!(last, 'a' | 'e' | 'i' | 'o' | 'u')
            {
                out.push(last);
            }
            i += 1;
            continue;
        }

        // Japanese Hiragana / Katakana
        let hira = to_hiragana(ch);
        if hira == 'っ' {
            double_next_consonant = true;
            i += 1;
            continue;
        }

        if let Some(next_ch) = chars.get(i + 1).copied()
            && let Some(dig) = kana_digraph(hira, to_hiragana(next_ch))
        {
            if double_next_consonant {
                if let Some(first) = dig.chars().next() {
                    out.push(first);
                }
                double_next_consonant = false;
            }
            out.push_str(dig);
            i += 2;
            continue;
        }

        if let Some(rom) = kana_single(hira) {
            if double_next_consonant {
                if let Some(first) = rom.chars().next() {
                    out.push(first);
                }
                double_next_consonant = false;
            }
            out.push_str(rom);
            i += 1;
            continue;
        }

        double_next_consonant = false;

        if let Some(rom) = romanize_cyrillic(ch) {
            out.push_str(rom);
            i += 1;
            continue;
        }

        if let Some(rom) = romanize_greek(ch) {
            out.push_str(rom);
            i += 1;
            continue;
        }

        out.push(ch);
        i += 1;
    }

    let trimmed = out.trim().to_string();
    if trimmed.is_empty() || trimmed == text.trim() {
        None
    } else {
        Some(trimmed)
    }
}

pub(crate) fn enrich_romanization(lines: &mut [SyncedLyricLine]) {
    for line in lines.iter_mut() {
        if line.romanized.is_none() {
            line.romanized = romanize_line(&line.text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::romanize_line;

    #[test]
    fn skips_plain_latin_text() {
        assert_eq!(romanize_line("Never gonna give you up"), None);
        assert_eq!(romanize_line("Corazón espinao"), None);
    }

    #[test]
    fn romanizes_hangul_kana_cyrillic_and_greek() {
        assert_eq!(romanize_line("사랑해").as_deref(), Some("saranghae"));
        assert_eq!(
            romanize_line("ありがとう ございます").as_deref(),
            Some("arigatou gozaimasu")
        );
        assert_eq!(romanize_line("きっと").as_deref(), Some("kitto"));
        assert_eq!(romanize_line("Привет мир").as_deref(), Some("Privet mir"));
        assert_eq!(romanize_line("Καλημέρα").as_deref(), Some("Kalimera"));
    }
}
