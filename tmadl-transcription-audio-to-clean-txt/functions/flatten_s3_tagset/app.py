import json
import datetime
import logging
import boto3

### Example Event:
# {
#   "Records":[  
#      {  
#         "eventVersion":"2.1",
#         "eventSource":"aws:s3",
#         "awsRegion":"us-west-2",
#         "eventTime":"1970-01-01T00:00:00.000Z",
#         "eventName":"ObjectCreated:Put",
#         "userIdentity":{  
#            "principalId":"AIDAJDPLRKLG7UEXAMPLE"
#         },
#         "requestParameters":{  
#            "sourceIPAddress":"127.0.0.1"
#         },
#         "responseElements":{  
#            "x-amz-request-id":"C3D13FE58DE4C810",
#            "x-amz-id-2":"FMyUVURIY8/IgAtTv8xRjskZQpcIZ9KG4V5Wp6S7S/JRWeUWerMUE5JgHvANOjpD"
#         },
#         "s3":{  
#            "s3SchemaVersion":"1.0",
#            "configurationId":"testConfigRule",
#            "bucket":{  
#               "name":"some-bucket-name",
#               "ownerIdentity":{  
#                  "principalId":"A3NL1KOZZKExample"
#               },
#               "arn":"arn:aws:s3:::some-bucket-name"
#            },
#            "object":{  
#               "key":"some-recording.mp3",
#               "size":1024,
#               "eTag":"d41d8cd98f00b204e9800998ecf8427e",
#               "versionId":"096fKKXTRTtl3on89fVO.nfljtsv6qko",
#               "sequencer":"0055AED6DCD90281E5"
#            }
#         }
#      }
#   ]
# }
# Example event for real:
# {
#   "s3SchemaVersion": "1.0",
#   "configurationId": "testConfigRule",
#   "bucket": {
#     "name": "some-bucket-name",
#     "ownerIdentity": {
#       "principalId": "some-principal"
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
#     "TagSet": [
#       {
#         "Key": "date",
#         "Value": "1999-01-31T06:42:00Z"
#       },
#       {
#         "Key": "duration",
#         "Value": "42"
#       },
#       {
#         "Key": "number_of_speakers",
#         "Value": "6"
#       },
#       {
#         "Key": "recording_id",
#         "Value": "some-recording"
#       },
#       {
#         "Key": "description",
#         "Value": "Recording about something"
#       },
#       {
#         "Key": "language",
#         "Value": "French"
#       },
#       {
#         "Key": "source",
#         "Value": "gmeet"
#       }
#     ]
#   }
# }
def lambda_handler(raw_event, context):
    print(json.dumps(raw_event, default=str))

    # Flatten the tags.TagSet into an actual dictionary
    # For real, this AWS way of representing tag is such a dread.
    # This lambda wouldn't even exist if not for this.
    tags = {tag['Key']: tag['Value'] for tag in raw_event['tags']['TagSet']}

    # Make sure duration and number_of_speakers are integers
    if 'duration' in tags:
        tags['duration'] = int(tags['duration'])
    if 'number_of_speakers' in tags:
        tags['number_of_speakers'] = int(tags['number_of_speakers'])

    # Add the tags to the event
    raw_event['tags'] = tags
    return raw_event