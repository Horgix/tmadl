static MODEL : &str = "eu.anthropic.claude-3-7-sonnet-20250219-v1:0";

static PROMPT_FRAGMENT_GENERIC_SUMMARY_REQUEST: &str = r#"Summarize the following transcript into clear and readable bullet points with a couple of paragraphs around to introduce the topic and wrap it up."#;
static PROMPT_FRAGMENT_MULTI_SPEAKERS: &str = r#"Speakers in the transcript could be denoted by their name, or by "spk_x", where `x` is a number. These represent distinct speakers in the conversation. When you refer to a speaker, you may refer to them by "Speaker 1" #in the case of "spk_1", "Speaker 2" in the case of "spk_2", and so forth."#;
static PROMPT_FRAGMENT_SINGLE_SPEAKER: &str = r#"The transcript features a single speaker who recorded themselves in order to get a transcribe and summary."#;

pub fn summarize() -> String {
    //format!("Model: {}", MODEL)
    let prompt = format!("
{PROMPT_FRAGMENT_GENERIC_SUMMARY_REQUEST}
{PROMPT_FRAGMENT_MULTI_SPEAKERS}
    
Transcript:
TODO");

    println!("{}", prompt);
    prompt
}

// Claude requires the "Anthropic Claude Messages API" format,
// https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters-anthropic-claude-messages.html
// let messages = [
//     {
//         "role": "user",
//         "content": [
//             {
//                 "type": "text",
//                 "text": "TODO_PROMPT_REF"
//             }
//         ]
//     }
// ]