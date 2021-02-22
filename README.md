# Tapalogi's Mailer Microservice

A mailer microservice that respects SMTP rate limits.

## Data Structures

### 1. MessageDraft

All draft/pending email must be in a valid json format, this service will consume from [MQ_TOPIC_SOURCE](https://github.com/Tapalogi/tapa-micro-mailer/blob/4ad9a630b660bff4a1e189bc4d84cfbaf58311d3/example/local.env#L3). Example format:

```json
{
  "id":"320b0555-4c73-4abf-aaf0-461b84860046", //UUID
  "email_to":"admin@example.com",
  "email_to_name":"Tapalogi Administrator",
  "email_from":"noreply@example.com",
  "email_from_name":"Tapalogi System",
  "subject":"Tapa Micro Mailer - Test #1613990722427731276",
  "body_type":"HTML", //HTML/ASCII
  "body":"Hello!! This is from example.com",
  "timestamp":"2021-02-22T10:45:22.427738+00:00" //RFC3339+FixedOffset
}
```

### 2. MessageFail

Every failed draft consumption will produce an event to [MQ_TOPIC_FAILURE](https://github.com/Tapalogi/tapa-micro-mailer/blob/4ad9a630b660bff4a1e189bc4d84cfbaf58311d3/example/local.env#L4). Example format:

```json
{
  "origin_offset":null,
  "service_instance_name":"MAILER-TEST_a883fe203bb31",
  "message_copy":"{
      \"id\":\"320b0555-4c73-4abf-aaf0-461b84860046\",
      \"email_to\":\"admin@example.com\",
      \"email_to_name\":\"Tapalogi Administrator\",
      \"email_from\":\"noreply@example.com\",
      \"email_from_name\":\"Tapalogi System\",
      \"subject\":\"Tapa Micro Mailer - Test #1613990722427731276\",
      \"body_type\":\"HTML\",
      \"body\":\"Hello!! This is from example.com\",
      \"timestamp\":\"2021-02-22T10:45:22.427738+00:00\"
    }",
  "fail_reason":"UNKNOWN", //OTHER/BAD_DRAFT/QUOTA_EXHAUSTED/UNKNOWN
  "timestamp":"2021-02-22T10:45:22.427738+00:00" //RFC3339+FixedOffset
}
```

### 3. MessageSent

Every successful draft consumption will produce an event to [MQ_TOPIC_SUCCESS](https://github.com/Tapalogi/tapa-micro-mailer/blob/4ad9a630b660bff4a1e189bc4d84cfbaf58311d3/example/local.env#L5). Example format:

```json
{
  "origin_offset":null,
  "service_instance_name":"MAILER-TEST_a883fe203bb31",
  "draft_id":"320b0555-4c73-4abf-aaf0-461b84860046",
  "timestamp":"2021-02-22T10:45:22.427738+00:00" //RFC3339+FixedOffset
}
```
