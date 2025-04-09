# TMA;DL - Too Much Audio, Didn't Listen

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Horgix/tmadl/build.yml)
![GitHub License (MIT)](https://img.shields.io/github/license/Horgix/tmadl)

`tmadl` is a meetings & discussions recordings processor.

It's a personal project trying to make it easier to process meetings and
discussion recordings with IA from your terminal/CLI.

## The gist of it

The workflow that enable TMA;DL is essentially:

1. **Record** a meeting or discussion —  _no matter if it's through the native
   GMeet feature, a local `ffmpeg` command (soon to be wrapped in `tmadl
   record`), a recording made on your phone, etc._
2. **Upload** the resulting audio file to S3 through `tmadl upload`
3. _An AWS StepFunction (deployed through AWS SAM as provided in [the
   `tmadl-transcription-audio-to-clean-txt`
   directory](./tmadl-transcription-audio-to-clean-txt)) automatically kicks
   in, transcribe the audio into text through AWS Transcribe and a couple
   lambdas_
4. **Summarize/interrogate** your transcrips using `tmadl ask` that queries AWS
   Bedrock (Anthropic Claude), providing meaningful context and the transcript
   in order to summarize it **the way you want**.

## The CLI

```
$ ./target/debug/tmadl --help

TMA;DL (Too Much Audio, Didn't Listen) — A terminal/CLI meetings & discussions recordings processor using AI

Usage: tmadl [COMMAND]

Commands:
  record  Record a new meeting/conversation # Through ffmpeg. To be implemented
  upload  Upload a local recording to S3
  list    List all recordings (and their transcription statuses)
  ask     Interrogate a transcription
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## The StepFunction

![TMA;DL audio transcription to clean txt through AWS
StepFunction](./tmadl-transcription-audio-to-clean-txt/docs/stepfunction.png)

## Internals

- The awesome **`clap` Rust library** is used to handle the CLI processing
  (commands/subcommands and their arguments/parameters)
- **AWS S3** to store the meeting recordings (audio files)
- **AWS Transcribe** to do the audio transcription to text
- **AWS Bedrock** to do the transcriptions summarization on-demand

TODO later add:

- Sam for reaction on S3
- Ratatui or other for the dynamic termui?

https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModel.html
- Uses the `InvokeModel` API from the Bedrock Runtime - _**not** the Converse
  API or something else_
- Claude message API format
https://docs.aws.amazon.com/bedrock/latest/userguide/model-parameters-anthropic-claude-messages.html
- High number of tokens + timeout to support long meetings


```
magicalrecord: aliased to ffmpeg -f pulse -i virtual_output_for_recording_purpose.monitor -f pulse -i alsa_input.pci-0000_07_00.6.HiFi__Mic1__source -filter_complex "[0:a]aresample=sample_rate=44100,volume=1[a0];[1:a]aresample=sample_rate=44100,volume=3[a1];[a0][a1]amix=inputs=2"  -ac 2
```

```
pactl load-module module-null-sink sink_name=virtual_output_for_recording_purpose
pactl load-module module-loopback source=virtual_output_for_recording_purpose.monitor
```

- id+date must be unique
- ID can be duplicate
- Files will be name as date_id.whateverextension
