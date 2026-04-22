## Development with AWS Services

Q: What is the Lambda execution model for synchronous vs asynchronous invocations?
A: Synchronous: caller waits for the response (API Gateway, SDK invoke). Asynchronous: Lambda queues the event and returns immediately (S3, SNS, EventBridge). For async, Lambda handles retries (2 retries by default) and can send failures to a destination or DLQ.

Q: How does DynamoDB partition data and why does partition key design matter?
A: DynamoDB hashes the partition key to determine which physical partition stores the item. Throughput is divided evenly across partitions. A poorly chosen partition key (low cardinality) creates hot partitions that get throttled even when total table capacity is available.

Q: What is the difference between DynamoDB Query and Scan, and when should you use each?
A: Query retrieves items by partition key with optional sort key conditions — efficient and reads only matching items. Scan reads every item in the table and filters afterward — expensive and slow. Always prefer Query. Use parallel Scan only for one-time exports.

Q: How does Lambda handle event source mappings for streams (Kinesis, DynamoDB Streams)?
A: Lambda polls the stream, retrieves batches of records, and invokes the function synchronously. If the function fails, the entire batch is retried (blocking the shard). Use bisect-batch-on-error, max retry attempts, and on-failure destinations to handle poison pills.

Q: What is the API Gateway Lambda proxy integration response format?
A: The Lambda function must return: { "statusCode": 200, "headers": { "Content-Type": "application/json" }, "body": "{\"key\": \"value\"}" }. The body must be a string (JSON.stringify for objects). Missing statusCode or non-string body causes a 502 error.

Q: How do DynamoDB transactions work?
A: TransactWriteItems and TransactGetItems provide ACID transactions across up to 100 items in multiple tables. All operations succeed or all fail. Transactions consume 2x the normal capacity units. Use for operations that must be atomic (e.g., transferring funds between accounts).

Q: What is the difference between SQS standard and FIFO queues?
A: Standard: unlimited throughput, at-least-once delivery, best-effort ordering. FIFO: 300 TPS (3,000 with batching), exactly-once processing, strict ordering within message groups. FIFO queue names must end in .fifo. Choose based on whether ordering and deduplication matter.

Q: How does Lambda concurrency work?
A: Each concurrent request uses one execution environment. Account default: 1,000 concurrent executions (shared across all functions). Reserved concurrency guarantees capacity for a function but also caps it. Provisioned concurrency pre-warms environments to eliminate cold starts.

Q: What is the SNS fan-out pattern and when should you use it?
A: Publish a message to an SNS topic that has multiple SQS queues subscribed. Each queue gets a copy. Use when a single event must trigger multiple independent processing pipelines (e.g., order placed → update inventory + send email + update analytics).

Q: How does Step Functions handle errors and retries?
A: State machines define Retry (with interval, backoff rate, max attempts) and Catch (fallback state) on each task. This is declarative — no retry logic in your Lambda code. Step Functions also supports timeouts (TimeoutSeconds, HeartbeatSeconds).

Q: What is the difference between Kinesis Data Streams and SQS for event processing?
A: Kinesis: real-time streaming, ordered within shard, multiple consumers can read the same data, data replay up to 365 days. SQS: message queue, messages deleted after processing, no replay, simpler. Use Kinesis for real-time analytics; SQS for decoupling and work queues.

Q: How does DynamoDB conditional writes prevent race conditions?
A: Conditional writes use ConditionExpression to check a condition before writing. If the condition fails, the write is rejected with ConditionalCheckFailedException. Common pattern: optimistic locking with a version attribute — update only if version matches what you read.

Q: What is the difference between API Gateway REST API and HTTP API?
A: REST API: request validation, caching, WAF integration, usage plans, resource policies, more authorizer types. HTTP API: 70% cheaper, lower latency, native JWT authorizer, simpler. HTTP API lacks caching, WAF, and request validation. Choose HTTP API unless you need REST-specific features.

Q: How does EventBridge differ from SNS for event routing?
A: EventBridge: content-based filtering on the event body, schema registry, archive and replay, SaaS integrations, more target types. SNS: simpler pub/sub, higher throughput, message attribute filtering only. EventBridge is preferred for complex event-driven architectures.

## Security

Q: How does IAM policy evaluation work when multiple policies apply?
A: 1) All policies are collected (identity-based, resource-based, SCPs, permission boundaries). 2) Explicit Deny always wins. 3) If no explicit Deny, there must be an explicit Allow. 4) Default is implicit Deny. For cross-account access, BOTH accounts must allow the action.

Q: What is the difference between Cognito user pools and identity pools?
A: User pools handle authentication — sign-up, sign-in, MFA, token issuance (JWT). Identity pools handle authorization — exchange tokens for temporary AWS credentials (STS). Typical flow: user authenticates with user pool → identity pool exchanges the JWT for AWS credentials.

Q: How does KMS envelope encryption work and why is it needed?
A: KMS Encrypt has a 4 KB limit. For larger data: 1) Call GenerateDataKey to get a plaintext and encrypted data key. 2) Encrypt data locally with the plaintext key. 3) Discard the plaintext key. 4) Store the encrypted data key with the ciphertext. Decryption reverses the process.

Q: What are the three types of server-side encryption for S3?
A: SSE-S3: Amazon manages keys, AES-256, simplest. SSE-KMS: AWS KMS manages keys, audit trail via CloudTrail, can use customer-managed CMK. SSE-C: customer provides the encryption key with each request, AWS performs encryption/decryption but doesn't store the key.

Q: How does STS AssumeRole work for cross-account access?
A: 1) Account B creates a role with a trust policy allowing Account A. 2) Account A's IAM entity calls sts:AssumeRole with the role ARN. 3) STS returns temporary credentials (access key, secret key, session token). 4) Use these credentials to access Account B's resources.

Q: What is the principle of least privilege and how do you implement it in Lambda?
A: Grant only the minimum permissions needed. For Lambda: create a dedicated execution role per function, scope actions to specific operations (e.g., dynamodb:GetItem not dynamodb:*), scope resources to specific ARNs (not *), add conditions where possible.

Q: How do API Gateway authorizers work?
A: Lambda authorizer: API Gateway calls your Lambda with the token/request, Lambda returns an IAM policy (cached by token for TTL). Cognito authorizer: API Gateway validates the JWT against the user pool directly (no custom code). IAM auth: client signs request with SigV4.

Q: What is the difference between Secrets Manager and Parameter Store for storing secrets?
A: Secrets Manager: built-in automatic rotation via Lambda, $0.40/secret/month, designed for credentials. Parameter Store SecureString: encrypted with KMS, free (standard tier), no built-in rotation. Use Secrets Manager when you need automatic rotation; Parameter Store for config values and simple secrets.

Q: How does CORS work with API Gateway and Lambda proxy integration?
A: The browser sends an OPTIONS preflight request. API Gateway handles the OPTIONS response (configure CORS on the resource). For the actual request with proxy integration, the Lambda function must include Access-Control-Allow-Origin (and other CORS headers) in its response — API Gateway passes the response through unchanged.

Q: What is a resource-based policy vs an identity-based policy?
A: Identity-based: attached to IAM users/roles, says "this identity can do X on Y." Resource-based: attached to resources (S3 bucket, Lambda, SQS), says "who can do X on this resource." Resource-based policies enable cross-account access without assuming a role.

Q: How does DynamoDB fine-grained access control work?
A: Use IAM policy conditions on dynamodb:LeadingKeys to restrict access based on partition key values. Example: a policy condition that only allows access to items where the partition key matches the authenticated user's ID. Works with Cognito identity pool credentials.

Q: What is AWS Signature Version 4 (SigV4)?
A: The signing process for authenticating AWS API requests. Steps: 1) Create a canonical request. 2) Create a string to sign (date, region, service). 3) Calculate the signature using HMAC-SHA256 with your secret key. 4) Add the signature to the Authorization header. AWS SDKs handle this automatically.

## Deployment

Q: What is the difference between SAM and CloudFormation?
A: SAM is a superset of CloudFormation with shorthand syntax for serverless resources (AWS::Serverless::Function, etc.). SAM templates are transformed into full CloudFormation templates during deployment. SAM also provides a CLI for local testing (sam local invoke/start-api).

Q: What are the Elastic Beanstalk deployment policies and their tradeoffs?
A: All at once: fastest, causes downtime. Rolling: updates in batches, reduced capacity during deployment. Rolling with additional batch: maintains full capacity, slower. Immutable: new ASG, zero downtime, easy rollback, most expensive. Traffic splitting: canary-style, routes percentage to new instances.

Q: How does CodeDeploy Blue/Green deployment work for ECS?
A: 1) CodeDeploy creates a new task set (green) with the new version. 2) Traffic is shifted from the old target group (blue) to the new target group on the ALB. 3) Shift can be all-at-once, canary, or linear. 4) If health checks fail, traffic shifts back to blue automatically.

Q: What is the difference between buildspec.yml and appspec.yml?
A: buildspec.yml: CodeBuild — defines build phases (install, pre_build, build, post_build), commands, and output artifacts. appspec.yml: CodeDeploy — defines deployment lifecycle hooks (ApplicationStop, BeforeInstall, AfterInstall, ValidateService) and file mappings.

Q: How do CloudFormation exports and imports work?
A: A stack exports output values with Export: { Name: "ExportName" }. Other stacks in the same region import with Fn::ImportValue: "ExportName". Creates a dependency — the exporting stack cannot be deleted while another stack imports its values.

Q: What are Lambda deployment strategies with CodeDeploy?
A: AllAtOnce: shift all traffic immediately. Canary: shift X% first, then 100% after Y minutes (e.g., Canary10Percent5Minutes). Linear: shift X% every Y minutes (e.g., Linear10PercentEvery1Minute). CodeDeploy monitors CloudWatch alarms and rolls back automatically on failure.

Q: How does Elastic Beanstalk .ebextensions work?
A: YAML/JSON config files in the .ebextensions/ directory of your source bundle. Can install packages, create files, run commands, configure services, set environment properties, and create AWS resources. Processed in alphabetical order during deployment.

Q: What is a CloudFormation custom resource?
A: Enables running custom provisioning logic (Lambda function or SNS topic) during stack create, update, and delete. CloudFormation sends a request to the Lambda/SNS, which performs the work and sends a SUCCESS/FAILED response to a pre-signed S3 URL.

Q: How does CodePipeline orchestrate CI/CD?
A: Pipeline stages: Source (CodeCommit, GitHub, S3) → Build (CodeBuild) → Test → Approval (manual) → Deploy (CodeDeploy, CloudFormation, ECS, Elastic Beanstalk). Artifacts pass between stages via S3. Each stage has one or more actions that run sequentially or in parallel.

Q: What is the difference between ECS rolling update and Blue/Green deployment?
A: Rolling update: ECS gradually replaces old tasks with new ones, maintaining minimum healthy percent. Managed by ECS directly. Blue/Green: CodeDeploy creates a new task set, shifts ALB traffic, supports canary/linear shifting and automatic rollback. Blue/Green gives more control.

Q: How does SAM handle local development and testing?
A: sam local invoke: runs a single Lambda invocation in a Docker container. sam local start-api: starts a local API Gateway endpoint. sam local start-lambda: starts a local Lambda endpoint. All use Docker to simulate the Lambda runtime environment.

Q: What is CloudFormation drift detection?
A: Compares the actual state of stack resources with the expected state defined in the template. Detects manual changes made outside CloudFormation (e.g., someone modified a security group in the console). Does not fix drift — only reports it.

## Troubleshooting and Optimization

Q: How does X-Ray trace a request across distributed services?
A: X-Ray uses the X-Amzn-Trace-Id header to propagate trace context. Each service creates segments (top-level) and subsegments (downstream calls). The X-Ray daemon collects segments and sends them to the X-Ray service, which assembles the full trace and service map.

Q: What causes a 502 Bad Gateway from API Gateway with Lambda?
A: The Lambda function returned a malformed response. With proxy integration, the response must include statusCode (integer) and body (string). Common causes: returning an object instead of a string for body, missing statusCode, unhandled exception in the function.

Q: How do you troubleshoot Lambda cold starts?
A: 1) Check CloudWatch Logs for Init Duration in REPORT lines. 2) Reduce deployment package size (use layers, remove unused dependencies). 3) Choose a lighter runtime (Python/Node.js vs Java). 4) Increase memory (faster CPU = faster init). 5) Use provisioned concurrency for latency-sensitive functions.

Q: What is the difference between CloudWatch Logs, Metrics, and Alarms?
A: Logs: text-based log data from services (Lambda, ECS, EC2). Metrics: time-series numerical data (CPU, invocations, errors). Alarms: monitor a metric and trigger actions when a threshold is breached. Use Logs Insights for ad-hoc queries, metric filters to extract metrics from logs.

Q: How do you handle DynamoDB hot partitions?
A: 1) Redesign the partition key for higher cardinality (add random suffix, composite key). 2) Use write sharding (distribute writes across multiple partition key values). 3) Use DAX for read-heavy hot keys. 4) Switch to on-demand capacity (adaptive capacity distributes throughput more flexibly).

Q: What does the Lambda REPORT line in CloudWatch Logs tell you?
A: Duration (execution time), Billed Duration (rounded up to nearest ms), Memory Size (configured), Max Memory Used (actual peak), Init Duration (cold start time, only on cold starts). If Max Memory Used equals Memory Size, the function is running out of memory.

Q: How do you use CloudWatch Logs Insights for troubleshooting?
A: SQL-like query language across log groups. Example: fields @timestamp, @message | filter @message like /ERROR/ | sort @timestamp desc | limit 20. Supports stats (count, avg, sum), parse for extracting fields, and visualization.

Q: What are X-Ray annotations vs metadata?
A: Annotations: indexed key-value pairs (string, number, boolean). Use for filtering traces in the X-Ray console (e.g., annotation.user_id = "123"). Metadata: non-indexed, any JSON object. Use for recording debug data that doesn't need to be searchable.

Q: How do you optimize Lambda function performance?
A: 1) Increase memory (proportionally increases CPU). 2) Reuse execution context (initialize SDK clients outside handler). 3) Minimize deployment package size. 4) Use provisioned concurrency for consistent latency. 5) Use RDS Proxy for database connections. 6) Use /tmp for caching between invocations.

Q: What causes SQS messages to end up in a dead-letter queue?
A: Messages that exceed the maxReceiveCount (number of times a message is received but not deleted). This happens when the consumer fails to process the message and the visibility timeout expires, making it available again. After maxReceiveCount failures, SQS moves it to the DLQ.

Q: How do you troubleshoot ECS Fargate tasks that fail to start?
A: 1) Check the task execution role (needs ECR, CloudWatch Logs permissions). 2) Check network — private subnet needs NAT gateway or VPC endpoints for ECR/S3/CloudWatch. 3) Check container logs in CloudWatch. 4) Check the task definition (image URI, port mappings, resource limits).
