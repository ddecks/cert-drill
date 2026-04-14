## Storage Services

Q: Amazon FSx for Lustre
A: High-performance POSIX file system for HPC/ML workloads. Sub-millisecond latency. Can link directly to S3 for transparent data access. Linux only.

Q: Amazon FSx for Windows File Server
A: Fully managed Windows-native file system using SMB protocol. Integrates with Active Directory. Windows workloads only.

Q: Amazon EFS
A: Managed NFS file system for Linux. Multi-AZ, elastic (grows/shrinks automatically). POSIX-compliant.

Q: EBS gp3 — max IOPS
A: 16,000 IOPS. Baseline 3,000 IOPS regardless of volume size. Can provision IOPS independently of storage.

Q: EBS gp2 — max IOPS
A: 16,000 IOPS. Scales at 3 IOPS per GB (need 5,334 GB to hit max). Burstable to 3,000 for small volumes.

Q: EBS io2 — max IOPS
A: 64,000 IOPS. Provisioned IOPS, designed for sustained high-performance workloads.

Q: EBS io2 Block Express — max IOPS
A: 256,000 IOPS. Requires Nitro-based instances. Highest IOPS available on EBS.

Q: EBS st1
A: Throughput-optimized HDD. Max 500 IOPS, 500 MB/s throughput. For big sequential workloads (data warehouses, log processing). Cannot be a boot volume.

Q: EBS sc1
A: Cold HDD. Max 250 IOPS. Cheapest EBS volume. For infrequent access. Cannot be a boot volume.

Q: When do you need io2 vs gp3?
A: If you need more than 16,000 sustained IOPS, you must use io2 or io2 Block Express. gp3 maxes at 16K.

Q: Instance store volumes
A: Ephemeral storage physically attached to the host. Highest IOPS possible but data is LOST on stop, terminate, or hardware failure.

Q: S3 Transfer Acceleration
A: Speeds up UPLOADS to S3 by routing through CloudFront edge locations. Does NOT help with reads/downloads.

## Caching

Q: Amazon DAX
A: DynamoDB Accelerator. In-memory cache that sits in front of DynamoDB. Works ONLY with DynamoDB. Microsecond read latency.

Q: Amazon ElastiCache
A: Managed Redis or Memcached. General-purpose cache for ANY data source including RDS, Aurora, or custom apps.

Q: DAX vs ElastiCache — when to use which?
A: DAX = DynamoDB only. ElastiCache = everything else (RDS, Aurora, any relational DB). DAX literally cannot work with RDS.

Q: ElastiCache for Redis vs Memcached
A: Redis: persistence, replication, pub/sub, complex data types. Memcached: simpler, multi-threaded, no persistence. Redis is almost always the exam answer.

Q: CloudFront caching
A: Caches HTTP content at 400+ edge locations. Reduces latency for reads and reduces S3 GET request costs. Not a database cache.

## Database Services

Q: Amazon Redshift
A: Columnar data warehouse for OLAP. Petabyte-scale. Complex analytical queries. NOT for transactional workloads.

Q: Amazon Aurora
A: MySQL/PostgreSQL-compatible relational DB. OLTP (transactional). Max ~128 TB storage. Auto-scaling storage in 10 GB increments.

Q: Redshift vs Aurora — when to use which?
A: "Petabytes" or "complex analytical queries" or "data warehouse" = Redshift. "Transactional app database" = Aurora/RDS.

Q: Aurora storage model
A: Auto-scales in 10 GB increments. You only pay for storage used. Unlike RDS, you don't pre-provision storage.

Q: RDS storage limitation
A: Once provisioned, RDS storage can only increase, never decrease. To reduce storage costs, migrate to Aurora.

Q: Aurora reader endpoint
A: Load-balances read traffic across all read replicas automatically. No need to manage individual replica endpoints.

Q: Aurora failover
A: Automatically promotes a reader to writer, typically within 30 seconds. No manual intervention needed.

Q: Amazon DynamoDB
A: Fully managed NoSQL key-value and document database. Single-digit millisecond latency at any scale. Ideal for gaming leaderboards, session stores.

Q: DynamoDB on-demand vs provisioned capacity
A: On-demand: pay per request, auto-scales, no capacity planning. Provisioned: set RCU/WCU, cheaper for predictable workloads.

Q: DynamoDB hot partitions — how to fix
A: Write sharding: append a random suffix to the partition key to distribute writes across more partitions.

Q: DynamoDB Streams
A: Change data capture — records item-level changes and lets you react to them (e.g., trigger Lambda). NOT related to partition management.

Q: Redshift pause/resume
A: Stops compute charges while paused, you only pay for storage. Ideal for clusters idle 70%+ of the time.

## Data Movement & ETL

Q: AWS DMS
A: Database Migration Service. Migrates data between databases. Supports ongoing replication.

Q: AWS SCT
A: Schema Conversion Tool. Converts database schema from one engine to another (e.g., Oracle → PostgreSQL).

Q: DMS + SCT — when do you need both?
A: Heterogeneous migration (different DB engines). SCT converts the schema, DMS migrates the data.

Q: AWS DataSync
A: File/object data transfer service. Moves data between on-prem and S3/EFS/FSx. NOT for databases.

Q: Amazon Data Firehose
A: Fully managed streaming delivery to S3, OpenSearch, Redshift, Splunk. Near-real-time (~60s buffer). No consumer code needed.

Q: Amazon Kinesis Data Streams
A: Raw data stream where YOU build custom consumers and manage shards. For custom real-time processing.

Q: Firehose vs Kinesis Data Streams
A: "Deliver to S3/OpenSearch/Redshift" = Firehose (managed, no code). "Custom real-time processing" = KDS (you write consumers).

Q: AWS Glue ETL
A: Serverless ETL service for data transformation. CSV→Parquet, schema discovery, data catalog. Purpose-built for ETL jobs.

Q: Amazon Athena
A: Serverless SQL query engine on S3 data. No data loading required. Pay per TB scanned. NOT an ETL tool.

Q: Glue ETL vs Athena
A: "Transform/convert data formats" = Glue ETL. "Query data in S3 with SQL" = Athena.

Q: Amazon EMR
A: Managed Hadoop/Spark clusters for big data processing. More control than Glue but you manage the cluster.

Q: AWS Snowball
A: Physical device for massive bulk data transfer (terabytes to petabytes). Ship it to/from AWS. Still exists, not sunset.

Q: AWS Snowcone
A: Smallest Snow family device. 8 TB usable storage. Edge computing + data transfer for remote/disconnected environments.

## Analytics & Visualization

Q: Amazon QuickSight
A: Serverless BI/visualization service. Connects natively to Athena, RDS, Redshift, S3. Purpose-built for dashboards and reports.

Q: AWS Glue DataBrew
A: Visual data preparation/cleaning tool (no-code ETL). NOT for visualization or dashboards.

Q: QuickSight vs Glue DataBrew
A: "Dashboard/visualization/BI" = QuickSight. "Data preparation/cleaning" = DataBrew.

Q: Amazon Managed Grafana
A: Managed Grafana for operational dashboards — metrics, logs, traces. More DevOps-focused than QuickSight.

Q: CloudWatch Dashboards
A: AWS resource metrics dashboards only. Limited to CloudWatch data sources.

## Networking & VPC

Q: Gateway VPC endpoint
A: For S3 and DynamoDB ONLY. FREE (no hourly or data charges). Added as a route table entry.

Q: Interface VPC endpoint
A: For most other AWS services. Costs money (hourly + per GB). Uses an ENI in your subnet.

Q: VPC endpoint policy
A: Controls WHAT resources are accessible through the endpoint. Can whitelist specific S3 bucket ARNs to prevent data exfiltration.

Q: NAT gateway
A: Allows private subnet resources to access the internet. Charges $0.045/GB for data processing. NOT needed for AWS service traffic if using VPC endpoints.

Q: When to use VPC endpoints vs NAT gateway
A: Traffic to AWS services (S3, DynamoDB, STS) = VPC endpoints (cheaper/free). Traffic to the internet = NAT gateway.

Q: Transit Gateway
A: Hub-and-spoke network connectivity for multiple VPCs and on-prem networks. Centralized routing.

Q: VPC peering
A: Connects two VPCs directly. NOT for accessing AWS services. Non-transitive (A↔B and B↔C doesn't mean A↔C).

Q: AWS Global Accelerator
A: Two static anycast IPs → nearest edge → AWS backbone to your ALB/NLB. Optimizes TCP/UDP routing, not HTTP caching.

Q: CloudFront vs Global Accelerator
A: CloudFront = HTTP content caching CDN. Global Accelerator = TCP/UDP routing optimization over AWS backbone.

## Load Balancers

Q: ALB (Application Load Balancer)
A: Layer 7 (HTTP/HTTPS). Dynamic IPs behind a DNS name. Supports path/host-based routing, WebSocket, SSL termination.

Q: NLB (Network Load Balancer)
A: Layer 4 (TCP/UDP). Supports static IPs via Elastic IP per AZ. Ultra-low latency. For firewall whitelisting.

Q: "Static IP" or "firewall whitelisting" in an exam question means...
A: NLB. ALB only has dynamic IPs behind a DNS name — cannot be whitelisted by IP.

Q: SSL termination at ALB
A: ALB terminates SSL/TLS and forwards plain HTTP to instances. Offloads CPU-intensive crypto work from EC2.

Q: ALB WebSocket support
A: ALB natively supports WebSocket connections. Upgrades HTTP connection to WebSocket at the ALB.

Q: Internal ALB
A: Load balancer only accessible within the VPC. Used for app tier in 3-tier architecture (public ALB → web → internal ALB → app → DB).

## Security Services

Q: Amazon GuardDuty
A: Threat DETECTION. Analyzes CloudTrail, VPC Flow Logs, DNS logs to find unauthorized access, compromised instances.

Q: Amazon Inspector
A: Vulnerability SCANNING. Finds software CVEs and network exposure on EC2 instances and containers.

Q: Amazon Macie
A: Sensitive data DISCOVERY. Finds PII and sensitive data in S3 buckets using ML.

Q: AWS WAF
A: HTTP request FILTERING. Blocks SQL injection, XSS, geo-blocking, rate limiting. Works with ALB, CloudFront, API Gateway.

Q: AWS Shield Standard
A: Free DDoS protection for Layer 3/4 attacks. Automatically enabled for all AWS accounts.

Q: AWS Shield Advanced
A: Paid DDoS protection including Layer 7. Includes cost protection (credits for scaling during attack) and 24/7 DRT access.

Q: GuardDuty vs Inspector
A: GuardDuty = threat detection (who's attacking?). Inspector = vulnerability scanning (what's exposed?).

## IAM & Access Control

Q: aws:CurrentTime condition key
A: Time-based restriction in IAM policies or trust policies. Used to restrict role assumption to business hours.

Q: aws:SourceVpc condition key
A: Restricts access to requests originating from a specific VPC.

Q: aws:sourceVpce condition key
A: Restricts access to requests coming through a specific VPC endpoint.

Q: aws:MultiFactorAuthPresent condition key
A: Enforces MFA. Deny all actions when MFA is not present to force users to set up MFA.

Q: SCP (Service Control Policy)
A: Preventive guardrail across AWS Organizations. Cannot be overridden by IAM policies in member accounts. Denies trump allows.

Q: IAM Identity Center (SSO)
A: Centralized access management across multiple AWS accounts. Federates with corporate Active Directory. Purpose-built for multi-account SSO.

Q: Amazon Cognito identity pools
A: Exchange user pool tokens or social identity tokens for temporary AWS credentials. For mobile/web apps accessing AWS services directly.

## Encryption

Q: SSE-S3
A: S3-managed encryption. AWS manages everything. Zero control over keys. Automatic.

Q: SSE-KMS with AWS managed key
A: KMS encryption with audit logs via CloudTrail. No key policy control. Auto-rotates.

Q: SSE-KMS with customer managed CMK
A: Full control over key policies + automatic annual rotation + audit logs. Most common exam answer for "company controls keys."

Q: SSE-C
A: Customer-provided keys. You send the key with every request. AWS never stores the key. Highest operational overhead.

Q: Default encryption vs enforcement on S3
A: Default encryption is a fallback — callers can override it. A bucket policy with deny condition is the only way to ENFORCE a specific key.

## Resilience & DR

Q: Multi-AZ (RDS)
A: Synchronous replication to standby in another AZ. Automatic failover. For HA, NOT read scaling.

Q: Read Replica (RDS)
A: Asynchronous replication. For read scaling. Manual promotion required. Can be cross-Region.

Q: Multi-AZ vs Read Replica
A: "Automatic failover + minimal data loss" = Multi-AZ. "Read scaling + reporting" = Read Replica.

Q: Pilot Light DR strategy
A: Core infrastructure (DB replicas) running at minimal cost. Scale compute on failover. RTO: hours. Low cost.

Q: Warm Standby DR strategy
A: Scaled-down but fully functional environment. RTO: minutes. Moderate cost.

Q: Active-Active DR strategy
A: Full capacity in both sites. Near-zero RTO. Most expensive.

Q: DR strategy for "minimize cost + RTO 4 hours"
A: Pilot Light. Cheapest option that can achieve RTO in hours.

Q: Sticky sessions vs externalized state
A: Sticky sessions pin user to one instance but session is LOST on termination. Externalize to ElastiCache/DynamoDB for truly stateless tier.

## S3 Storage Classes

Q: S3 Standard
A: Frequent access. Highest storage cost. No retrieval fees. No minimum storage duration.

Q: S3 Standard-IA
A: Known infrequent access. Lower storage cost but has retrieval fees. 30-day minimum storage duration.

Q: S3 One Zone-IA
A: Same as Standard-IA but single AZ. Lower availability. Cheaper. Don't use if availability matters.

Q: S3 Intelligent-Tiering
A: Unpredictable access patterns. Auto-moves between frequent/infrequent tiers. No retrieval fees. Small monitoring fee per object.

Q: S3 Glacier Instant Retrieval
A: Archive with millisecond retrieval. For data accessed once per quarter.

Q: S3 Glacier Flexible Retrieval
A: Archive with minutes-to-hours retrieval (expedited: 1-5 min, standard: 3-5 hrs, bulk: 5-12 hrs).

Q: S3 Glacier Deep Archive
A: Cheapest storage (~$1/TB/month). 12-48 hour retrieval. For data retained years for compliance, never accessed.

Q: "Unpredictable access patterns" in an exam question means...
A: S3 Intelligent-Tiering. NOT Standard-IA (which has retrieval fees that could spike unpredictably).

## Compute & Containers

Q: ECS on Fargate
A: Serverless containers. No EC2 instances to manage. Define the task, Fargate handles infrastructure.

Q: ECS Service Auto Scaling
A: Scales tasks (containers), not instances. Use custom CloudWatch metrics like SQS queue depth.

Q: API Gateway + Lambda
A: Serverless API pattern. Scales to zero when idle (no cost). Handles thousands of concurrent requests. For bursty/unpredictable traffic.

Q: Compute Savings Plans
A: Up to 66% savings for steady 24/7 workloads. Flexible across instance types, regions, OS. More flexible than Reserved Instances.

Q: Spot Instances — when to use
A: Fault-tolerant workloads that can be interrupted. Batch processing, CI/CD builds, data analysis. Up to 90% savings.

Q: Mixed instances policy (ASG)
A: On-Demand for baseline capacity + Spot for burst. Balances cost savings with availability.

## Messaging & Queues

Q: SQS Standard vs FIFO
A: Standard: at-least-once, unlimited throughput. FIFO: exactly-once, 300 msg/s per message group (3,000 with high throughput).

Q: SNS fan-out pattern
A: SNS topic → multiple SQS queues. Each consumer gets every message independently. For pub/sub.

Q: Single SQS queue + multiple consumers
A: Competing consumers pattern. Each message goes to only ONE consumer. NOT fan-out.

Q: Amazon MQ
A: Managed ActiveMQ/RabbitMQ. Supports MQTT, AMQP, STOMP, JMS. For migrating legacy on-prem message queues.

Q: "Specific protocol mentioned (MQTT, AMQP, JMS)" in an exam question means...
A: Amazon MQ. SQS/SNS only support AWS-native APIs.

Q: Dead-letter queue (DLQ)
A: Captures messages that fail processing repeatedly (after maxReceiveCount). Prevents poison messages from blocking the main queue.

Q: SQS FIFO + idempotency
A: FIFO queues handle deduplication at the queue level. "App can't be modified for idempotency" = use FIFO queue.

## Route 53

Q: Latency-based routing
A: Routes users to the Region with lowest network latency. "Fastest response time" = latency-based.

Q: Geolocation routing
A: Routes by user's geographic location. Doesn't always mean lowest latency (border cases).

Q: Failover routing
A: Active-passive. Health checks on primary, auto-routes to secondary when primary is unhealthy.

Q: Latency-based vs Geolocation routing
A: "Fastest response" = latency-based. "Users in France go to EU region" = geolocation.

## Cost Optimization

Q: S3 gateway VPC endpoint cost
A: FREE. No hourly charge, no data processing charge. NAT gateway charges $0.045/GB.

Q: AWS Budgets vs CloudWatch billing alarms
A: Budgets: forecasted cost alerts (projected spend). CloudWatch: actual spend alerts only.

Q: AWS Trusted Advisor
A: Checks for underutilized Reserved Instances, idle resources, security gaps. Cost optimization recommendations.

Q: AWS Compute Optimizer
A: Analyzes CloudWatch metrics and recommends right-sized instance types. For over-provisioned EC2.

Q: Reserved concurrency (Lambda)
A: Sets a MAXIMUM limit on concurrent executions. Caps cost during spikes. NOT the same as provisioned concurrency (which pre-warms and increases cost).
