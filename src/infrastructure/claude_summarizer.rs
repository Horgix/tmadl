use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use aws_sdk_s3::primitives::Blob;
use serde_json::json;

use crate::domain::recording::Recording;
use crate::domain::summary::SummaryRequest;

static MODEL : &str = "eu.anthropic.claude-3-7-sonnet-20250219-v1:0";

static PROMPT_FRAGMENT_GENERIC_SUMMARY_REQUEST: &str = r#"Summarize the following transcript into clear and readable bullet points with a couple of paragraphs around to introduce the topic and wrap it up."#;
static PROMPT_FRAGMENT_MULTI_SPEAKERS: &str = r#"Speakers in the transcript could be denoted by their name, or by "spk_x", where `x` is a number. These represent distinct speakers in the conversation. When you refer to a speaker, you may refer to them by "Speaker 1" #in the case of "spk_1", "Speaker 2" in the case of "spk_2", and so forth."#;
static PROMPT_FRAGMENT_SINGLE_SPEAKER: &str = r#"The transcript features a single speaker who recorded themselves in order to get a transcribe and summary."#;
static PROMPT_FRAGMENT_ADDITIONAL_NOTES_PREFIX: &str = r#"Additional notes for you to take into account:"#;

pub fn summarize(summary_request: SummaryRequest) -> String {
    // If the summary_request's recording contains a description, or if theyre's
    // any additional_context, build a list of strings merging both into a  ist with '- ' as a string
    // and join them with '\n' to create a bullet point list.
    let additional_notes =  if  summary_request.recording.description.is_some() || summary_request.additional_context.is_some() {
        let description_note = match summary_request.recording.description {
            Some(description) => {
                vec![format!("This recording is about: {}", description)]
            },
            None => {
                vec![]
            }
        };
        let list_of_additional_notes = [
            description_note,
            summary_request.additional_context.unwrap_or(vec![])
            ].concat()
            .iter()
            .map(|note| format!("- {}", note))
            .collect::<Vec<String>>()
            .join("\n");
        format!("{}\n{}", PROMPT_FRAGMENT_ADDITIONAL_NOTES_PREFIX, list_of_additional_notes)
        } else {
            "".to_string()
        };

    //format!("Model: {}", MODEL)
    let prompt = format!("
{PROMPT_FRAGMENT_GENERIC_SUMMARY_REQUEST}
{PROMPT_FRAGMENT_MULTI_SPEAKERS}
{additional_notes}

Transcript:
TODO
");
    println!("{}", prompt);
    prompt
}

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
        // Claude requires the "Anthropic Claude Messages API" format,
        // https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters-anthropic-claude-messages.html
        //
        // In the Rust SDK, invoke_model cannot directly take a Message as
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
            .model_id(self.model.to_owned())
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