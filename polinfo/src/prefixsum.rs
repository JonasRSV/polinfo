use crate::core;
use crate::textprocessing;
use std::collections::HashMap;


pub struct PrefixSumFactory {
    wordifier: textprocessing::Wordifier
}


impl PrefixSumFactory {

    fn accumulate_word_counts(words: &Vec<String>, counts: &mut HashMap<String, u64>) {
        for word in words.iter() {
            if !counts.contains_key(word) {
                counts.insert(word.clone(), 0);
            }

            *counts.get_mut(word).unwrap() += 1;

        }
    }

    pub fn new() -> Result<Self, String> {
        match textprocessing::Wordifier::new() {
            Err(err) => Err(format!("Failed to create PrefixSumFactory {:?}", err)),
            Ok(wordifier) => Ok(
                PrefixSumFactory{
                    wordifier
                })
        }

    }

    pub fn from_anforanden(&self, anforanden: &mut Vec<core::Anforande>) -> core::PrefixSum {
        anforanden.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut counts = Vec::new();
        let mut dates = Vec::new();

        let mut anchor = anforanden.first().unwrap().time;
        let mut word_counts = HashMap::new();


        for anforande in anforanden.iter() {
            let words = self.wordifier.words(&anforande.content);

            //println!("word {:?}", words);

            PrefixSumFactory::accumulate_word_counts(&words, &mut word_counts);

            if anforande.time.signed_duration_since(anchor).num_weeks() > 4 {
                dates.push(anforande.time);
                counts.push(word_counts.clone());

                anchor = anforande.time;
            }
        }

        dates.push(anforanden.last().unwrap().time);
        counts.push(word_counts.clone());

        core::PrefixSum {
            counts,
            dates
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core;
    use chrono::NaiveDateTime;
    use dotenv::dotenv;

    #[test]
    fn new_from_anforanden() {
        dotenv().ok();
        let factory = PrefixSumFactory::new().expect("Failed to construct PrefixSumFactory");

        let mut anforanden = vec![
            core::Anforande {
                content: "Hello wat sjukvård a beautiful day, what hello?".to_owned(),
                time: NaiveDateTime::parse_from_str("2018-01-02 0:0:0", "%Y-%m-%d %H:%M:%S").unwrap()
            },
            core::Anforande {
                content: "Hello what miljö a beautiful day, what hello?".to_owned(),
                time: NaiveDateTime::parse_from_str("2018-01-02 0:0:0", "%Y-%m-%d %H:%M:%S").unwrap()
            },
        ];


        let prefixsum = factory.from_anforanden(&mut anforanden);
        println!("{:?}", prefixsum);
        assert!(prefixsum.counts.len() > 0);
    }
}
