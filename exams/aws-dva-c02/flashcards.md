## SQS (Simple Queue Service)

Q: SQS long polling
A: Allows the SQS service to wait until a message is available in the queue before sending a response. Reduces empty responses and API calls. Maximum long poll timeout is 20 seconds.

Q: SQS visibility timeout
A: A period during which SQS prevents other consumers from receiving and processing a message. Default is 30 seconds. Use ChangeMessageVisibility API to extend it if processing takes longer.

Q: SQS maximum long poll timeout
A: 20 seconds. Set via the WaitTimeSeconds parameter on ReceiveMessage or the queue's ReceiveMessageWaitTimeSeconds attribute.

Q: EC2 fleet constantly polling empty SQS queues burning CPU — what to do?
A: Enable SQS long polling. This eliminates empty responses and reduces wasted CPU cycles and API costs.

Q: ChangeMessageVisibility API
A: Extends the visibility timeout for a message currently being processed. Use when processing takes longer than the default visibility timeout.

Q: SQS message size limit
A: 256 KB of text in any format. For larger payloads, use the Extended Client Library to store the payload in S3 and send a reference in the SQS message.

Q: SQS — was it the first AWS service?
A: Yes, SQS was the first publicly available AWS service (launched in 2004).

Q: SQS standard vs FIFO queue
A: Standard: unlimited throughput, at-least-once delivery, best-effort ordering. FIFO: 300 msg/sec (3,000 with batching), exactly-once processing, strict ordering. FIFO queue names must end in .fifo.

Q: SQS dead-letter queue (DLQ)
A: A queue that receives messages that failed processing after a configured number of attempts (maxReceiveCount). Used to isolate problem messages for debugging.

Q: SQS delay queue
A: Postpones delivery of new messages for a configurable period (0–900 seconds). Messages are invisible to consumers during the delay period.

Q: SQS FIFO deduplication
A: FIFO queues prevent duplicate messages within a 5-minute deduplication interval. Use content-based deduplication (SHA-256 hash of body) or provide a MessageDeduplicationId.

Q: SQS message group ID
A: Used in FIFO queues to guarantee ordering within a group. Messages with different group IDs can be processed in parallel. Required for FIFO queues.

Q: How to fan out SQS messages to multiple queues?
A: Use SNS. Publish to an SNS topic with multiple SQS queue subscriptions. Each queue receives a copy of every message.

Q: SQS priority — payment processing vs promotional emails
A: Use 2 SQS queues. Program consumers to poll the payment (high-priority) queue first. Only process the email queue when the payment queue is empty.

Q: Can SQS messages be delivered multiple times?
A: Yes, standard SQS queues guarantee at-least-once delivery. Duplicates are possible. Use FIFO queues for exactly-once processing or implement idempotency.

## SNS (Simple Notification Service)

Q: SNS push or pull?
A: Push-based. SNS pushes messages to subscribers immediately when published to a topic.

Q: SNS supported protocols
A: Amazon SQS, HTTP/S, email, SMS, Lambda. Does NOT support FPS.

Q: SNS message filtering
A: Subscription filter policies allow subscribers to receive only messages matching specific attribute criteria, avoiding processing irrelevant messages.

Q: Can SNS messages be customized per protocol?
A: Yes, you can publish different message content for each protocol (e.g., different text for SMS vs email) in a single publish call.

Q: SNS fan-out to SQS
A: Publish to an SNS topic subscribed by multiple SQS queues. Each queue gets a copy. This decouples the publisher from consumers and enables parallel processing.

Q: SNS topic
A: A communication channel for publishing messages. Subscribers (SQS, Lambda, HTTP, email, SMS) receive messages published to the topic.

## DynamoDB

Q: DynamoDB — what type of database?
A: NoSQL key-value and document database. Fully managed, serverless, single-digit millisecond latency at any scale.

Q: DynamoDB write capacity unit (WCU)
A: 1 WCU = one write per second for items up to 1 KB. Items larger than 1 KB require additional WCUs (rounded up).

Q: DynamoDB read capacity unit (RCU)
A: 1 RCU = one strongly consistent read per second for items up to 4 KB, OR two eventually consistent reads per second for items up to 4 KB.

Q: DynamoDB consistency models
A: Eventually consistent reads (default, may return stale data) and strongly consistent reads (always returns latest data, costs 2x the RCU).

Q: DynamoDB write throughput calculation steps
A: 1) Calculate items per second. 2) Calculate WCU per item (item size in KB, rounded up). 3) Multiply items/sec × WCU per item.

Q: DynamoDB read throughput calculation steps
A: 1) Calculate items per second. 2) Calculate RCU per item (item size / 4 KB, rounded up). 3) Multiply items/sec × RCU per item. 4) If eventually consistent, divide by 2.

Q: 600 items/min, 5 KB each — write throughput?
A: 10 items/sec × 5 WCU per item = 50 WCU.

Q: 600 items/min, 5 KB each, strongly consistent — read throughput?
A: 10 items/sec × 2 RCU per item (5 KB rounds up to 8 KB = 2 × 4 KB) = 20 RCU.

Q: 600 items/min, 5 KB each, eventually consistent — read throughput?
A: 10 items/sec × 2 RCU per item = 20 RCU for strongly consistent. Divide by 2 for eventually consistent = 10 RCU.

Q: 25 items/sec, 13 KB each, strongly consistent — read throughput?
A: 13 KB / 4 KB = 3.25, rounds up to 4 RCU per item. 25 × 4 = 100 RCU.

Q: 25 items/sec, 13 KB each, eventually consistent — read throughput?
A: 100 RCU (strongly consistent) / 2 = 50 RCU.

Q: DynamoDB BatchGetItem
A: Retrieves multiple items from one or more tables in a single API call. Up to 100 items or 16 MB total. More efficient than individual GetItem calls.

Q: DynamoDB BatchWriteItem
A: Writes or deletes up to 25 items across one or more tables in a single call. NOT transactional — individual operations can fail independently.

Q: DynamoDB TransactWriteItems
A: ACID transaction across up to 100 items in one or more tables. All operations succeed or all fail. Costs 2x the WCU of normal writes.

Q: DynamoDB TransactGetItems
A: Reads up to 100 items atomically across tables. Costs 2x the RCU of normal reads. Guarantees a consistent snapshot.

Q: DynamoDB Query vs Scan
A: Query finds items by partition key (and optional sort key condition) — efficient. Scan reads every item in the table — expensive. Always prefer Query.

Q: DynamoDB ProvisionedThroughputExceededException
A: You exceeded the provisioned throughput for the table or index. Implement exponential backoff, consider auto-scaling, or switch to on-demand capacity.

Q: DynamoDB local secondary index (LSI)
A: Alternate sort key on the same partition key. Must be created at table creation time. Maximum 5 per table. Shares the table's provisioned throughput.

Q: DynamoDB global secondary index (GSI)
A: Alternate partition key and optional sort key. Can be created anytime. Has its own provisioned throughput separate from the base table. Maximum 20 per table.

Q: DynamoDB on-demand capacity
A: Pay-per-request pricing. No capacity planning needed. Scales instantly. Good for unpredictable workloads. More expensive per request than provisioned at steady state.

Q: DynamoDB auto-scaling
A: Automatically adjusts provisioned capacity based on actual traffic patterns using target utilization percentage. Uses Application Auto Scaling under the hood.

Q: DynamoDB TTL (Time to Live)
A: Automatically deletes expired items based on a timestamp attribute. Deletion happens within 48 hours of expiration (not instant). No additional cost.

Q: DynamoDB Streams
A: Captures item-level changes (insert, update, delete) in near real-time. Can trigger Lambda functions. Retains data for 24 hours. Used for replication, analytics, and event-driven architectures.

Q: DynamoDB DAX (DynamoDB Accelerator)
A: In-memory cache for DynamoDB. Microsecond read latency. Works ONLY with DynamoDB. Drop-in replacement — no application code changes needed for reads.

Q: DynamoDB optimistic locking
A: Use a version number attribute with conditional writes. The update succeeds only if the version matches. Prevents lost updates in concurrent scenarios.

Q: DynamoDB item size limit
A: 400 KB maximum per item. For larger data, store the object in S3 and keep a reference (S3 URL) in DynamoDB.

Q: DynamoDB partition key design best practice
A: Choose a high-cardinality attribute that distributes requests evenly. Avoid hot partitions. Examples: user_id, order_id. Bad: status, date.

Q: Can you select a specific AZ for a DynamoDB table?
A: No. DynamoDB is a regional service that automatically replicates across multiple AZs. You cannot choose a specific AZ.

Q: DynamoDB global tables
A: Multi-region, multi-active replication. Writes in any region are replicated to all other regions. Requires DynamoDB Streams enabled.

## Lambda

Q: Lambda maximum timeout
A: 900 seconds (15 minutes). Default is 3 seconds. Set via the Timeout configuration.

Q: Lambda deployment package size limits
A: ZIP: 50 MB compressed, 250 MB uncompressed. Container image: 10 GB. Use layers to share common dependencies.

Q: Lambda execution context reuse
A: Lambda reuses the execution environment for subsequent invocations. Code outside the handler (global scope) runs once and persists across invocations. Use this for DB connections, SDK clients.

Q: Lambda cold start
A: First invocation requires downloading code, starting the runtime, and running initialization. Adds latency (100ms–10s depending on runtime and package size). Mitigate with provisioned concurrency.

Q: Lambda provisioned concurrency
A: Pre-initializes a specified number of execution environments. Eliminates cold starts. You pay for the provisioned environments whether they're used or not.

Q: Lambda reserved concurrency
A: Guarantees a maximum number of concurrent executions for a function. Also acts as a throttle — invocations beyond this limit are throttled.

Q: Lambda layers
A: ZIP archives containing libraries, custom runtimes, or data. Up to 5 layers per function. Shared across functions. Reduces deployment package size.

Q: Lambda environment variables
A: Key-value pairs available to the function at runtime. Encrypted at rest with KMS. Can use a customer-managed KMS key for additional encryption.

Q: Lambda destinations
A: Route asynchronous invocation results to SQS, SNS, Lambda, or EventBridge. Configure separate destinations for success and failure. More flexible than DLQs.

Q: Lambda event source mapping
A: Polls stream/queue sources (Kinesis, DynamoDB Streams, SQS) and invokes the function with batches of records. Managed by the Lambda service.

Q: Lambda in a VPC
A: Lambda can access VPC resources (RDS, ElastiCache) by configuring VPC subnets and security groups. Needs a NAT gateway for internet access from private subnets.

Q: Lambda /tmp storage
A: Up to 10 GB of ephemeral storage in /tmp. Persists across invocations in the same execution environment. Use for temporary file processing.

Q: Lambda concurrency limit
A: 1,000 concurrent executions per region by default (soft limit, can be increased). Shared across all functions in the account unless reserved concurrency is set.

Q: Lambda supported runtimes
A: Python, Node.js, Java, .NET, Go, Ruby, and custom runtimes via the Runtime API. Container images can use any runtime.

Q: Lambda@Edge vs CloudFront Functions
A: Lambda@Edge runs at regional edge caches (up to 5 sec, 10 MB). CloudFront Functions run at all edge locations (sub-millisecond, 2 MB, viewer request/response only). Use CloudFront Functions for simple transformations.

## API Gateway

Q: API Gateway REST vs HTTP API
A: REST API: full-featured (caching, request validation, WAF, usage plans). HTTP API: lower latency, lower cost, simpler, supports JWT authorizers natively. Use HTTP API unless you need REST-specific features.

Q: API Gateway Lambda proxy integration
A: API Gateway passes the entire request to Lambda as an event object. Lambda must return statusCode, headers, and body. No request/response mapping templates needed.

Q: API Gateway stage variables
A: Name-value pairs associated with a deployment stage. Use to configure different backends per stage (e.g., dev Lambda alias vs prod Lambda alias).

Q: API Gateway caching
A: Caches endpoint responses for a configurable TTL (0–3600 seconds). Reduces backend invocations. Configured per stage. Cache sizes from 0.5 GB to 237 GB.

Q: API Gateway throttling
A: Default: 10,000 requests/sec per region, 5,000 burst. Per-client throttling via usage plans and API keys. Returns 429 Too Many Requests when throttled.

Q: API Gateway usage plans and API keys
A: Usage plans set throttle and quota limits. API keys identify clients and associate them with usage plans. API keys are NOT a security mechanism — use authorizers for auth.

Q: API Gateway CORS
A: For REST APIs, enable CORS on the resource. For proxy integrations, the Lambda function must return Access-Control-Allow-Origin headers. The OPTIONS preflight is handled by API Gateway.

Q: API Gateway authorizer types
A: Lambda authorizer (custom auth logic), Cognito user pool authorizer (JWT validation), IAM authorization (SigV4 signing). HTTP APIs also support JWT authorizers natively.

Q: API Gateway WebSocket API
A: Supports persistent bidirectional connections. Routes: $connect, $disconnect, $default, and custom routes. Use for chat, real-time dashboards, gaming.

## S3

Q: S3 maximum object size
A: 5 TB. Single PUT upload limit is 5 GB. Use multipart upload for objects larger than 5 GB (recommended for objects > 100 MB).

Q: S3 storage — unlimited?
A: Yes, S3 provides unlimited total storage. Individual objects are limited to 5 TB.

Q: S3 minimum object size
A: 0 bytes. There is no minimum (though S3 Standard-IA and other storage classes have a minimum billable size of 128 KB).

Q: S3 pre-signed URLs
A: Time-limited URLs granting temporary access to upload or download a specific object. Generated server-side using AWS credentials. Default expiration: 1 hour (configurable up to 7 days).

Q: S3 Transfer Acceleration
A: Speeds up uploads to S3 by routing through CloudFront edge locations. Uses optimized network paths. Does NOT help with downloads.

Q: S3 event notifications
A: Trigger Lambda, SQS, or SNS when objects are created, deleted, or restored. Configure with prefix/suffix filters to target specific keys.

Q: S3 CORS (Cross-Origin Resource Sharing)
A: Must be enabled on the bucket when a web page in one domain needs to access S3 resources in another domain. Configured via CORS rules on the bucket.

Q: S3 default encryption
A: AES-256 (SSE-S3) by default. Can be configured to use SSE-KMS with a specific key. Applies to all new objects that don't specify encryption.

Q: S3 versioning
A: Keeps multiple versions of an object. Protects against accidental deletes (creates a delete marker). Once enabled, can only be suspended, not disabled.

Q: S3 consistency model (current)
A: Strong read-after-write consistency for all operations (PUTs, DELETEs, list operations) as of December 2020. No more eventual consistency.

Q: Are newly created S3 buckets public by default?
A: No. All new buckets are private by default. S3 Block Public Access is enabled by default at the account level.

Q: S3 static website hosting URL format
A: http://BUCKET-NAME.s3-website-REGION.amazonaws.com (no www).

Q: HTTP 200 response from S3
A: Indicates a successful upload (PUT) or retrieval (GET). A 200 after a PUT confirms the object was stored.

## Kinesis

Q: Kinesis Data Streams
A: Real-time data streaming. Data retained 24 hours (up to 365 days). Consumers: Lambda, KCL applications, Kinesis Data Analytics. You manage shard capacity.

Q: Kinesis Data Firehose
A: Near real-time delivery (60-second minimum buffer). Fully managed, no shards. Delivers to S3, Redshift, OpenSearch, Splunk, HTTP endpoints. Can transform data with Lambda.

Q: Kinesis shard capacity
A: Each shard: 1 MB/sec or 1,000 records/sec write, 2 MB/sec read. Scale by adding/removing shards (resharding).

Q: Kinesis enhanced fan-out
A: Dedicated 2 MB/sec throughput per consumer per shard (push model via SubscribeToShard). Without it, all consumers share the 2 MB/sec per shard (pull model).

Q: Kinesis vs SQS
A: Kinesis: real-time streaming, ordered within shard, multiple consumers, data replay. SQS: message queue, decoupling, at-least-once delivery, no replay after deletion.

## Cognito

Q: Cognito user pools
A: User directory for sign-up/sign-in. Issues JWT tokens (ID, access, refresh). Supports MFA, password policies, social/SAML federation. Handles authentication.

Q: Cognito identity pools (federated identities)
A: Exchanges tokens (from user pools, social providers, SAML) for temporary AWS credentials via STS. Handles authorization to AWS resources.

Q: Cognito user pool vs identity pool
A: User pool = authentication (who are you?). Identity pool = authorization (what AWS resources can you access?). Often used together: user pool authenticates, identity pool grants AWS access.

Q: Cognito Lambda triggers
A: Custom logic at various auth flow points: pre-sign-up, post-confirmation, pre-authentication, post-authentication, pre-token generation, custom message, user migration.

## Step Functions

Q: Step Functions — what is it?
A: Serverless workflow orchestration. Define workflows as state machines in JSON (Amazon States Language). Supports sequential, parallel, branching, error handling, and human approval steps.

Q: Step Functions Standard vs Express
A: Standard: up to 1 year duration, exactly-once execution, $0.025 per 1,000 transitions. Express: up to 5 minutes, at-least-once, priced by invocations and duration. Use Express for high-volume, short-duration workflows.

Q: Step Functions task types
A: Activity tasks (external workers poll for work), Lambda tasks (invoke Lambda), service integrations (call AWS services directly like DynamoDB, SQS, SNS, ECS).

## EventBridge

Q: Amazon EventBridge
A: Serverless event bus. Routes events from AWS services, SaaS apps, and custom sources to targets using rules with event pattern matching. Successor to CloudWatch Events.

Q: EventBridge event rules
A: Match incoming events by pattern (source, detail-type, detail fields) and route to one or more targets (Lambda, SQS, SNS, Step Functions, etc.).

Q: EventBridge vs SNS
A: EventBridge: content-based filtering on event body, schema registry, more targets, SaaS integration. SNS: simpler pub/sub, higher throughput, message filtering on attributes only.

## Other Services

Q: EC2 instance metadata endpoint
A: http://169.254.169.254/latest/meta-data/ — retrieves instance metadata (public IP, private IP, IAM role, instance type, etc.) from within the instance.

Q: EC2 instance store vs EBS
A: Instance store: ephemeral, physically attached, highest IOPS, data lost on stop/terminate. EBS: persistent, network-attached, survives stop/start, supports snapshots.

Q: EBS encryption
A: Configure encryption when creating the volume. Uses AES-256 with KMS keys. Encrypts data at rest, in transit between instance and volume, and all snapshots.

Q: Elastic Beanstalk — is it free?
A: Yes, Elastic Beanstalk itself is free. You pay only for the underlying resources it provisions (EC2, ELB, RDS, etc.). Same for CloudFormation.

Q: Elastic Beanstalk supported platforms
A: Java, .NET, PHP, Node.js, Python, Ruby, Go, Docker. Supports single-instance and load-balanced environments.

Q: ElastiCache — what is it?
A: Managed in-memory cache. Two engines: Redis (persistence, replication, pub/sub, complex data types) and Memcached (simpler, multi-threaded, no persistence).

Q: ElastiCache use case for developers
A: Store session data, cache database query results, reduce database load. Sub-millisecond latency.

Q: Redis vs Memcached
A: Redis: persistence, replication, pub/sub, sorted sets, geospatial. Memcached: simpler, multi-threaded, no persistence. Redis is almost always the exam answer.

Q: AWS X-Ray
A: Distributed tracing service. Visualizes request flow across services. Identifies performance bottlenecks and errors. Uses the X-Amzn-Trace-Id header for trace propagation.

Q: CloudWatch Logs
A: Centralized log storage and analysis. Lambda, ECS, EC2 (via agent), API Gateway all send logs here. Use Logs Insights for SQL-like queries across log groups.

Q: CloudWatch Metrics
A: Time-series data for AWS resources. Standard metrics (free, 5-min intervals) and detailed monitoring (1-min intervals). Custom metrics via PutMetricData API.

Q: CloudWatch Alarms
A: Monitor a metric and trigger actions (SNS, Auto Scaling, EC2) when a threshold is breached. States: OK, ALARM, INSUFFICIENT_DATA.

Q: AWS SDK default region
A: us-east-1. Always explicitly set the region in your SDK configuration to avoid unexpected behavior.

Q: AWS SDK supported languages
A: Python (boto3), JavaScript/Node.js, Java, .NET, Go, Ruby, PHP, C++, Rust, Swift, Kotlin.

Q: Route 53 — why the name?
A: DNS uses port 53. Route 53 is Amazon's scalable DNS service.

Q: Amazon Redshift
A: Columnar data warehouse for OLAP (Online Analytics Processing). Petabyte-scale. NOT for transactional workloads (OLTP). Use RDS/Aurora for OLTP.

Q: OLTP vs OLAP
A: OLTP (Online Transaction Processing): frequent, simple queries (RDS, Aurora, DynamoDB). OLAP (Online Analytics Processing): complex analytical queries on large datasets (Redshift).

Q: Amazon Aurora
A: AWS proprietary relational database. MySQL and PostgreSQL compatible. Up to 5x MySQL and 3x PostgreSQL performance. Auto-scales storage up to 128 TB.

Q: AWS AppSync
A: Managed GraphQL service. Real-time subscriptions via WebSockets. Offline data sync for mobile. Integrates with DynamoDB, Lambda, HTTP data sources.

Q: Amazon SWF (Simple Workflow Service)
A: Coordinates tasks across distributed components. Supports long-running human tasks. Uses workers (process tasks) and deciders (control coordination). Largely replaced by Step Functions for new applications.

Q: Web Identity Federation
A: Allows users to authenticate with social identity providers (Google, Facebook, Amazon) and receive temporary AWS credentials. Uses AssumeRoleWithWebIdentity API call.

Q: SAML federation with AWS
A: 1) User authenticates with ADFS. 2) Receives SAML assertion. 3) Posts assertion to AWS SAML endpoint (https://signin.aws.amazon.com/saml). 4) AssumeRoleWithSAML returns temporary credentials. 5) User accesses AWS console.

Q: AssumeRoleWithWebIdentity
A: STS API call that returns temporary credentials when authenticating with a web identity provider (social login). Used in Web Identity Federation.

Q: AssumeRoleWithSAML
A: STS API call that returns temporary credentials when federating with Active Directory via SAML 2.0.

Q: IAM — what does it manage?
A: Users, groups, roles, and their access to AWS resources. Provides centralized control, integrates with Active Directory, fine-grained access control. Does NOT do biometric auth.

Q: IAM best practice for EC2
A: Use IAM roles (instance profiles), never hardcoded access keys. Roles provide temporary credentials via the instance metadata service.

Q: CloudFormation template language
A: JSON or YAML. Defines AWS resources declaratively. Free to use; you pay for provisioned resources.

Q: CloudFormation Fn::GetAtt
A: Intrinsic function that returns the value of an attribute from a resource. Example: Fn::GetAtt for an ELB returns its DNSName.

Q: CloudFormation rollback behavior
A: On failure, CloudFormation rolls back and deletes all resources created during the failed operation by default. Use --disable-rollback for debugging.

Q: VPC — how many per region by default?
A: 5 VPCs per region (soft limit, can be increased).

Q: Internet gateways per VPC
A: 1 internet gateway per VPC. Cannot attach multiple.

Q: Security groups vs Network ACLs
A: Security groups: stateful, instance-level, allow rules only. NACLs: stateless, subnet-level, allow and deny rules with numbered priority.

Q: Availability Zone vs Region
A: Region = geographic area (e.g., us-east-1). AZ = isolated data center(s) within a region (e.g., us-east-1a). Regions contain multiple AZs.
