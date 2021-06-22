use crate::core;
use crate::textprocessing;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use log::info;


pub struct PrefixSumFactory {
    wordifier: textprocessing::Wordifier,
    drop_less_than: f64

}


impl PrefixSumFactory {

    fn accumulate_word_counts(words: &Vec<String>, counts: &mut HashMap<String, f64>) {
        for word in words.iter() {
            if !counts.contains_key(word) {
                counts.insert(word.clone(), 0.0);
            }

            *counts.get_mut(word).unwrap() += 1.0;

        }
    }

    pub fn new() -> Result<Self, String> {
        match textprocessing::Wordifier::new() {
            Err(err) => Err(format!("Failed to create PrefixSumFactory {:?}", err)),
            Ok(wordifier) => Ok(
                PrefixSumFactory{
                    wordifier,
                    drop_less_than: 50.0
                })
        }
    }

    fn purge_rare_words(&self, prefixsum: &mut core::PrefixSum) {
        let last = prefixsum.counts.last().unwrap();

        let not_common_enough: Vec<String> = last.into_iter()
            .filter(|(_, count)| **count < self.drop_less_than)
            .map(|(word, _)| word.clone())
            .collect();

        for count in prefixsum.counts.iter_mut() {
            for word in not_common_enough.iter() {
                count.remove(word);
            }
        }
    }

    /// Calculate PrefixSum from a vector of texttimes
    ///
    /// # Arguments
    ///
    /// * `tts` - texttimes are struct of text and their time.
    ///
    pub fn from_texttimes(&self, tts: &mut Vec<core::TextTime>) -> core::PrefixSum {
        tts.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        let mut counts = Vec::new();
        let mut dates = Vec::new();

        let mut word_counts = HashMap::new();
        let mut current_time: NaiveDateTime = chrono::NaiveDate::from_ymd(
            1995, 1, 1).and_hms(0, 0, 0);

        let mut tts_iter = tts.iter().peekable();

        while tts_iter.peek().is_some() {
            let tt = tts_iter.next().unwrap();
            while  current_time < tt.time {
                current_time = current_time + chrono::Duration::weeks(16);

                counts.push(word_counts.clone());
                dates.push(current_time.clone());

            }

            let words = self.wordifier.words(&tt.content);
            PrefixSumFactory::accumulate_word_counts(&words, &mut word_counts);

            info!("{} - Unique Words {}  \r", current_time, word_counts.len());


        }

        let mut prefixsum = core::PrefixSum {
            counts,
            dates
        };

        self.purge_rare_words(&mut prefixsum);
        info!("Unique words after purging rare {}", 
            prefixsum.counts.last().unwrap().len());

        prefixsum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core;
    use chrono::NaiveDateTime;
    use dotenv::dotenv;

    #[test]
    fn new_from_tts() {
        dotenv().ok();
        let factory = PrefixSumFactory::new().expect("Failed to construct PrefixSumFactory");

        let mut tts = vec![
            core::TextTime {
                content: "Hello wat sjukvård a beautiful day, what hello?".to_owned(),
                time: NaiveDateTime::parse_from_str("2018-01-02 0:0:0", "%Y-%m-%d %H:%M:%S").unwrap()
            },
            core::TextTime {
                content: "Hello what miljö a beautiful day, what hello?".to_owned(),
                time: NaiveDateTime::parse_from_str("2018-01-02 0:0:0", "%Y-%m-%d %H:%M:%S").unwrap()
            },
        ];


        let prefixsum = factory.from_texttimes(&mut tts);
        println!("{:?}", prefixsum);
        assert!(prefixsum.counts.len() > 0);
    }
}
