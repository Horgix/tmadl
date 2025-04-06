# TMA;DL - Too Much Audio, Didn't Listen

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Horgix/tmadl/build.yml)

`tmadl` is a meetings & discussions recordings processor.

It's a personal project trying to make it easier to process meetings and
discussion recordings with IA from your terminal/CLI.

## Internals

- The awesome **`clap` Rust library** is used to handle the CLI processing
  (commands/subcommands and their arguments/parameters)
- **AWS S3** to store the meeting recordings (audio files)
- **AWS Transcribe** to do the audio transcription to text
- **AWS Bedrock** to do the transcriptions summarization on-demand
