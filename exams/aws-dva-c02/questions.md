# AWS DVA-C02 Practice Exam — 100 Questions

Weighted by exam domain:
- Domain 1: Development with AWS Services (Q1–Q32) — 32%
- Domain 2: Security (Q33–Q58) — 26%
- Domain 3: Deployment (Q59–Q82) — 24%
- Domain 4: Troubleshooting and Optimization (Q83–Q100) — 18%

Passing target: 720/1000 (~72%). Aim for 85%+ in practice.

---

## Domain 1: Development with AWS Services (Questions 1–32)

**Q1.** A developer is building a serverless application that processes images uploaded to S3. The function must run for up to 10 minutes per image. Which compute service should the developer use?

A) AWS Lambda with default timeout
B) AWS Lambda with timeout set to 600 seconds
C) Amazon ECS with Fargate
D) Amazon EC2 with Auto Scaling

**Q2.** A developer needs to store user session data that expires automatically after 24 hours. The data must be accessible with sub-millisecond latency. Which service is MOST appropriate?

A) Amazon DynamoDB with TTL enabled
B) Amazon RDS with a scheduled cleanup job
C) Amazon ElastiCache for Redis with TTL on keys
D) Amazon S3 with lifecycle policies

**Q3.** An application writes 600 items per minute to a DynamoDB table. Each item is 5 KB. What is the minimum write capacity units (WCU) needed?

A) 10
B) 50
C) 100
D) 600

**Q4.** A developer is using the AWS SDK to make API calls and receives intermittent throttling errors. What is the recommended approach?

A) Increase the timeout value in the SDK configuration
B) Implement exponential backoff with jitter
C) Switch to synchronous API calls
D) Create a new IAM user with higher rate limits

**Q5.** A developer needs to retrieve multiple items from a DynamoDB table in a single API call using a list of known primary keys. Which API call should they use?

A) Scan with a filter expression
B) Query with a key condition expression
C) BatchGetItem
D) GetItem in a loop

**Q6.** A Lambda function processes messages from an SQS queue. The function occasionally fails and the same message is processed multiple times. How should the developer prevent duplicate processing?

A) Increase the SQS visibility timeout
B) Enable SQS long polling
C) Implement idempotency in the Lambda function
D) Use SQS short polling

**Q7.** A developer needs to send a single message to multiple SQS queues simultaneously. Which architecture should they use?

A) Write to each SQS queue individually from the application
B) Use an SNS topic with SQS queue subscriptions
C) Use SQS message groups
D) Use Amazon EventBridge with SQS targets

**Q8.** An application reads 25 items per second from DynamoDB. Each item is 13 KB. The application uses eventually consistent reads. What is the minimum read capacity units (RCU) needed?

A) 25
B) 50
C) 100
D) 200

**Q9.** A developer is building a REST API using API Gateway and Lambda. The API must support request validation before invoking the Lambda function. How should this be implemented?

A) Add validation logic in the Lambda function
B) Configure API Gateway request validators with a JSON schema model
C) Use a Lambda authorizer to validate requests
D) Add a WAF rule to validate request bodies

**Q10.** A developer needs to process DynamoDB table changes in near real-time to update a search index. Which feature should they use?

A) DynamoDB Accelerator (DAX)
B) DynamoDB Streams with a Lambda trigger
C) DynamoDB on-demand backups
D) DynamoDB global tables

**Q11.** A Lambda function needs to access a relational database in a private VPC subnet. The function experiences connection exhaustion under load. What should the developer do?

A) Increase the Lambda function memory
B) Use Amazon RDS Proxy
C) Move the database to a public subnet
D) Increase the Lambda function timeout

**Q12.** A developer is building an application that must process messages in strict FIFO order and ensure each message is delivered exactly once. Which SQS configuration should they use?

A) Standard queue with deduplication logic in the consumer
B) FIFO queue with content-based deduplication
C) Standard queue with long polling enabled
D) FIFO queue with a visibility timeout of 0

**Q13.** An application uses API Gateway with a Lambda proxy integration. The developer needs to return a custom HTTP status code and headers. How should the Lambda function respond?

A) Throw an exception with the status code
B) Return an object with statusCode, headers, and body fields
C) Configure the status code in the API Gateway method response
D) Set the status code in the Lambda execution context

**Q14.** A developer needs to store 500 MB objects and retrieve them with low latency. The objects are written once and read frequently. Which storage solution is MOST cost-effective?

A) Amazon S3 Standard
B) Amazon DynamoDB
C) Amazon EFS
D) Amazon EBS

**Q15.** A developer is using SQS and needs to ensure that consumers do not receive a message that is currently being processed by another consumer. What should they configure?

A) Message retention period
B) Visibility timeout
C) Delay queue
D) Dead-letter queue

**Q16.** A Lambda function is triggered by an S3 PUT event. The function processes the object and writes results back to the SAME bucket. The developer notices the function is being invoked in an infinite loop. What is the cause and fix?

A) The Lambda function timeout is too short; increase it
B) The S3 event triggers on the output write; use a prefix/suffix filter or a different bucket
C) The Lambda concurrency limit is too low; increase it
D) The S3 event notification is misconfigured; switch to SNS

**Q17.** A developer needs to implement a GraphQL API with real-time subscriptions for a mobile application. Which AWS service should they use?

A) Amazon API Gateway REST API
B) Amazon API Gateway WebSocket API
C) AWS AppSync
D) Amazon SNS

**Q18.** An application uses DynamoDB and needs to read 10 items per second, each 8 KB, using strongly consistent reads. What is the minimum RCU needed?

A) 10
B) 20
C) 40
D) 80

**Q19.** A developer is building a step-by-step order processing workflow that includes a manual approval step. Which AWS service is BEST suited for orchestrating this workflow?

A) Amazon SQS
B) AWS Step Functions
C) Amazon EventBridge
D) AWS Lambda

**Q20.** A developer needs to invoke a Lambda function asynchronously and ensure failed events are captured for later analysis. What should they configure?

A) A dead-letter queue on the SQS trigger
B) An on-failure destination on the Lambda function
C) CloudWatch Logs Insights
D) X-Ray tracing on the Lambda function

**Q21.** An application needs to read an item from DynamoDB and immediately update it based on the current value. The developer must ensure no other process modifies the item between the read and write. What should they use?

A) Strongly consistent reads followed by PutItem
B) Conditional writes using a version attribute (optimistic locking)
C) DynamoDB transactions with TransactWriteItems
D) BatchWriteItem with error handling

**Q22.** A developer is building a chat application that requires persistent WebSocket connections. Which AWS service supports WebSocket APIs?

A) Application Load Balancer
B) Amazon API Gateway
C) Amazon CloudFront
D) AWS AppSync only

**Q23.** A developer needs to read all items from a DynamoDB table for a one-time data export. The table has 50 GB of data. Which approach is MOST efficient?

A) Use Query with no key condition
B) Use Scan with parallel scan segments
C) Use BatchGetItem with all primary keys
D) Use GetItem in a loop for each partition key

**Q24.** A Lambda function needs to make an HTTPS call to an external API. The function runs in a VPC with private subnets only. The call is timing out. What should the developer do?

A) Increase the Lambda function timeout to 15 minutes
B) Add a NAT gateway to the VPC and update the route table
C) Attach an internet gateway to the private subnet
D) Remove the Lambda function from the VPC

**Q25.** A developer is using Amazon Kinesis Data Streams to ingest clickstream data. The stream has 4 shards. What is the maximum number of Lambda function invocations that can run concurrently for this stream?

A) 1
B) 4
C) 40
D) 1000

**Q26.** A developer needs to cache frequently accessed API Gateway responses to reduce Lambda invocations. What should they enable?

A) Lambda provisioned concurrency
B) API Gateway stage-level caching
C) CloudFront distribution in front of API Gateway
D) DynamoDB Accelerator (DAX)

**Q27.** An application uses S3 to store user uploads. The developer needs to allow users to upload files directly to S3 from a browser without routing through the application server. What should they use?

A) S3 bucket policy allowing public writes
B) Pre-signed URLs generated by the application
C) S3 Transfer Acceleration
D) Cross-origin resource sharing (CORS) only

**Q28.** A developer is writing a Lambda function that processes records from a Kinesis stream. If the function fails, the entire batch is retried. How can the developer prevent a single bad record from blocking the stream?

A) Increase the batch size
B) Configure bisect batch on function error
C) Decrease the parallelization factor
D) Enable enhanced fan-out

**Q29.** A developer needs to store application configuration values that change infrequently and should be accessible by multiple Lambda functions. Which service is MOST appropriate?

A) AWS Secrets Manager
B) AWS Systems Manager Parameter Store
C) Amazon S3
D) Environment variables in each Lambda function

**Q30.** A developer is building an event-driven architecture where an order service must notify inventory, shipping, and billing services when an order is placed. Which service provides loose coupling with filtering capabilities?

A) Amazon SQS
B) Amazon SNS with message filtering
C) Amazon EventBridge with event rules
D) Direct Lambda invocation

**Q31.** A developer needs to atomically write items to multiple DynamoDB tables as part of a single business operation. If any write fails, all writes must be rolled back. What should they use?

A) BatchWriteItem across tables
B) DynamoDB Streams with a Lambda function
C) TransactWriteItems
D) Conditional writes on each table

**Q32.** A developer is building a Lambda function that must initialize a database connection pool. The initialization takes 3 seconds. How can the developer minimize the impact on request latency?

A) Increase the Lambda function memory
B) Place the initialization code outside the handler function
C) Use provisioned concurrency
D) Both B and C

---

## Domain 2: Security (Questions 33–58)

**Q33.** A developer needs to store database credentials that are automatically rotated every 30 days. Which service should they use?

A) AWS Systems Manager Parameter Store SecureString
B) AWS Secrets Manager with automatic rotation
C) AWS KMS with key rotation
D) Environment variables encrypted with KMS

**Q34.** A Lambda function needs to read objects from an S3 bucket and write to a DynamoDB table. What is the MOST secure way to grant these permissions?

A) Embed IAM user access keys in the function code
B) Store access keys in environment variables
C) Attach an IAM execution role with least-privilege policies
D) Use the Lambda function's resource-based policy

**Q35.** A developer is building an API that must authenticate users with social identity providers (Google, Facebook) and issue temporary AWS credentials. Which service should they use?

A) AWS IAM Identity Center
B) Amazon Cognito identity pools (federated identities)
C) AWS STS AssumeRole
D) Amazon Cognito user pools only

**Q36.** An API Gateway REST API must only be accessible to authenticated users. The developer wants to use JWT tokens issued by Amazon Cognito. What is the simplest way to implement this?

A) Lambda authorizer that validates the JWT
B) API Gateway Cognito user pool authorizer
C) IAM authorization with SigV4
D) API key requirement on the API

**Q37.** A developer needs to encrypt application data before storing it in S3. The encryption must happen client-side and the developer must manage the encryption keys. Which approach should they use?

A) SSE-S3
B) SSE-KMS
C) SSE-C
D) Client-side encryption with a client-managed CMK

**Q38.** A developer is using AWS KMS to encrypt data. The data payload is 10 MB. What is the correct approach?

A) Call KMS Encrypt directly with the 10 MB payload
B) Use envelope encryption: generate a data key with GenerateDataKey, encrypt locally, store the encrypted data key
C) Split the data into 4 KB chunks and call KMS Encrypt for each
D) Use SSE-S3 instead since KMS cannot handle large payloads

**Q39.** A developer needs to grant a third-party application temporary access to an S3 bucket in their account. The access should expire after 1 hour. What should they use?

A) Create an IAM user and share the credentials
B) Use STS AssumeRole with a cross-account role and 1-hour session duration
C) Make the S3 bucket public temporarily
D) Share a pre-signed URL for each object

**Q40.** A Lambda function's environment variables contain a database connection string. The developer needs to encrypt this value at rest and decrypt it at runtime. What should they do?

A) Enable Lambda environment variable encryption with a KMS key and decrypt in the function code using the KMS API
B) Store the value in plaintext since Lambda encrypts environment variables by default
C) Use AWS Secrets Manager instead of environment variables
D) Both A and C are valid approaches

**Q41.** A developer is building a multi-tenant application. Each tenant's data is stored in DynamoDB with a tenant_id attribute. How should the developer ensure that users can only access their own tenant's data?

A) Use IAM policies with a condition key referencing the DynamoDB leading key
B) Implement access control in the application code only
C) Create a separate DynamoDB table per tenant
D) Use DynamoDB fine-grained access control with IAM policy conditions on dynamodb:LeadingKeys

**Q42.** An application running on EC2 needs to call AWS APIs. The developer finds hardcoded access keys in the application code. What is the MOST secure remediation?

A) Move the access keys to environment variables
B) Store the access keys in AWS Secrets Manager
C) Replace the access keys with an IAM instance profile (role)
D) Encrypt the access keys with KMS

**Q43.** A developer needs to sign API requests to AWS services manually (without the SDK). Which signing process must they implement?

A) Basic HTTP authentication
B) AWS Signature Version 4 (SigV4)
C) OAuth 2.0 client credentials
D) HMAC-SHA1 with the secret access key

**Q44.** A developer is using Amazon Cognito user pools for authentication. They need to add custom claims to the ID token before it is issued. What should they use?

A) Cognito identity pools
B) A pre-token generation Lambda trigger
C) API Gateway request transformation
D) A custom authorizer Lambda function

**Q45.** A developer needs to restrict API Gateway access so that only requests from a specific VPC can reach the API. What should they configure?

A) API Gateway resource policy with a VPC endpoint condition
B) API Gateway API keys
C) Lambda authorizer checking the source IP
D) WAF rules on the API Gateway

**Q46.** An application stores sensitive configuration in Systems Manager Parameter Store as SecureString parameters. Which service encrypts these values?

A) Amazon S3 server-side encryption
B) AWS KMS
C) AWS CloudHSM
D) The Parameter Store service itself without KMS

**Q47.** A developer needs to ensure that an S3 bucket only accepts requests over HTTPS. What should they add to the bucket policy?

A) A condition: `"aws:SecureTransport": "true"` with a Deny effect when false
B) Enable S3 Transfer Acceleration
C) Enable default encryption on the bucket
D) Configure the bucket for static website hosting with HTTPS

**Q48.** A developer is using Amazon Cognito user pools. After a user signs in, which token should the application use to access API Gateway with a Cognito authorizer?

A) Access token
B) ID token
C) Refresh token
D) Either access or ID token depending on the authorizer configuration

**Q49.** A developer needs to allow a Lambda function in Account A to access a DynamoDB table in Account B. What is the correct approach?

A) Create an IAM user in Account B and share credentials with Account A
B) Create a cross-account IAM role in Account B and have the Lambda function assume it using STS
C) Use a DynamoDB VPC endpoint in Account A
D) Enable DynamoDB global tables between the accounts

**Q50.** A developer is building an application that uses AWS STS to generate temporary credentials. What are the THREE components of temporary security credentials?

A) Access key ID, secret access key, and session token
B) Username, password, and MFA code
C) Access key ID, secret access key, and MFA code
D) Client ID, client secret, and authorization code

**Q51.** A developer needs to validate that all objects uploaded to an S3 bucket are encrypted with a specific KMS key. What should they configure?

A) S3 default encryption setting
B) A bucket policy with a condition on `s3:x-amz-server-side-encryption-aws-kms-key-id`
C) AWS Config rule for S3 encryption
D) CloudTrail logging for S3 PUT events

**Q52.** An application uses API Gateway with Lambda. The developer needs to throttle requests to 100 requests per second per client. What should they configure?

A) Lambda reserved concurrency set to 100
B) API Gateway usage plan with throttling and API keys
C) Application-level rate limiting in the Lambda function
D) CloudFront rate limiting

**Q53.** A developer is implementing CORS for an API Gateway REST API. Where must CORS headers be configured?

A) Only in the Lambda function response
B) Only in the API Gateway method response
C) In the API Gateway CORS configuration and the Lambda function response for proxy integrations
D) In the CloudFront distribution settings

**Q54.** A developer needs to ensure that IAM policies attached to Lambda execution roles follow least privilege. The function only reads from a specific DynamoDB table. Which policy statement is MOST restrictive?

A) Allow dynamodb:* on resource *
B) Allow dynamodb:GetItem, dynamodb:Query on the specific table ARN
C) Allow dynamodb:Read* on the specific table ARN
D) Allow dynamodb:GetItem, dynamodb:Query, dynamodb:Scan on resource *

**Q55.** A developer is using AWS X-Ray to trace requests through a microservices application. To propagate the trace across services, what must be included in downstream HTTP requests?

A) The X-Ray daemon address
B) The X-Amzn-Trace-Id header
C) The AWS access key ID
D) The X-Ray segment document

**Q56.** A developer needs to authenticate machine-to-machine API calls to API Gateway without user interaction. Which authorization method is MOST appropriate?

A) Cognito user pool authorizer
B) IAM authorization with SigV4 signing
C) API keys only
D) Lambda authorizer with hardcoded credentials

**Q57.** A developer discovers that their Lambda function's IAM role has AdministratorAccess. What is the security risk and remediation?

A) No risk since Lambda functions are isolated; no change needed
B) The function can access any AWS resource; replace with a least-privilege policy scoped to required services and resources
C) The function will run slower; reduce the policy size
D) The function cannot be invoked by other services; add a resource-based policy

**Q58.** A developer needs to securely pass sensitive data to an ECS container at runtime without baking it into the container image. What should they use?

A) Dockerfile ENV instructions
B) ECS task definition secrets referencing Secrets Manager or SSM Parameter Store
C) S3 bucket with the configuration file
D) EC2 instance user data

---

## Domain 3: Deployment (Questions 59–82)

**Q59.** A developer wants to deploy a serverless application defined with AWS SAM. Which command packages the application and uploads artifacts to S3?

A) sam init
B) sam build && sam deploy
C) sam package
D) sam local start-api

**Q60.** A developer is using AWS CodePipeline with CodeBuild and CodeDeploy. The build stage compiles the application and runs unit tests. Where are the build commands defined?

A) appspec.yml
B) buildspec.yml
C) template.yaml
D) pipeline.json

**Q61.** A developer needs to deploy a new version of a Lambda function and gradually shift traffic from the old version to the new version over 30 minutes. Which deployment configuration should they use?

A) Lambda AllAtOnce
B) Lambda Canary10Percent30Minutes
C) Lambda Linear10PercentEvery3Minutes
D) Lambda Blue/Green with manual approval

**Q62.** A developer is using Elastic Beanstalk to deploy a web application. They need zero-downtime deployments with the ability to roll back instantly. Which deployment policy should they choose?

A) All at once
B) Rolling
C) Rolling with additional batch
D) Immutable

**Q63.** A developer needs to define infrastructure and Lambda functions in a single template and deploy using the AWS CLI. Which framework should they use?

A) AWS CloudFormation only
B) AWS SAM (Serverless Application Model)
C) AWS CDK only
D) Terraform

**Q64.** A developer is using CodeDeploy to deploy to EC2 instances. They need to run a script to stop the existing application before the new version is installed. Which lifecycle event hook should they use?

A) BeforeInstall
B) ApplicationStop
C) AfterInstall
D) ValidateService

**Q65.** A developer has a CloudFormation stack that failed to create due to an error in one resource. What happens to the other resources that were successfully created?

A) They remain in the account and must be manually deleted
B) CloudFormation rolls back and deletes all resources created in the stack by default
C) CloudFormation pauses and waits for manual intervention
D) Only the failed resource is deleted

**Q66.** A developer needs to deploy a Docker container to AWS with minimal infrastructure management. The container runs a long-lived web service. Which service requires the LEAST operational overhead?

A) Amazon ECS on EC2
B) Amazon ECS on Fargate
C) Amazon EKS on EC2
D) Amazon EC2 with Docker installed

**Q67.** A developer is using SAM and needs to test a Lambda function locally before deploying. Which command invokes the function locally with a test event?

A) sam deploy --test
B) sam local invoke -e event.json
C) sam validate
D) sam publish

**Q68.** A developer needs to create a reusable CloudFormation template that accepts different values for different environments (dev, staging, prod). What should they use?

A) CloudFormation Mappings only
B) CloudFormation Parameters
C) CloudFormation Outputs
D) Nested stacks

**Q69.** A developer is deploying an application using CodeDeploy with a Blue/Green deployment on ECS. How does CodeDeploy route traffic to the new task set?

A) By updating the ECS service directly
B) By shifting traffic on the ALB target groups
C) By updating Route 53 DNS records
D) By modifying the security group rules

**Q70.** A developer needs to automate the creation of Lambda layers that are shared across multiple functions. Where should the layer content be defined in a SAM template?

A) AWS::Serverless::Function Properties
B) AWS::Serverless::LayerVersion resource
C) AWS::Lambda::Alias resource
D) AWS::CloudFormation::CustomResource

**Q71.** A developer is using Elastic Beanstalk and needs to customize the EC2 instances by installing additional packages during deployment. Where should they define these customizations?

A) In the Elastic Beanstalk console only
B) In .ebextensions configuration files in the application source bundle
C) In the EC2 user data script
D) In a separate CloudFormation template

**Q72.** A developer needs to deploy a Lambda function that uses a container image instead of a ZIP file. What is the maximum container image size allowed?

A) 50 MB
B) 250 MB
C) 1 GB
D) 10 GB

**Q73.** A developer is using CodePipeline and needs to require manual approval before deploying to production. What should they add to the pipeline?

A) A Lambda function that pauses execution
B) A manual approval action in the pipeline stage
C) A CodeBuild project that waits for input
D) An SNS notification with a callback URL

**Q74.** A developer needs to reference the output of one CloudFormation stack in another stack. What should they use?

A) CloudFormation Parameters
B) CloudFormation Exports and Fn::ImportValue
C) CloudFormation Mappings
D) AWS Systems Manager Parameter Store

**Q75.** A developer is deploying a serverless application and needs to create a DynamoDB table, a Lambda function, and an API Gateway endpoint. Which SAM resource types should they use? (Select TWO)

A) AWS::Serverless::Function
B) AWS::Serverless::Api
C) AWS::Serverless::SimpleTable
D) AWS::EC2::Instance
E) Both A and C

**Q76.** A developer needs to ensure that a CodeDeploy deployment to EC2 is automatically rolled back if the health check fails after deployment. What should they configure?

A) CloudWatch alarm-based automatic rollback in the deployment group
B) A Lambda function triggered by CodeDeploy events
C) Manual rollback through the CodeDeploy console
D) An SNS notification to the operations team

**Q77.** A developer is using CloudFormation and needs to run a custom script during stack creation that is not supported by any CloudFormation resource type. What should they use?

A) CloudFormation Macros
B) CloudFormation custom resource backed by a Lambda function
C) CloudFormation StackSets
D) CloudFormation drift detection

**Q78.** A developer needs to deploy the same application to multiple AWS regions simultaneously. Which tool should they use?

A) CloudFormation StackSets
B) CodeDeploy with multiple deployment groups
C) Elastic Beanstalk with environment cloning
D) SAM deploy with --region flag run multiple times

**Q79.** A developer is using Elastic Beanstalk and wants to run a specific version of their application on a separate environment for testing before swapping it into production. Which Elastic Beanstalk feature should they use?

A) Rolling deployment
B) Environment cloning and CNAME swap
C) All at once deployment
D) Traffic splitting

**Q80.** A developer needs to store build artifacts from CodeBuild so they can be used by a subsequent CodeDeploy stage in CodePipeline. Where are these artifacts stored?

A) Amazon EFS
B) Amazon S3 (pipeline artifact bucket)
C) Amazon DynamoDB
D) AWS CodeArtifact

**Q81.** A developer is writing a SAM template and needs to specify that a Lambda function should be triggered by an S3 PUT event. How should they define this in the template?

A) Add an AWS::S3::Bucket resource with NotificationConfiguration
B) Add an S3 event source in the function's Events property
C) Create a separate CloudWatch Events rule
D) Use an AWS::Lambda::EventSourceMapping resource

**Q82.** A developer needs to deploy a new version of an ECS service with zero downtime. The service runs behind an ALB. Which deployment type should they use?

A) ECS rolling update
B) ECS Blue/Green deployment with CodeDeploy
C) Stop the old tasks and start new tasks
D) Both A and B achieve zero downtime

---

## Domain 4: Troubleshooting and Optimization (Questions 83–100)

**Q83.** A developer's Lambda function is experiencing high latency on the first invocation after a period of inactivity. What is this called and how can it be mitigated?

A) Timeout error; increase the function timeout
B) Cold start; use provisioned concurrency
C) Throttling; request a concurrency limit increase
D) Memory pressure; increase the function memory

**Q84.** A developer is troubleshooting a distributed application and needs to visualize the end-to-end request flow across API Gateway, Lambda, and DynamoDB. Which service should they use?

A) Amazon CloudWatch Logs
B) AWS X-Ray
C) AWS CloudTrail
D) Amazon CloudWatch Metrics

**Q85.** A developer receives a `ProvisionedThroughputExceededException` from DynamoDB. What does this indicate and what is the recommended short-term fix?

A) The table does not exist; create the table
B) The request rate exceeds provisioned capacity; implement exponential backoff and consider switching to on-demand capacity mode
C) The item size exceeds 400 KB; reduce the item size
D) The table is being deleted; wait and retry

**Q86.** A Lambda function is returning a 502 Bad Gateway error when invoked through API Gateway. What is the MOST likely cause?

A) The API Gateway stage is not deployed
B) The Lambda function is returning a malformed response (missing statusCode or body)
C) The Lambda function does not have permission to be invoked by API Gateway
D) The API Gateway timeout is too short

**Q87.** A developer needs to monitor the number of messages in an SQS queue that are available for processing. Which CloudWatch metric should they use?

A) NumberOfMessagesSent
B) ApproximateNumberOfMessagesVisible
C) ApproximateNumberOfMessagesNotVisible
D) NumberOfMessagesDeleted

**Q88.** A developer's application is making API calls to AWS and receiving `ThrottlingException` errors. The application uses the AWS SDK. What built-in SDK feature helps handle this?

A) Connection pooling
B) Automatic retry with exponential backoff
C) Request batching
D) Response caching

**Q89.** A developer needs to find out why a Lambda function is running out of memory. Where should they look?

A) AWS X-Ray traces
B) CloudWatch Logs (the function's log group)
C) CloudTrail events
D) Lambda console deployment history

**Q90.** A developer is using DynamoDB and notices that read operations on a table with a hot partition key are being throttled while overall table capacity is not fully utilized. What should they do?

A) Increase the table's provisioned read capacity
B) Redesign the partition key to distribute reads more evenly
C) Enable DynamoDB Streams
D) Switch to strongly consistent reads

**Q91.** A developer is using CloudWatch Logs and needs to search for all log entries containing the text "ERROR" across multiple Lambda functions. What should they use?

A) CloudWatch Logs Insights query across log groups
B) CloudWatch Alarms
C) CloudWatch Metrics filter
D) AWS X-Ray analytics

**Q92.** A developer deploys a new Lambda function version but the application is still invoking the old version. The function is invoked via an alias. What is the MOST likely cause?

A) The Lambda function was not published as a new version
B) The alias still points to the old version
C) The IAM role does not have permission to invoke the new version
D) The function's reserved concurrency is set to 0

**Q93.** A developer needs to optimize a Lambda function that processes large JSON payloads. The function currently takes 8 seconds to execute with 128 MB memory. What should they try FIRST?

A) Rewrite the function in a compiled language
B) Increase the function memory (which also increases CPU)
C) Increase the function timeout
D) Split the payload into smaller chunks

**Q94.** An application writes to DynamoDB and the developer notices that write latency spikes during peak hours. The table uses provisioned capacity. What are TWO actions that can help? (Select TWO)

A) Enable DynamoDB auto-scaling
B) Switch to on-demand capacity mode
C) Add a local secondary index
D) Reduce the item size
E) Enable DynamoDB Streams

**Q95.** A developer is using API Gateway and notices that responses are slow. The backend Lambda function executes in 200ms but the API response takes 2 seconds. What should they investigate?

A) Lambda cold starts
B) API Gateway integration latency and logging
C) DynamoDB throttling
D) CloudFront cache misses

**Q96.** A developer needs to set up a CloudWatch alarm that triggers when a Lambda function's error rate exceeds 5% over a 5-minute period. Which approach should they use?

A) Create an alarm on the Errors metric with a static threshold
B) Create a metric math expression dividing Errors by Invocations and alarm on that
C) Create an alarm on the Duration metric
D) Use CloudWatch Logs metric filters only

**Q97.** A developer is using X-Ray and needs to record custom metadata about a specific operation within a Lambda function. What should they add to the code?

A) A new X-Ray segment
B) An X-Ray subsegment with annotations or metadata
C) A CloudWatch custom metric
D) A CloudWatch Logs structured log entry

**Q98.** A developer's ECS Fargate task is failing to pull a container image from Amazon ECR. What are TWO possible causes? (Select TWO)

A) The task execution role does not have ECR permissions
B) The Fargate task is in a private subnet without a NAT gateway or VPC endpoint to ECR
C) The container image is too large for Fargate
D) The ECS cluster does not have enough EC2 instances
E) The container port mapping is incorrect

**Q99.** A developer is optimizing a DynamoDB query that returns 500 items but the application only needs 10 specific attributes from each item. How can they reduce read costs?

A) Use a Scan instead of a Query
B) Use a projection expression to return only the needed attributes
C) Increase the read capacity units
D) Use eventually consistent reads only

**Q100.** A developer needs to trace a request that flows through API Gateway → Lambda → SQS → Lambda. X-Ray shows the first Lambda but not the second. What is the MOST likely cause?

A) X-Ray does not support SQS
B) The second Lambda function does not have X-Ray tracing enabled
C) SQS does not propagate the X-Ray trace header
D) The X-Ray daemon is not running
