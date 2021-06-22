use crate::core;
use crate::textprocessing;
use std::collections::HashMap;


pub struct PrefixSumFactory {
    wordifier: textprocessing::Wordifier,
    keep_top: usize

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
                    keep_top: 3000
                })
        }

    }

    fn purge_rare_words(&self, prefixsum: &mut core::PrefixSum) {
        let last = prefixsum.counts.last().unwrap();

        let mut word_count: Vec<(String, f64)> = last
            .into_iter()
            .map(|(word, count)| (word.clone(), count.clone()))
            .collect();

        word_count.sort_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap());

        let least_common: Vec<String> = word_count
            .iter()
            .take(word_count.len() - self.keep_top)
            .map(|(word, _)| word.clone())
            .collect();

        //println!("most common {:?}", word_count.iter().rev().take(100));


        for count in prefixsum.counts.iter_mut() {
            for word in least_common.iter() {
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

        let mut anchor = tts.first().unwrap().time;
        let mut word_counts = HashMap::new();


        for tt in tts.iter() {
            let words = self.wordifier.words(&tt.content);
            PrefixSumFactory::accumulate_word_counts(&words, &mut word_counts);
            if tt.time.signed_duration_since(anchor).num_weeks() > 4 {
                dates.push(tt.time);
                counts.push(word_counts.clone());

                anchor = tt.time;
            }
        }

        let mut prefixsum = core::PrefixSum {
            counts,
            dates
        };

        self.purge_rare_words(&mut prefixsum);

        prefixsum
    }
}

pub fn norm_words(prefixsum: &core::PrefixSum) -> HashMap<String, f64> {
    let mut word_counts = HashMap::new();
    for count in prefixsum.counts.iter() {
        for key in count.keys() {
            if !word_counts.contains_key(key) {
                word_counts.insert(key.to_owned(), 0.0);
            }

            *word_counts.get_mut(key).unwrap() += count.get(key).unwrap();
        }
    }

    word_counts.clone()
}

pub fn norm_count(prefixsum: &core::PrefixSum) -> f64 {
    norm_words(prefixsum).values().sum() 
}

pub fn div_by_words(lhs: &mut core::PrefixSum, rhs: &HashMap<String, f64>) {
    for count in lhs.counts.iter_mut() {
        let keys: Vec<String> = count.keys().into_iter().map(|s| s.to_owned()).collect();
        for key in keys.iter() {
            count.insert(key.to_owned(), count.get(key).unwrap() / rhs.get(key).unwrap());
        }
    }
}

pub fn div_by_count(lhs: &mut core::PrefixSum, rhs: f64) {
    for count in lhs.counts.iter_mut() {
        let keys: Vec<String> = count.keys().into_iter().map(|s| s.to_owned()).collect();
        for key in keys.iter() {
            count.insert(key.to_owned(), count.get(key).unwrap() / rhs);
        }
    }
}

pub fn add_maps(lhs: &HashMap<String, f64>, rhs: &HashMap<String, f64>) 
    -> HashMap<String, f64>{
    let mut word_counts = HashMap::new();

    for key in lhs.keys() {
        if !word_counts.contains_key(key) {
            word_counts.insert(key.to_owned(), 0.0);
        }

        let m = word_counts.get_mut(key).unwrap();
        *m += lhs.get(key).unwrap();
    }

    for key in rhs.keys() {
        if !word_counts.contains_key(key) {
            word_counts.insert(key.to_owned(), 0.0);
        }

        let m = word_counts.get_mut(key).unwrap();
        *m += rhs.get(key).unwrap();
    }


    word_counts
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
