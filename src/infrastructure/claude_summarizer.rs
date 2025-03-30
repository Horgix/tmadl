use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_s3::primitives::Blob;
use serde_json::json;

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

pub struct ClaudeSummarizer {
    client: BedrockClient,
    model: String,
}

impl ClaudeSummarizer {
    #[tokio::main(flavor = "current_thread")]
    pub async fn new() -> ClaudeSummarizer {
        let config = aws_config::defaults(BehaviorVersion::latest())
                .load()
                .await;
        let client = BedrockClient::new(&config);

        ClaudeSummarizer {
            client: client,
            model: MODEL.to_owned(),
        }
    }

    #[tokio::main(flavor = "current_thread")]
    pub async fn summarize(&self, input: &str) -> String {
        // Whelp, nevermind. invoke_model cannot directly take a Message as
        // input, it needs a Blob. Plus, the InvokeModelRequest builder doesn't
        // exist in the aws_sdk_bedrockruntime crate.
        // The cleanest solution would be to implement JSON serialization using
        // serde for Message, ContentBlock, and ConversationRole, and then
        // convert the JSON string to a Blob. But that's a lot of mess.
        // So, let's just print the JSON for now.
        // let bedrockClaudeInput = Message::builder()
        //         .role(ConversationRole::User)
        //         .content(ContentBlock::Text(input.to_string()))
        //         .build()
        //         .map_err(|_| "failed to build message").unwrap();
        let bedrock_claude_input = json!(
            [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": input
                        }
                    ]
                }
            ]);

        let model_parameters = json!({
                "anthropic_version": "bedrock-2023-05-31",
                "max_tokens": 20000,
                "system": "You are an AI assistant that excels at summarizing conversations.",
                "messages": bedrock_claude_input,
                "temperature": 1.0,
                "top_p": 0.999,
                "top_k": 40,
            });

        let response = self.client
            .invoke_model()
            .model_id(MODEL)
            .body(Blob::new(model_parameters.to_string().as_bytes().to_vec()))
            .send()
            .await;

        match response {
            Ok(output) => {
                let output_blob: Vec<u8> = output.body().clone().into();
                let output_json = String::from_utf8(output_blob).unwrap();
                println!("Output: {}", output_json);
                output_json
            }
            Err(err) => {
                println!("Error: {:?}", err);
                String::from("Error")
            }
        }
    }
}