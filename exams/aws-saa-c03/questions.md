# AWS SAA-C03 Practice Exam — 100 Questions

Weighted by exam domain:
- Domain 1: Design Secure Architectures (Q1–Q30) — 30%
- Domain 2: Design Resilient Architectures (Q31–Q56) — 26%
- Domain 3: Design High-Performing Architectures (Q57–Q80) — 24%
- Domain 4: Design Cost-Optimized Architectures (Q81–Q100) — 20%

Passing target: 720/1000 (~72%). Aim for 85%+ in practice.

---

## Domain 1: Design Secure Architectures (Questions 1–30)

**Q1.** A company wants to centrally manage access across 20 AWS accounts. Developers should use their corporate Active Directory credentials to sign in to the AWS Management Console. Which service should the architect use?

A) AWS IAM with cross-account roles in each account
B) AWS IAM Identity Center (SSO) with AD Connector
C) Amazon Cognito user pools federated with Active Directory
D) AWS Directory Service for Microsoft Active Directory in each account

**Q2.** An application on EC2 needs to read objects from S3 and write messages to SQS. What is the MOST secure way to grant these permissions?

A) Store IAM user access keys in environment variables on the instance
B) Attach an IAM role with an S3 read and SQS write policy to the EC2 instance
C) Embed access keys in the application configuration file
D) Create an IAM user with full S3 and SQS access and store credentials in Secrets Manager

**Q3.** A company requires that all data stored in Amazon S3 be encrypted at rest using keys that are automatically rotated annually, with minimal operational overhead. Which encryption option meets this requirement?

A) SSE-S3
B) SSE-KMS with an AWS managed key
C) SSE-KMS with a customer managed key and automatic rotation enabled
D) SSE-C with keys rotated by the application

**Q4.** A solutions architect needs to restrict S3 bucket access so that objects can only be accessed from within a specific VPC. Which approach should they use?

A) S3 bucket policy with a condition for `aws:SourceVpc`
B) S3 ACLs restricting access to specific IP ranges
C) IAM policy attached to all users denying S3 access outside the VPC
D) Enable S3 Block Public Access

**Q5.** A company needs to encrypt data in transit between an on-premises data center and AWS. Which TWO options provide encryption in transit? (Select TWO)

A) AWS Site-to-Site VPN
B) VPC peering
C) AWS Direct Connect with MACsec
D) NAT gateway
E) Internet gateway

**Q6.** An application stores database credentials in plaintext configuration files on EC2 instances. The security team requires that credentials be stored securely and rotated automatically. Which solution meets these requirements?

A) Store credentials in an encrypted S3 bucket
B) Use AWS Secrets Manager with automatic rotation enabled
C) Store credentials in AWS Systems Manager Parameter Store as a String parameter
D) Encrypt the configuration files using the instance's SSH key

**Q7.** A company uses an ALB to route traffic to EC2 instances. They need to ensure that traffic between clients and the ALB is encrypted using HTTPS. What must the architect configure?

A) Install SSL certificates on each EC2 instance
B) Associate an ACM certificate with the ALB HTTPS listener
C) Enable SSL termination on the NAT gateway
D) Configure security groups to only allow port 443

**Q8.** A solutions architect needs to ensure that an IAM role can only be assumed during business hours (9 AM–5 PM UTC). How should this be implemented?

A) Add a condition in the role's trust policy using `aws:CurrentTime`
B) Use an SCP to restrict role assumption by time
C) Create a Lambda function that attaches/detaches the role on a schedule
D) Add a condition in the IAM permissions policy using `aws:CurrentTime`

**Q9.** A company has an S3 bucket that must never be made public, even accidentally. What is the MOST effective preventive control?

A) Enable CloudTrail logging on the bucket
B) Use Amazon Macie to scan for public access
C) Enable S3 Block Public Access at the account level
D) Create a CloudWatch alarm for public bucket events

**Q10.** An application running in a private subnet needs to call the S3 API without traffic traversing the internet. What should the architect configure?

A) A NAT gateway in the public subnet
B) An S3 gateway VPC endpoint
C) An internet gateway with a route from the private subnet
D) VPC peering with the S3 service VPC

**Q11.** A company needs to allow a third-party auditor temporary, read-only access to their AWS account for 1 hour. What is the MOST secure approach?

A) Create an IAM user with read-only access and delete it after 1 hour
B) Share the root account credentials temporarily
C) Create an IAM role with read-only access and provide the auditor with a cross-account assume-role link with a 1-hour session duration
D) Create an access key pair and revoke it after 1 hour

**Q12.** A company needs to log all API calls across all AWS accounts in their organization for compliance. Which service should they use?

A) Amazon CloudWatch Logs
B) AWS CloudTrail with an organization trail
C) AWS Config
D) VPC Flow Logs

**Q13.** An application uses Amazon RDS MySQL. The security team requires that connections to the database be encrypted in transit. What should the architect do?

A) Enable encryption at rest on the RDS instance
B) Configure the application to use SSL/TLS connections and enforce `require_secure_transport` on the RDS instance
C) Place the RDS instance in a private subnet
D) Enable IAM database authentication

**Q14.** A company needs to detect unauthorized access attempts and potential threats across their AWS accounts. Which service provides this capability?

A) AWS WAF
B) Amazon GuardDuty
C) AWS Shield
D) Amazon Inspector

**Q15.** A solutions architect is designing a VPC. The application tier must accept traffic only from the web tier, and the database tier must accept traffic only from the application tier. What should they use?

A) Network ACLs on each subnet
B) Security groups with references to other security groups
C) Route table configurations
D) AWS Network Firewall

**Q16.** A company needs to ensure that EBS volumes attached to EC2 instances are always encrypted. How can they enforce this across the account with MINIMAL effort?

A) Create an IAM policy denying `ec2:CreateVolume` without encryption
B) Enable EBS encryption by default in the account settings for each Region
C) Use AWS Config to detect unencrypted volumes
D) Use a Lambda function to encrypt volumes after creation

**Q17.** A web application is vulnerable to SQL injection attacks. Which AWS service can the architect use to create rules that block SQL injection attempts?

A) AWS Shield Standard
B) AWS WAF
C) Amazon GuardDuty
D) AWS Network Firewall

**Q18.** A company uses AWS Organizations. They want to ensure that no one can disable CloudTrail logging in any member account. What should they do?

A) Enable CloudTrail in each account with MFA delete
B) Apply an SCP that denies `cloudtrail:StopLogging` and `cloudtrail:DeleteTrail`
C) Use AWS Config rules to detect when CloudTrail is disabled
D) Create an IAM policy in each account denying CloudTrail modifications

**Q19.** An application needs to authenticate mobile app users and provide them with temporary AWS credentials to access S3 directly. Which service should the architect use?

A) IAM users with access keys
B) Amazon Cognito identity pools
C) AWS IAM Identity Center
D) AWS STS directly from the mobile app

**Q20.** A company stores encryption keys in AWS KMS. They need to ensure that a specific key can only be used by EC2 instances accessing it through a VPC endpoint. How should they configure this?

A) Use a KMS key policy with a `kms:ViaService` condition
B) Use a KMS key policy with an `aws:sourceVpce` condition referencing the VPC endpoint ID
C) Restrict the key to specific IAM users
D) Use a separate KMS key per EC2 instance

**Q21.** A company needs to protect a web application from common exploits and allow only traffic from specific countries. Which service should they use?

A) AWS Shield Standard and security groups
B) AWS WAF with geo-match conditions and rate-based rules
C) Amazon GuardDuty and NACLs
D) AWS Network Firewall and Route 53

**Q22.** A solutions architect needs to grant an AWS Lambda function in Account A access to a DynamoDB table in Account B. What is the correct approach?

A) Create an IAM user in Account B and store credentials in the Lambda environment variables
B) Create an IAM role in Account B with DynamoDB access and have the Lambda function assume it using STS
C) Make the DynamoDB table publicly accessible
D) Use VPC peering between the two accounts

**Q23.** A company needs to ensure that all objects uploaded to S3 by any principal are encrypted with a particular KMS key. How should they enforce this?

A) Enable default encryption on the bucket with the KMS key
B) Add a bucket policy that denies `s3:PutObject` requests that don't include the correct `x-amz-server-side-encryption-aws-kms-key-id` header
C) Configure each application to always specify the key
D) Use S3 Object Lock to enforce encryption

**Q24.** A company is deploying a public-facing API using API Gateway. They need to ensure that only authenticated users can access the API. Which THREE methods can they use? (Select THREE)

A) API Gateway resource policies
B) Lambda authorizers
C) Amazon Cognito user pool authorizers
D) VPC security groups
E) IAM authorization
F) S3 bucket policies

**Q25.** A company needs to provide shell access to EC2 instances without opening SSH ports or managing SSH keys, while maintaining an audit trail. Which approach is BEST?

A) Store SSH keys in AWS Secrets Manager and distribute them manually
B) Use AWS Systems Manager Session Manager
C) Store SSH keys in an encrypted S3 bucket
D) Use IAM users with SSH key pairs

**Q26.** A company has a regulatory requirement to ensure that their AWS resources are deployed only in us-east-1 and eu-west-1. They use AWS Organizations. What is the MOST effective enforcement mechanism?

A) IAM policies in each account restricting Region access
B) SCPs attached to the OU denying all actions outside allowed Regions
C) AWS Config rules that flag resources in non-compliant Regions
D) CloudFormation StackSets that only deploy to allowed Regions

**Q27.** An application uses an Amazon RDS database. The security team wants to ensure that the database is not publicly accessible and can only be reached from the application's security group. What should the architect verify?

A) The RDS instance has "Publicly Accessible" set to No and the DB security group allows inbound only from the application security group
B) The RDS instance is in a public subnet with restrictive NACLs
C) The RDS instance uses IAM authentication
D) The RDS instance has encryption at rest enabled

**Q28.** A company needs to rotate their RDS database master password every 30 days without application downtime. Which solution requires the LEAST operational effort?

A) Use a Lambda function triggered by EventBridge on a 30-day schedule to update the password
B) Use AWS Secrets Manager with automatic rotation configured for 30 days
C) Manually update the password and redeploy the application monthly
D) Use IAM database authentication instead of passwords

**Q29.** A company wants to prevent data exfiltration from their VPC. They need to ensure that S3 access from EC2 instances is restricted to only company-owned buckets. What should they configure?

A) S3 bucket policies on company buckets allowing VPC access
B) A VPC endpoint policy on the S3 gateway endpoint that restricts access to specific bucket ARNs
C) Security group rules blocking S3 traffic to non-company buckets
D) NACLs filtering S3 traffic by IP address

**Q30.** A company needs to enforce multi-factor authentication for all IAM users who have console access. What is the MOST effective way to enforce this?

A) Send an email reminder to all users to enable MFA
B) Create an IAM policy that denies all actions except MFA setup when MFA is not present, and attach it to all users
C) Use AWS Config to detect users without MFA
D) Enable MFA on the root account only

---

## Domain 2: Design Resilient Architectures (Questions 31–56)

**Q31.** A company runs a stateless web application on EC2 instances behind an ALB. They need the application to survive the failure of an entire Availability Zone. What is the MINIMUM configuration?

A) Deploy instances in a single AZ with an Auto Scaling group
B) Deploy instances across at least two AZs with an Auto Scaling group and ALB
C) Deploy instances in a single AZ with a standby instance in another AZ
D) Use a Network Load Balancer with cross-zone load balancing disabled

**Q32.** An application writes data to an RDS MySQL database. The company requires the database to automatically failover to a standby in case of an infrastructure failure, with minimal data loss. Which feature should the architect enable?

A) Read replicas
B) Multi-AZ deployment
C) Cross-Region replication
D) Automated snapshots every 5 minutes

**Q33.** A company has a web application that stores user session data on the local EC2 instance. When instances are terminated by Auto Scaling, users lose their sessions. How should the architect fix this?

A) Increase the instance size to handle more users
B) Store session data in Amazon ElastiCache or DynamoDB
C) Disable Auto Scaling scale-in actions
D) Use sticky sessions on the ALB

**Q34.** A company needs to design a disaster recovery strategy with an RTO of 4 hours and an RPO of 1 hour. The solution must minimize costs during normal operations. Which DR strategy is MOST appropriate?

A) Multi-site active-active
B) Warm standby
C) Pilot light
D) Backup and restore

**Q35.** An application uses an SQS standard queue. Messages are sometimes processed more than once, causing duplicate database entries. The application cannot be modified to handle idempotency. What should the architect do?

A) Increase the visibility timeout
B) Switch to an SQS FIFO queue
C) Enable long polling
D) Use a dead-letter queue

**Q36.** A company runs a critical application on a single EC2 instance with data stored on an instance store volume. What is the BIGGEST risk with this architecture?

A) The instance cannot be stopped and started
B) Data is lost if the instance is stopped, terminated, or the underlying hardware fails
C) Instance store volumes cannot be encrypted
D) Instance store volumes have lower IOPS than EBS

**Q37.** A company needs to decouple a front-end web application from a back-end processing system. The back-end takes 5–10 minutes to process each request. Users should receive an immediate acknowledgment. Which architecture is BEST?

A) The web app calls the back-end synchronously and waits for a response
B) The web app sends a message to an SQS queue, returns a success response, and the back-end polls the queue
C) The web app writes to an S3 bucket and the back-end uses S3 event notifications
D) The web app calls the back-end via API Gateway with a 10-minute timeout

**Q38.** A company uses Amazon Aurora MySQL. They need to ensure the database can handle a sudden increase in read traffic during a marketing campaign. What should the architect recommend?

A) Upgrade to a larger Aurora instance
B) Add Aurora read replicas and use the Aurora reader endpoint
C) Enable Multi-AZ on the Aurora cluster
D) Switch to DynamoDB for the campaign period

**Q39.** A company has an application that publishes messages to multiple downstream consumers. Each consumer needs to receive every message independently. Which architecture should the architect use?

A) One SQS queue with multiple consumers polling
B) An SNS topic with an SQS queue subscription for each consumer (fan-out pattern)
C) Multiple SQS queues with the application sending to each queue
D) Amazon MQ with multiple consumers

**Q40.** A company needs to ensure that their S3 data is protected against accidental deletion. Which TWO features should they enable? (Select TWO)

A) S3 Versioning
B) S3 Transfer Acceleration
C) MFA Delete
D) S3 Intelligent-Tiering
E) S3 Block Public Access

**Q41.** A company runs a containerized application on Amazon ECS with Fargate. The application needs to scale based on the number of messages in an SQS queue. How should the architect configure this?

A) Use an ALB target tracking scaling policy
B) Configure ECS Service Auto Scaling with a custom CloudWatch metric based on the SQS queue depth
C) Use EC2 Auto Scaling with the SQS queue metric
D) Manually adjust the desired task count based on queue depth

**Q42.** A company has a Lambda function that processes records from a DynamoDB stream. If the function fails, it retries indefinitely and blocks processing of new records. How should the architect handle this?

A) Increase the Lambda function timeout
B) Configure a maximum retry attempts limit and an on-failure destination (e.g., SQS dead-letter queue)
C) Remove the DynamoDB stream trigger
D) Increase the Lambda function memory

**Q43.** A company needs to run a web application that must remain available even if an entire AWS Region becomes unavailable. Which architecture supports this?

A) Multi-AZ deployment within a single Region
B) Multi-Region deployment with Route 53 failover routing
C) A single Region with multiple VPCs
D) CloudFront distribution with a single origin

**Q44.** A company uses an Auto Scaling group with a minimum of 2 and maximum of 10 instances across 3 AZs. During a scale-in event, which instances does Auto Scaling terminate first by default?

A) The oldest instances
B) Instances in the AZ with the most instances, then the instance with the oldest launch configuration/template
C) The newest instances
D) Instances with the lowest CPU utilization

**Q45.** A company has an application that writes to an RDS PostgreSQL database. They need to create a read-only copy of the database in another Region for reporting purposes. Which feature should they use?

A) Multi-AZ standby in another Region
B) Cross-Region read replica
C) AWS DMS continuous replication
D) RDS automated backups to another Region

**Q46.** A company runs a batch processing application that reads from an SQS queue. Occasionally, messages that cannot be processed successfully keep returning to the queue. How should the architect handle these poison messages?

A) Increase the visibility timeout to 12 hours
B) Configure a dead-letter queue with a maximum receive count
C) Delete the messages manually
D) Increase the message retention period

**Q47.** A company needs to deploy a highly available application across two AZs. The application requires a static IP address for each AZ for firewall whitelisting by partners. Which load balancer should they use?

A) Application Load Balancer
B) Network Load Balancer
C) Classic Load Balancer
D) Gateway Load Balancer

**Q48.** A company uses Amazon S3 for storing critical business data. They need to replicate all objects to a bucket in another Region for compliance. Which feature should they use?

A) S3 Transfer Acceleration
B) S3 Cross-Region Replication (CRR)
C) AWS DataSync scheduled task
D) S3 batch operations to copy objects

**Q49.** A company has a three-tier application. The web tier uses an ALB, the app tier runs on EC2, and the database is RDS Multi-AZ. During a load test, the app tier becomes a bottleneck. What should the architect add?

A) A larger RDS instance
B) An Auto Scaling group for the app tier EC2 instances
C) A second ALB for the app tier
D) Read replicas for the RDS instance

**Q50.** A company uses AWS Step Functions to orchestrate a multi-step order processing workflow. One step calls an external API that occasionally times out. How should the architect make this step resilient?

A) Increase the Step Functions state machine timeout
B) Configure retry with exponential backoff and a catch block to handle failures
C) Remove the external API call from the workflow
D) Use a larger Lambda function for the API call

**Q51.** A company runs a legacy application that requires a fixed IP address. The application must be highly available across two AZs. The architect deploys the application on EC2 instances behind a Network Load Balancer. How should they handle the fixed IP requirement?

A) Assign an Elastic IP to each EC2 instance
B) Assign an Elastic IP to each NLB subnet (one per AZ)
C) Use a Route 53 A record pointing to the NLB DNS name
D) Use an ALB with a static IP address

**Q52.** A company has an application that uses Amazon Aurora with one writer and two reader instances. The writer instance fails. What happens automatically?

A) The application must manually promote a reader
B) Aurora automatically promotes one of the reader instances to writer, typically within 30 seconds
C) Aurora launches a new writer instance from the last snapshot
D) The application fails until the writer is manually restarted

**Q53.** A company needs to migrate an on-premises message queue that uses the MQTT protocol to AWS. Which service should they use?

A) Amazon SQS
B) Amazon SNS
C) Amazon MQ
D) Amazon Kinesis

**Q54.** A company runs a web application on EC2 instances in an Auto Scaling group. They want to ensure that new instances are healthy before receiving traffic from the ALB. What should they configure?

A) EC2 status checks only
B) ALB health checks on the target group with a health check path
C) CloudWatch alarms on CPU utilization
D) A Lambda function that tests each new instance

**Q55.** A company uses Route 53 for DNS. They have an active-passive failover setup with a primary site in us-east-1 and a secondary site in eu-west-1. How should they configure Route 53?

A) Simple routing with both endpoints
B) Weighted routing with 100% to primary
C) Failover routing with health checks on the primary endpoint
D) Latency-based routing

**Q56.** A company needs to ensure that their Auto Scaling group always maintains at least 2 healthy instances, even during deployments. Which update policy should they use?

A) Rolling update with minimum instances in service set to 2
B) Replace all instances at once
C) Manual replacement of instances one at a time
D) Blue/green deployment using a second Auto Scaling group

---

## Domain 3: Design High-Performing Architectures (Questions 57–80)

**Q57.** A company needs a storage solution for a Linux-based HPC workload that requires sub-millisecond latency and high throughput. The data is also stored in S3. Which storage service should they use?

A) Amazon EFS
B) Amazon EBS io2
C) Amazon FSx for Lustre linked to S3
D) Amazon S3 with Transfer Acceleration

**Q58.** An application performs a mix of read and write operations on an EBS volume. The workload requires sustained 64,000 IOPS. Which EBS volume type should the architect choose?

A) gp3
B) io2 Block Express
C) st1
D) gp2

**Q59.** A company has a DynamoDB table with a partition key of `user_id`. During peak hours, a small number of very active users cause hot partitions. How should the architect address this?

A) Increase the provisioned write capacity units
B) Add a random suffix to the partition key to distribute writes more evenly
C) Switch to a sort key based on timestamp
D) Enable DynamoDB Streams

**Q60.** A company needs to serve static assets (images, CSS, JS) for a web application with the lowest possible latency globally. The assets are stored in S3. What should the architect configure?

A) Enable S3 Transfer Acceleration
B) Create a CloudFront distribution with the S3 bucket as the origin
C) Replicate the S3 bucket to multiple Regions
D) Use Route 53 geolocation routing to direct users to the nearest S3 bucket

**Q61.** An application needs to process 10,000 messages per second from an SQS FIFO queue. The current throughput limit is 300 messages per second per message group. How can the architect increase throughput?

A) Switch to a standard queue
B) Use multiple message group IDs to parallelize processing
C) Increase the visibility timeout
D) Enable long polling

**Q62.** A company runs a relational database on RDS MySQL. They need to cache frequently accessed query results to reduce database load. Which service should they use?

A) Amazon CloudFront
B) Amazon ElastiCache for Redis
C) Amazon DynamoDB DAX
D) AWS Global Accelerator

**Q63.** A company needs to run a containerized microservices application. They want to avoid managing the underlying infrastructure. Which compute option should they use?

A) Amazon ECS on EC2
B) Amazon ECS on AWS Fargate
C) Amazon EC2 with Docker installed
D) Amazon EKS on EC2

**Q64.** A company has a data lake in S3. Analysts need to run ad-hoc SQL queries against CSV and Parquet files without loading them into a database. Which service should they use?

A) Amazon Redshift
B) Amazon RDS
C) Amazon Athena
D) Amazon EMR

**Q65.** An application uses an ALB to distribute traffic to EC2 instances. The application requires WebSocket connections for real-time features. Does the ALB support this?

A) No, they must use a Network Load Balancer
B) No, they must use a Classic Load Balancer
C) Yes, ALB natively supports WebSocket connections
D) Yes, but only with a CloudFront distribution in front

**Q66.** A company needs to migrate a large on-premises Oracle database to AWS. They want to move to PostgreSQL to reduce licensing costs. Which service should they use for the schema and data migration?

A) AWS DataSync
B) AWS DMS with the AWS Schema Conversion Tool (SCT)
C) AWS Transfer Family
D) AWS Snowball Edge

**Q67.** A company has an API that receives bursty traffic — idle for hours, then thousands of requests per second. They need the API to scale to zero when idle and handle bursts automatically. Which architecture is BEST?

A) EC2 instances in an Auto Scaling group behind an ALB
B) API Gateway with Lambda integration
C) ECS Fargate behind an ALB
D) EC2 instances with Reserved capacity

**Q68.** A company needs to stream real-time application logs from multiple EC2 instances to S3 for long-term storage and to OpenSearch for real-time analysis. Which service should they use for ingestion?

A) Amazon Kinesis Data Streams
B) Amazon Data Firehose
C) Amazon SQS
D) AWS Glue

**Q69.** A company has a DynamoDB table that experiences unpredictable traffic patterns — sometimes very high, sometimes very low. They want to avoid provisioning capacity. Which capacity mode should they use?

A) Provisioned capacity with auto scaling
B) On-demand capacity mode
C) Reserved capacity
D) Provisioned capacity with a fixed high value

**Q70.** A company needs to deploy a global application where users in each Region are routed to the nearest healthy endpoint with the fastest response time. Which Route 53 routing policy should they use?

A) Geolocation routing
B) Weighted routing
C) Latency-based routing
D) Simple routing

**Q71.** A company has a large dataset in S3 that needs to be transformed from CSV to Parquet format for analytics. Which serverless service can perform this transformation?

A) Amazon EMR
B) AWS Glue ETL
C) Amazon Athena
D) AWS Lambda

**Q72.** A company runs a high-traffic web application. They need to offload SSL/TLS termination from their EC2 instances to improve performance. Which approach should they use?

A) Terminate SSL at the ALB and send HTTP traffic to instances
B) Use CloudFront for SSL termination only
C) Install SSL certificates on each instance and use a Network Load Balancer in TCP mode
D) Use a NAT gateway for SSL termination

**Q73.** A company needs a database that can handle millions of requests per second with single-digit millisecond latency for a gaming leaderboard. The data model is key-value. Which database should they use?

A) Amazon RDS MySQL
B) Amazon Aurora
C) Amazon DynamoDB
D) Amazon Redshift

**Q74.** A company has an application that needs to process images uploaded to S3. Processing takes 2–3 minutes per image. Which architecture is MOST scalable?

A) An EC2 instance polling S3 for new objects
B) S3 event notification triggering a Lambda function (with increased timeout)
C) S3 event notification → SQS queue → Lambda or ECS Fargate consumers
D) A cron job on EC2 that checks S3 every minute

**Q75.** A company needs to accelerate TCP-based application traffic from global users to their ALB in us-east-1. Which service should they use?

A) Amazon CloudFront
B) AWS Global Accelerator
C) Route 53 latency-based routing
D) S3 Transfer Acceleration

**Q76.** A company has a multi-tier application. The web tier needs to communicate with the application tier using a load balancer, but the app tier should not be exposed to the internet. Which configuration is correct?

A) Public ALB for the web tier, internal ALB for the app tier
B) Public ALB for both tiers
C) Network Load Balancer with public IPs for both tiers
D) No load balancer for the app tier, direct instance-to-instance communication

**Q77.** A company needs to run complex analytical queries across petabytes of structured data. Which service is MOST appropriate?

A) Amazon DynamoDB
B) Amazon Aurora
C) Amazon Redshift
D) Amazon ElastiCache

**Q78.** A company has an application that needs to read the same S3 objects thousands of times per second. They want to reduce S3 GET request costs and improve latency. What should they do?

A) Enable S3 Transfer Acceleration
B) Place a CloudFront distribution in front of the S3 bucket
C) Enable S3 Requester Pays
D) Use S3 Intelligent-Tiering

**Q79.** A company needs to build a real-time dashboard that visualizes data from multiple AWS data sources including Athena, RDS, and Redshift. Which service should they use?

A) Amazon Managed Grafana
B) Amazon QuickSight
C) Amazon CloudWatch Dashboards
D) AWS Glue DataBrew

**Q80.** A company runs an application that requires a shared Windows file system accessible by multiple EC2 Windows instances. Which storage service should they use?

A) Amazon EFS
B) Amazon FSx for Windows File Server
C) Amazon S3
D) Amazon EBS Multi-Attach

---

## Domain 4: Design Cost-Optimized Architectures (Questions 81–100)

**Q81.** A company runs a production web application 24/7 on EC2 instances with steady, predictable traffic. They want to reduce compute costs. Which purchasing option provides the MOST savings?

A) On-Demand Instances
B) Spot Instances
C) Compute Savings Plans
D) Dedicated Hosts

**Q82.** A company stores 100 TB of data in S3 Standard. Analysis shows that 80% of the data hasn't been accessed in over 90 days, but access patterns are unpredictable. Which storage class MINIMIZES cost without risking availability?

A) S3 Standard-IA
B) S3 One Zone-IA
C) S3 Intelligent-Tiering
D) S3 Glacier Instant Retrieval

**Q83.** A company has multiple AWS accounts. They want a single view of costs across all accounts and the ability to share Reserved Instance discounts. Which feature should they use?

A) AWS Cost Explorer in each account
B) AWS Organizations with consolidated billing
C) AWS Budgets in each account
D) IAM cost allocation tags

**Q84.** A company runs a development environment with 10 RDS instances that are only used Monday–Friday, 9 AM–6 PM. How can they reduce costs the MOST?

A) Purchase Reserved Instances for all 10 databases
B) Use AWS Lambda and EventBridge to stop instances outside business hours
C) Migrate all databases to DynamoDB on-demand
D) Use smaller instance types

**Q85.** A company has a data processing pipeline that runs for 2 hours every day. The pipeline uses 20 EC2 instances and is fault-tolerant. If interrupted, the pipeline can resume from the last checkpoint. Which purchasing strategy MINIMIZES cost?

A) On-Demand Instances
B) Spot Instances with checkpointing
C) Reserved Instances (1-year, no upfront)
D) Dedicated Instances

**Q86.** A company uses NAT gateways in each of their 6 VPCs for outbound internet access from private subnets. Most traffic is to AWS services (S3, DynamoDB, STS). How can they reduce NAT gateway costs?

A) Replace NAT gateways with NAT instances
B) Create VPC gateway endpoints for S3 and DynamoDB, and interface endpoints for other AWS services
C) Move all workloads to public subnets
D) Use a single NAT gateway shared across all VPCs

**Q87.** A company runs a web application on EC2 instances behind an ALB. They notice that many instances are running at 15% CPU utilization. What should the architect recommend?

A) Add more instances for redundancy
B) Use AWS Compute Optimizer to right-size the instances to a smaller type
C) Switch to Spot Instances
D) Increase the Auto Scaling maximum capacity

**Q88.** A company stores application logs in S3. Logs older than 1 year are never accessed but must be retained for 7 years for compliance. Which storage class is MOST cost-effective for logs older than 1 year?

A) S3 Standard-IA
B) S3 One Zone-IA
C) S3 Glacier Deep Archive
D) S3 Glacier Flexible Retrieval

**Q89.** A company runs a serverless application using API Gateway and Lambda. They are concerned about Lambda costs during traffic spikes. Which Lambda feature can help control costs?

A) Provisioned concurrency
B) Reserved concurrency (setting a maximum)
C) Increasing memory allocation
D) Using Lambda@Edge instead

**Q90.** A company has a large EC2 fleet with a mix of On-Demand and Reserved Instances. They want to identify underutilized Reserved Instances. Which tool should they use?

A) AWS Trusted Advisor
B) Amazon CloudWatch
C) AWS Config
D) AWS CloudTrail

**Q91.** A company transfers 10 TB of data per month from EC2 instances in us-east-1 to the internet. They want to reduce data transfer costs. Which approach helps?

A) Use a NAT gateway instead of an internet gateway
B) Place a CloudFront distribution in front of the application to cache responses at edge locations
C) Use VPC peering
D) Enable S3 Transfer Acceleration

**Q92.** A company runs an Auto Scaling group with On-Demand instances. The application can tolerate some instances being interrupted. They want to reduce costs while maintaining availability. What should they do?

A) Use a mixed instances policy with both On-Demand and Spot Instances
B) Switch entirely to Spot Instances
C) Use Reserved Instances for the base capacity
D) Reduce the minimum capacity to zero

**Q93.** A company has an RDS MySQL database with 2 TB of storage provisioned. Actual usage is only 500 GB. How can they reduce storage costs?

A) RDS storage cannot be reduced once provisioned — they must migrate to a new instance with less storage
B) Enable storage auto scaling and reduce the allocated storage
C) Switch to Aurora, which automatically scales storage and you only pay for what you use
D) Delete unused data to free up space

**Q94.** A company uses Amazon Redshift for data warehousing. The cluster is idle 70% of the time. How can they reduce costs?

A) Use Redshift Reserved Nodes
B) Pause the cluster during idle periods and resume when needed
C) Migrate to RDS PostgreSQL
D) Reduce the number of nodes permanently

**Q95.** A company has a VPC with private subnets that need to access S3 frequently. They currently route S3 traffic through a NAT gateway. How can they eliminate the NAT gateway data processing charges for S3 traffic?

A) Use S3 Transfer Acceleration
B) Create an S3 gateway VPC endpoint (free, no data processing charges)
C) Move instances to a public subnet
D) Use AWS Direct Connect

**Q96.** A company runs a CI/CD pipeline that spins up EC2 instances for 30-minute build jobs. Builds happen 50 times per day. The builds are fault-tolerant. Which purchasing option is MOST cost-effective?

A) On-Demand Instances
B) Spot Instances
C) Reserved Instances
D) Dedicated Hosts

**Q97.** A company has an application that uses an Application Load Balancer. They notice high costs from the ALB. Investigation shows that 60% of requests are for static content. How can they reduce ALB costs?

A) Switch to a Network Load Balancer
B) Serve static content through CloudFront backed by S3, reducing requests hitting the ALB
C) Increase the instance size to handle more requests
D) Enable ALB connection draining

**Q98.** A company wants to receive alerts when their monthly AWS bill is projected to exceed $10,000. Which service should they use?

A) AWS Cost Explorer
B) AWS Budgets with a forecasted cost budget and alert threshold
C) AWS CloudTrail
D) Amazon CloudWatch billing alarms only

**Q99.** A company runs a database on an EC2 instance using a gp2 EBS volume. They need 3,000 IOPS consistently but are paying for a 1 TB gp2 volume to get that performance. How can they reduce cost?

A) Switch to a gp3 volume — gp3 provides 3,000 baseline IOPS regardless of volume size, so they can use a smaller volume
B) Switch to an io2 volume
C) Switch to an st1 volume
D) Add a second gp2 volume in a RAID 0 configuration

**Q100.** A company has multiple teams each running their own AWS accounts. They want to track costs by team, project, and environment. What should they implement?

A) Separate AWS Organizations for each team
B) Cost allocation tags applied to all resources, with tag enforcement policies
C) A single AWS account with IAM groups per team
D) AWS Budgets in each account without tags

---

