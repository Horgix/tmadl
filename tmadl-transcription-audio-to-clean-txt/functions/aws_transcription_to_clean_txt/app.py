import json
import datetime
import logging
import boto3
import tempfile
import os

def convert_to_txt_file(json_file):
    """
    Convert the JSON output of Amazon Transcribe to plaintext format, and write it to a file.
    TODO find source of this logic again in AWS blogpost and reference it here.

    Args:
        json_file (str): Path to the JSON file with the default Transcribe output.

    Returns:
        str: The converted transcript
        str: The path to the output text file or None
    """
    try:
        with open(json_file, "r") as f:
            data = json.load(f)
    except json.decoder.JSONDecodeError:

        logger.error("File is not a valid JSON file")
        return None

    # Save the converted output to a tempoary file
    temp_file = tempfile.NamedTemporaryFile(dir="/tmp", suffix=".txt", delete=False)
    output_path = temp_file.name

    current_speaker = None
    current_text = ""
    output = []

    with open(output_path, "w", encoding="utf-8") as output_file:
        for item in data["results"].get("items", []):
            if item["type"] == "pronunciation":
                content = item["alternatives"][0]["content"]
                speaker_label = item["speaker_label"] # Horgix

                if speaker_label != current_speaker:
                    if current_text:
                        output_file.write(f"{current_speaker}: {current_text.strip()}\n")
                        output.append(f"{current_speaker}: {current_text.strip()}\n")
                    current_speaker = speaker_label
                    current_text = content
                else:
                    current_text += " " + content
            elif item["type"] == "punctuation":
                current_text += item["alternatives"][0]["content"]

        if current_text:
            output_file.write(f"{speaker_label}: {current_text.strip()}\n")
            output.append(f"{speaker_label}: {current_text.strip()}\n")

    return "".join(output), output_path

### Example input:
# {
#   "s3SchemaVersion": "1.0",
#   "configurationId": "testConfigRule",
#   "bucket": {
#     "name": "some-bucket-name",
#     "ownerIdentity": {
#       "principalId": "some-principal-id"
#     },
#     "arn": "arn:aws:s3:::some-bucket-name"
#   },
#   "object": {
#     "key": "raw-recordings/some-recording.mp3",
#     "size": 1024,
#     "eTag": "d41d8cd98f00b204e9800998ecf8427e",
#     "versionId": "096fKKXTRTtl3on89fVO.nfljtsv6qko",
#     "sequencer": "0055AED6DCD90281E5"
#   },
#   "tags": {
#     "date": "1999-01-31T06:42:00Z",
#     "duration": 42,
#     "number_of_speakers": 6,
#     "recording_id": "some-recording",
#     "description": "This is the recording of something",
#     "language": "French",
#     "source": "gmeet"
#   },
#   "TranscriptionJob": {
#     "TranscriptionJob": {
#       "CompletionTime": "2025-04-08T00:20:52.040Z",
#       "CreationTime": "2025-04-08T00:18:37.138Z",
#       "LanguageCode": "fr-FR",
#       "Media": {
#         "MediaFileUri": "s3://some-bucket-name/raw-recordings/some-recording.mp3"
#       },
#       "MediaFormat": "mp4",
#       "MediaSampleRateHertz": 44100,
#       "Settings": {
#         "ChannelIdentification": false,
#         "MaxSpeakerLabels": 6,
#         "ShowAlternatives": false,
#         "ShowSpeakerLabels": true
#       },
#       "StartTime": "2025-04-08T00:18:37.166Z",
#       "Transcript": {
#         "TranscriptFileUri": "https://s3.eu-west-3.amazonaws.com/some-bucket-name/transcriptions/some-recording.json"
#       },
#       "TranscriptionJobName": "transcripton-from-stepfunction-demo",
#       "TranscriptionJobStatus": "COMPLETED"
#     }
#   }
# }
def lambda_handler(raw_event, context):
    logger = logging.getLogger()
    logger.setLevel("INFO")
    logger.info(json.dumps(raw_event, default=str))

    s3_client = boto3.client("s3")

    transcription_file_url = raw_event["TranscriptionJob"]["TranscriptionJob"]["Transcript"]["TranscriptFileUri"]
    bucket_name = raw_event["bucket"]["name"]
    object_key = transcription_file_url[len(f"https://s3.eu-west-3.amazonaws.com/{bucket_name}/"):]
    file_basename = object_key.split("/")[-1].split(".")[0]

    logger.debug(f"Transcription file URL: {transcription_file_url}")
    logger.debug(f"Bucket name: {bucket_name}")
    logger.debug(f"File basename: {file_basename}")
    logger.debug(f"File key: {object_key}")

    try:
        s3_client.download_file(bucket_name,
                                object_key,
                                f"/tmp/{file_basename}.json")
        logger.info(f"Downloaded JSON")
    except Exception as e:
        logger.error(f"Error downloading JSON file: {e}")
        raise e

    # Convert to a clean txt file with per-speaker lines
    transcript_content, local_transcript_output_file = convert_to_txt_file(f"/tmp/{file_basename}.json")
    if not local_transcript_output_file or not os.path.exists(transcript_output_file):
        logger.error("Error converting transcription to txt file")
        exit

    logger.info(f"Converted transcription to {local_transcript_output_file}")

    # Save the transcription to S3 so it can be referenced or reviewed at a later time
    try:
        transcription_job_name_txt = f"{file_basename}.txt"
        s3_client.upload_file(local_transcript_output_file,
                              bucket_name,
                              f"transcriptions/{file_basename}.txt")
        logger.info(f"Uploaded TXT")
    except Exception as e:
        logger.error(f"Error uploading txt file to S3: {e}")
        raise e
    
    # Finish by returning the transcription cleaned s3 path in a lambda compliant way
    return {
        "statusCode": 200,
        "body": json.dumps({
            "transcription_file": f"s3://{bucket_name}/transcriptions/{file_basename}.txt",
        }),
        "headers": {
            "Content-Type": "application/json"
        }
    }
    