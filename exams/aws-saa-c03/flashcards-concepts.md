## Design Secure Architectures

Q: What is the principle of least privilege?
A: Grant only the minimum permissions required to perform a task. In AWS, this means scoping IAM policies to specific actions, resources, and conditions rather than using wildcards (*).

Q: How does an IAM role differ from an IAM user?
A: A role has no long-term credentials (no password or access keys). Instead, it provides temporary security credentials via STS AssumeRole. Roles are assumed by trusted entities (EC2 instances, Lambda functions, other accounts).

Q: What is a resource-based policy and how does it differ from an identity-based policy?
A: Identity-based policies attach to IAM users/roles and say "this identity can do X." Resource-based policies attach to resources (S3 buckets, SQS queues, KMS keys) and say "who can access this resource." Resource-based policies enable cross-account access without assuming a role.

Q: How does S3 bucket policy evaluation work when both an IAM policy and bucket policy exist?
A: AWS evaluates the union of all applicable policies. An explicit Deny in any policy always wins. If there's no explicit Deny, there must be an explicit Allow in at least one policy. For cross-account access, BOTH the IAM policy and the bucket policy must allow the action.

Q: What is the shared responsibility model?
A: AWS is responsible for security OF the cloud (physical infrastructure, hypervisor, managed service internals). The customer is responsible for security IN the cloud (OS patching, network config, IAM, encryption, application code).

Q: What does AWS KMS envelope encryption do?
A: KMS generates a data key. The data key encrypts your data (locally). KMS then encrypts the data key with a CMK. You store the encrypted data key alongside the ciphertext. This avoids sending large payloads to KMS and lets you encrypt data larger than 4 KB.

Q: What is the difference between symmetric and asymmetric KMS keys?
A: Symmetric keys use a single 256-bit AES key that never leaves KMS — all encrypt/decrypt happens via API calls. Asymmetric keys have a public/private pair; the public key can be downloaded for client-side encryption or signature verification outside AWS.

Q: How does AWS Organizations SCP inheritance work?
A: SCPs are inherited down the OU tree. An account's effective permissions are the intersection of all SCPs from the root down to the account's OU, combined with the account's IAM policies. SCPs don't grant permissions — they set the maximum boundary.

Q: What is a VPC security group vs a network ACL?
A: Security groups are stateful (return traffic automatically allowed), operate at the instance/ENI level, and only support allow rules. NACLs are stateless (must explicitly allow return traffic), operate at the subnet level, and support both allow and deny rules with numbered priority.

Q: How does AWS Certificate Manager (ACM) work with load balancers?
A: ACM provisions and auto-renews free public SSL/TLS certificates. You attach them to ALB, NLB, or CloudFront. The certificate terminates TLS at the load balancer, offloading encryption from your instances.

Q: What is AWS Secrets Manager and how does it differ from Parameter Store?
A: Secrets Manager is purpose-built for secrets (DB credentials, API keys) with automatic rotation via Lambda. Parameter Store is a general-purpose key-value store — it can hold secrets but has no built-in rotation. Secrets Manager costs $0.40/secret/month; Parameter Store standard tier is free.

Q: What is a VPC flow log?
A: A capture of IP traffic metadata (source/dest IP, ports, protocol, action, bytes) going to/from network interfaces in a VPC. Published to CloudWatch Logs or S3. Does NOT capture packet contents — only metadata.

Q: What does enabling S3 Object Lock do?
A: Prevents objects from being deleted or overwritten for a fixed retention period (retention mode) or indefinitely (legal hold). Governance mode allows users with special permissions to override; Compliance mode cannot be overridden by anyone, including the root account.

Q: How does AWS PrivateLink work?
A: PrivateLink exposes a service via an interface VPC endpoint (ENI with a private IP in your subnet). Traffic stays on the AWS network and never traverses the public internet. Used for accessing AWS services or your own services across VPCs/accounts privately.

Q: What is the purpose of an S3 access point?
A: Access points are named network endpoints with dedicated access policies, simplifying access management for shared S3 buckets. Each access point can be scoped to a specific VPC and has its own DNS name and policy, replacing complex bucket policies.

## Design Resilient Architectures

Q: What does "loosely coupled" mean in AWS architecture?
A: Components communicate through well-defined interfaces (queues, event buses, APIs) rather than direct dependencies. If one component fails, others continue operating. SQS queues between tiers is a classic example.

Q: What is the difference between horizontal and vertical scaling?
A: Vertical scaling (scale up) means increasing the size of a single instance (more CPU/RAM). Horizontal scaling (scale out) means adding more instances. Horizontal scaling provides better fault tolerance because no single point of failure exists.

Q: How does an Auto Scaling group achieve high availability?
A: By distributing instances across multiple Availability Zones. If one AZ fails, ASG launches replacement instances in healthy AZs. The desired capacity is maintained automatically.

Q: What is an Availability Zone?
A: One or more discrete data centers within a Region, each with independent power, cooling, and networking. AZs within a Region are connected by low-latency links but are physically separated to isolate failures.

Q: What is the difference between RPO and RTO?
A: RPO (Recovery Point Objective) is the maximum acceptable data loss measured in time — how far back you can afford to lose. RTO (Recovery Time Objective) is the maximum acceptable downtime — how quickly you must recover.

Q: How does S3 cross-region replication work?
A: Asynchronously copies objects from a source bucket to a destination bucket in another Region. Requires versioning enabled on both buckets. Replicates new objects and updates; does NOT replicate existing objects retroactively (use S3 Batch Replication for that).

Q: What is an ELB health check and how does it affect availability?
A: The load balancer periodically sends requests to registered targets. If a target fails consecutive health checks (configurable threshold), it's marked unhealthy and removed from rotation. Traffic only goes to healthy targets.

Q: How does Amazon SQS help with resilience?
A: SQS decouples producers from consumers. If a consumer crashes, messages remain in the queue (up to 14 days retention). When the consumer recovers, it processes the backlog. The producer never needs to know about consumer failures.

Q: What is the difference between a stateful and stateless application tier?
A: Stateful tiers store session data locally on the instance — if the instance dies, the session is lost. Stateless tiers store session data externally (ElastiCache, DynamoDB) so any instance can serve any request, enabling seamless scaling and failover.

Q: How does Route 53 health checking enable failover?
A: Route 53 monitors endpoint health via HTTP/HTTPS/TCP checks from multiple global locations. If the primary endpoint fails health checks, Route 53 automatically routes DNS queries to a secondary endpoint (failover routing policy).

Q: What is the Aurora Global Database and when would you use it?
A: Aurora Global Database replicates an Aurora cluster across Regions with sub-second replication lag. The secondary Region can be promoted to primary in under a minute. Use it for disaster recovery and low-latency global reads.

Q: What is the difference between backup and snapshots in AWS?
A: EBS snapshots are point-in-time copies stored in S3 (incremental after the first). RDS automated backups include daily snapshots plus transaction logs for point-in-time recovery. Both are Region-specific but can be copied cross-Region.

Q: How does DynamoDB Global Tables provide multi-Region resilience?
A: Global Tables replicate a DynamoDB table across multiple Regions with active-active writes. Each Region can read and write locally. Conflicts are resolved with a last-writer-wins strategy. Requires DynamoDB Streams enabled.

Q: What is the purpose of an SQS visibility timeout?
A: When a consumer receives a message, it becomes invisible to other consumers for the visibility timeout period. If the consumer processes and deletes it in time, it's gone. If not, it reappears in the queue for another consumer to retry.

## Design High-Performing Architectures

Q: What is the difference between a Region, AZ, and edge location?
A: A Region is a geographic area with multiple AZs. An AZ is one or more data centers within a Region. An edge location is a CloudFront/Global Accelerator point of presence for caching content close to users — there are 400+ edge locations vs ~30 Regions.

Q: How does Amazon CloudFront improve performance?
A: CloudFront caches content at edge locations worldwide. When a user requests content, it's served from the nearest edge location instead of the origin server. This reduces latency and offloads traffic from the origin.

Q: What is connection draining (deregistration delay) on a load balancer?
A: When an instance is deregistered or fails health checks, the load balancer stops sending new requests but allows in-flight requests to complete within the deregistration delay period (default 300 seconds) before forcibly closing connections.

Q: How does EBS performance differ between gp3 and io2?
A: gp3 provides a baseline of 3,000 IOPS and 125 MB/s, independently scalable up to 16,000 IOPS and 1,000 MB/s. io2 provides up to 64,000 IOPS with a guaranteed IOPS-to-storage ratio of 500:1. io2 is for workloads needing sustained high IOPS beyond gp3's ceiling.

Q: What is S3 multipart upload and when should you use it?
A: Multipart upload breaks a large file into parts uploaded in parallel, then assembles them. AWS recommends it for files over 100 MB. It improves throughput, allows pause/resume, and handles network failures per-part rather than restarting the whole upload.

Q: How does Amazon ElastiCache improve application performance?
A: ElastiCache stores frequently accessed data in memory (microsecond latency) instead of querying a database (millisecond latency). Common patterns: cache database query results, store session data, cache API responses.

Q: What is read replica lag and why does it matter?
A: Read replicas use asynchronous replication, so there's a delay (lag) between the primary and replica. Applications reading from replicas may see stale data. For strong consistency, read from the primary. Replicas are for eventual consistency workloads.

Q: How does S3 request rate performance work?
A: S3 automatically scales to 3,500 PUT/COPY/POST/DELETE and 5,500 GET/HEAD requests per second per prefix. Distributing objects across multiple prefixes multiplies this throughput linearly.

Q: What is placement group and what are the three types?
A: Placement groups control EC2 instance placement. Cluster: packs instances close together in one AZ for low-latency networking (HPC). Spread: places instances on distinct hardware to reduce correlated failures (max 7 per AZ). Partition: groups instances into logical partitions on separate racks (big distributed workloads like HDFS/Cassandra).

Q: How does Lambda concurrency work?
A: Each concurrent request gets its own execution environment. Lambda scales automatically up to the account's concurrency limit (default 1,000 per Region). Cold starts occur when new environments are created. Provisioned concurrency pre-warms environments to eliminate cold starts.

Q: What is Amazon EFS performance mode vs throughput mode?
A: Performance mode: General Purpose (low latency, default) or Max I/O (higher aggregate throughput, slightly higher latency). Throughput mode: Bursting (scales with storage size), Provisioned (set throughput independent of size), or Elastic (auto-scales throughput).

Q: How does DynamoDB partition key design affect performance?
A: DynamoDB distributes data across partitions based on the partition key hash. A poorly chosen key (e.g., date) concentrates traffic on one partition, creating a hot partition. A high-cardinality key (e.g., user_id) distributes traffic evenly.

Q: What is the difference between synchronous and asynchronous replication?
A: Synchronous replication waits for the replica to confirm the write before acknowledging to the client — guarantees zero data loss but adds latency (used by Multi-AZ RDS). Asynchronous replication acknowledges immediately and replicates in the background — lower latency but potential data loss (used by read replicas).

Q: What is Amazon Global Accelerator's anycast IP approach?
A: Global Accelerator provides two static anycast IPs that route traffic to the nearest AWS edge location, then over the AWS global backbone to your endpoint. Unlike DNS-based routing, anycast IPs are instantly effective with no TTL/caching delays.

## Design Cost-Optimized Architectures

Q: How does S3 Lifecycle policy reduce costs?
A: Lifecycle rules automatically transition objects between storage classes based on age (e.g., Standard → Standard-IA after 30 days → Glacier after 90 days → Deep Archive after 180 days) and can expire/delete objects after a set period.

Q: What is the difference between On-Demand, Reserved, and Spot pricing?
A: On-Demand: pay by the second, no commitment, highest price. Reserved: 1 or 3 year commitment for up to 72% savings on steady-state workloads. Spot: bid on unused capacity for up to 90% savings, but instances can be interrupted with 2-minute notice.

Q: How do Savings Plans differ from Reserved Instances?
A: Savings Plans commit to a $/hour spend (not a specific instance type). Compute Savings Plans apply across EC2, Fargate, and Lambda in any Region or instance family. More flexible than RIs, which lock you to a specific instance type and Region (unless convertible).

Q: What is the cost difference between NAT Gateway and VPC endpoints?
A: NAT Gateway charges $0.045/hour plus $0.045/GB processed. S3/DynamoDB Gateway VPC endpoints are completely free. For heavy AWS service traffic, VPC endpoints can save significant money.

Q: How does S3 Intelligent-Tiering save money?
A: It automatically moves objects between frequent and infrequent access tiers based on usage patterns. No retrieval fees when objects move between tiers. You pay a small monitoring fee per object ($0.0025 per 1,000 objects). Ideal when access patterns are unpredictable.

Q: What is right-sizing and how does AWS Compute Optimizer help?
A: Right-sizing means matching instance types to actual workload requirements. Compute Optimizer analyzes CloudWatch metrics (CPU, memory, network) over 14 days and recommends optimal instance types. Over-provisioned instances waste money; under-provisioned ones hurt performance.

Q: When should you use Fargate vs EC2 for ECS?
A: Fargate: pay per task (vCPU + memory per second), no instance management, ideal for variable/bursty workloads. EC2: manage your own instances, better for steady-state workloads where you can use Reserved Instances/Savings Plans, and when you need GPU or specific instance types.

Q: How does S3 Requester Pays work?
A: The requester (not the bucket owner) pays for data transfer and request costs. The bucket owner still pays for storage. Useful for sharing large datasets — the consumer bears the access cost.

Q: What is the cost impact of data transfer in AWS?
A: Data transfer IN to AWS is free. Data transfer OUT to the internet is charged per GB (tiered pricing). Data transfer between AZs costs $0.01/GB each way. Data transfer within the same AZ using private IPs is free. These costs add up quickly in high-traffic architectures.

Q: How does Reserved capacity work for DynamoDB?
A: DynamoDB Reserved Capacity commits to a minimum provisioned throughput (RCU/WCU) for 1 or 3 years at a significant discount. Applies only to provisioned mode tables. For unpredictable workloads, on-demand mode avoids over-provisioning.

Q: What is the cost advantage of Aurora Serverless v2?
A: Aurora Serverless v2 scales compute in fine-grained increments (0.5 ACU) based on demand. You pay only for the capacity used. Ideal for variable workloads — avoids paying for a large provisioned instance during idle periods while scaling up for peaks.

Q: How do you reduce data transfer costs between Regions?
A: Use CloudFront to cache content at edge locations (cheaper than direct transfer). Use S3 Transfer Acceleration for uploads. Compress data before transfer. Architect to keep traffic within a single Region when possible.

Q: What is the cost model for Lambda?
A: You pay per request ($0.20 per million) plus per GB-second of compute time. No charge when code isn't running. Free tier includes 1M requests and 400,000 GB-seconds per month. Cost-effective for sporadic workloads; can be expensive for sustained high-throughput.

Q: How does deleting unused EBS snapshots save money?
A: EBS snapshots are stored in S3 and charged per GB-month. Incremental snapshots mean only changed blocks are stored, but old snapshots still accumulate cost. Regularly auditing and deleting unneeded snapshots (especially from terminated instances) reduces storage costs.
