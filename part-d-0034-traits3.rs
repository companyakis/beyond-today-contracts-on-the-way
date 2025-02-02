fn main() {

    let turkic = TurkicPeople {language: "Turkish, Uzbek, Turkmen etc...".to_string()};

    let usa = USAPeople {language: "English".to_string()};

    turkic.speak_lang();

    usa.speak_lang();

// Turkic people language: Turkish, Uzbek, Turkmen etc...
// USA people speak: English

}

trait Speak {

    fn speak_lang(&self);
}

struct TurkicPeople {
    language: String,
}

impl Speak for TurkicPeople {
    fn speak_lang(&self) {
        println!("Turkic people language: {}", self.language);
    }
}

struct USAPeople {
    language: String,
}

impl Speak for USAPeople {
    fn speak_lang(&self) {
        println!("USA people speak: {}", self.language);
    }
}
    





