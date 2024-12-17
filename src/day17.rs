// Ensure all relevant items are marked with `pub` keyword

const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

// Your Solution here ...
// Replace the local part of an email with Christmas emojis, for example santa@north.pole should be anonymized to 🎅🎄🎁🎄🎅@north.pole.
// If the email is invalid, turn every character into emojis, for example santa should be anonymized to 🎅🎄🎁🎄🎅
pub trait Anonymized {
    fn anonymize_email(&self) -> String;
}

impl Anonymized for String {

    fn anonymize_email(&self) -> String {

        let length = CHRISTMAS_EMOJIS.len();

        match self.split_once("@") {
            Some((name, domain)) 
                => format!("{}@{}", 
                    (0..name.len()).map(|i| CHRISTMAS_EMOJIS[i % length]).collect::<String>(), 
                    domain),
            None 
                => (0..self.len()).map(|i| CHRISTMAS_EMOJIS[i % length]).collect()
        }
    }
}


pub fn main() {
    let emails = vec![
        "rudolph.therapysessions@northpole.com".to_string(),
        "elfhr.complaint@northpole.urgent".to_string(),
        "santas.rage.management@christmaschaos.noel".to_string(),
        "overtimepay.never@elfexploitation.workshop".to_string(),
        "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
        "reindeer.workers.comp@antler.insurance".to_string(),
        "naughty.list.revenge@santasecrets.com".to_string(),
        "workshop.ptsd.support@elves.anonymous".to_string(),
        "performance.anxiety@santa.breakdown".to_string(),
        "existential.crisis@northpole.void".to_string(),
    ];

    for email in emails {
        let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
        println!("Original: {} -> Anonymized: {}", email, anonymized_email);
    }
}



// fn splitEmail(txt:&str) -> Option<(&str, &str)> {
//     let pos:Vec<&str> = txt.split("@").collect();
//     if pos.len() != 2 {
//         None
//     } else {
//         Some((pos[0], pos[1]))
//     }
// }
// fn hide(txt: &str) -> String {
//     (0..txt.len()).map(|i| CHRISTMAS_EMOJIS[i % CHRISTMAS_EMOJIS.len()]).collect()
// }
// impl Anonymize for String {
//     fn anonymize_email(&self) -> String {
//         if let Some((left, right)) = splitEmail(self) {
//             hide(left) + "@" + &hide(right)
//         } else{
//             hide(self)
//         }
//     }
// }