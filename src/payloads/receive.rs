use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub id: String,
    pub language: String,
    pub text: String,
    pub engine: String,
    pub truncated: bool,
    pub time_taken: u32,
    pub corrections: Vec<Correction>,
    pub sentences: Vec<Sentence>,
    pub auto_replacements: Vec<AutoReplacement>,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Correction {
    pub group: String,
    pub short_description: String,
    pub long_description: String,
    pub start_index: usize,
    pub end_index: usize,
    pub mistake_text: String,
    pub correction_text: String,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sentence {
    // Define fields based on the actual structure of sentences
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoReplacement {
    // Define fields based on the actual structure of autoReplacements
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Suggestion {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub text_length: u32,
    pub word_count: u32,
    pub sentence_count: u32,
    pub longest_sentence: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_api_response_deserialization() {
        let payload = r#"
        {
            "id": "a2f65624-0844-4d62-beea-a28220bcb9b1",
            "language": "fra",
            "text": "Je suis nul en maths",
            "engine": "Refine",
            "truncated": false,
            "timeTaken": 140,
            "corrections": [
                {
                    "group": "AutoCorrected",
                    "shortDescription": "",
                    "longDescription": "",
                    "startIndex": 15,
                    "endIndex": 18,
                    "mistakeText": "math",
                    "correctionText": "maths",
                    "suggestions": [
                        {
                            "text": "maths"
                        }
                    ]
                }
            ],
            "sentences": [],
            "autoReplacements": [],
            "stats": {
                "textLength": 19,
                "wordCount": 5,
                "sentenceCount": 1,
                "longestSentence": 19
            }
        }
        "#;

        let api_response: ApiResponse = serde_json::from_str(payload).unwrap();

        assert_eq!(api_response.id, "a2f65624-0844-4d62-beea-a28220bcb9b1");
        assert_eq!(api_response.language, "fra");
        assert_eq!(api_response.text, "Je suis nul en maths");
        assert_eq!(api_response.engine, "Refine");
        assert_eq!(api_response.truncated, false);
        assert_eq!(api_response.time_taken, 140);
        assert_eq!(api_response.corrections.len(), 1);
        assert_eq!(api_response.corrections[0].group, "AutoCorrected");
        assert_eq!(api_response.corrections[0].start_index, 15);
        assert_eq!(api_response.corrections[0].end_index, 18);
        assert_eq!(api_response.corrections[0].mistake_text, "math");
        assert_eq!(api_response.corrections[0].correction_text, "maths");
        assert_eq!(api_response.corrections[0].suggestions.len(), 1);
        assert_eq!(api_response.corrections[0].suggestions[0].text, "maths");
        assert_eq!(api_response.sentences.len(), 0);
        assert_eq!(api_response.auto_replacements.len(), 0);
        assert_eq!(api_response.stats.text_length, 19);
        assert_eq!(api_response.stats.word_count, 5);
        assert_eq!(api_response.stats.sentence_count, 1);
        assert_eq!(api_response.stats.longest_sentence, 19);
    }
}
