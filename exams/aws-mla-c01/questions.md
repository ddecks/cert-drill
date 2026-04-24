# AWS MLA-C01 Practice Exam — 100 Questions

Weighted by exam domain:
- Domain 1: Data Preparation for Machine Learning (Q1–Q28) — 28%
- Domain 2: ML Model Development (Q29–Q54) — 26%
- Domain 3: Deployment and Orchestration of ML Workflows (Q55–Q76) — 22%
- Domain 4: ML Solution Monitoring, Maintenance, and Security (Q77–Q100) — 24%

Passing target: 720/1000 (~72%). Aim for 85%+ in practice.

---

## Domain 1: Data Preparation for Machine Learning (Questions 1–28)

**Q1.** A data engineer needs to ingest streaming clickstream data from a web application and store it in Amazon S3 for downstream ML training. The data must arrive within 60 seconds. Which service should they use?

A) Amazon Kinesis Data Streams with a custom consumer
B) Amazon Kinesis Data Firehose with S3 as the destination
C) Amazon SQS with a Lambda function writing to S3
D) AWS Database Migration Service

**Q2.** A team is preparing tabular training data stored in CSV files on S3. They need to perform visual data exploration, detect anomalies, and apply transformations without writing code. Which AWS service is MOST appropriate?

A) Amazon SageMaker Studio notebooks
B) AWS Glue DataBrew
C) Amazon Athena
D) AWS Lambda

**Q3.** An ML engineer has a dataset with a categorical feature containing 200 unique city names. Which encoding technique is MOST appropriate to avoid creating an excessively high-dimensional feature space?

A) One-hot encoding
B) Label encoding
C) Target encoding
D) Binary encoding

**Q4.** A dataset for a fraud detection model contains 99.5% legitimate transactions and 0.5% fraudulent transactions. Which technique should the ML engineer use to address this class imbalance? (Select TWO)

A) Remove the majority class samples until balanced
B) Apply SMOTE to generate synthetic minority samples
C) Use a cost-sensitive learning algorithm with higher weight on the minority class
D) Increase the batch size during training
E) Convert the problem to unsupervised learning

**Q5.** An ML engineer needs to create labeled training data for an image classification model. The team has 100,000 unlabeled images. Which AWS service provides a managed data labeling workflow with human annotators?

A) Amazon Rekognition Custom Labels
B) Amazon SageMaker Ground Truth
C) Amazon Mechanical Turk directly
D) Amazon Augmented AI (A2I)

**Q6.** A data pipeline ingests JSON records from multiple sources into S3. Some records have missing fields, duplicates, and inconsistent date formats. Which AWS service can the engineer use to build a serverless ETL job to clean and transform this data?

A) Amazon EMR
B) AWS Glue
C) Amazon Redshift Spectrum
D) AWS Step Functions

**Q7.** An ML engineer needs to store training data in a columnar format that supports efficient compression and is optimized for analytical queries. Which file format should they choose?

A) CSV
B) JSON
C) Apache Parquet
D) Apache Avro

**Q8.** A team is building a feature store for their ML models. They need both an online store for low-latency inference lookups and an offline store for batch training. Which AWS service provides this capability?

A) Amazon DynamoDB and Amazon S3 used separately
B) Amazon SageMaker Feature Store
C) Amazon ElastiCache and Amazon Redshift
D) AWS Glue Data Catalog

**Q9.** An ML engineer discovers that a numerical feature in their dataset has extreme outliers that are skewing model training. Which transformation technique should they apply?

A) One-hot encoding
B) Min-max normalization
C) Robust scaling using median and interquartile range
D) Label encoding

**Q10.** A company needs to catalog and discover datasets across multiple S3 buckets and RDS databases for ML workloads. They need a centralized metadata repository. Which service should they use?

A) AWS Glue Data Catalog
B) Amazon Macie
C) AWS Lake Formation
D) Amazon S3 Inventory

**Q11.** An ML engineer is preparing text data for a natural language processing model. The raw text contains HTML tags, special characters, and inconsistent casing. Which preprocessing steps should they apply? (Select TWO)

A) One-hot encode each word
B) Remove HTML tags and special characters
C) Convert all text to lowercase for consistency
D) Convert text to Parquet format
E) Apply min-max scaling to the text

**Q12.** A data engineer needs to convert a large collection of CSV files in S3 to Apache Parquet format for more efficient ML training. Which approach is MOST cost-effective for a one-time conversion?

A) Use an Amazon EMR cluster with Apache Spark
B) Use an AWS Glue ETL job
C) Download files locally and convert with pandas
D) Use Amazon Athena CTAS (CREATE TABLE AS SELECT)

**Q13.** An ML engineer needs to split their dataset into training, validation, and test sets. The dataset has a time-series component with timestamps. What is the correct approach?

A) Randomly shuffle and split 80/10/10
B) Use stratified random sampling
C) Split chronologically — earliest data for training, middle for validation, latest for test
D) Use k-fold cross-validation

**Q14.** A team is using Amazon SageMaker Data Wrangler to prepare data. They want to reuse the same transformation pipeline across multiple datasets. What should they do?

A) Export the Data Wrangler flow as a SageMaker Pipeline
B) Manually recreate the transformations each time
C) Save the transformed data and skip the pipeline
D) Use Amazon Athena views instead

**Q15.** An ML engineer needs to detect and measure pre-training bias in a dataset used for a loan approval model. Which tool should they use?

A) Amazon SageMaker Debugger
B) Amazon SageMaker Clarify
C) Amazon SageMaker Model Monitor
D) AWS Glue DataBrew

**Q16.** A dataset contains a feature with values ranging from 0 to 1,000,000 and another feature ranging from 0 to 1. An ML engineer wants to ensure both features contribute equally during model training. Which technique should they apply?

A) Feature hashing
B) Feature standardization (z-score normalization)
C) One-hot encoding
D) Principal Component Analysis

**Q17.** An ML engineer needs to process 50 TB of raw log data stored in S3 to extract features for model training. The processing requires distributed computing. Which service is MOST appropriate?

A) AWS Lambda
B) Amazon SageMaker Processing jobs with Spark
C) Amazon Athena
D) AWS Glue DataBrew

**Q18.** A company stores sensitive customer data that will be used for ML training. They must ensure PII is removed before the data is used. Which approach should they take?

A) Use Amazon Macie to identify PII, then use AWS Glue to mask or remove it
B) Manually review all records
C) Encrypt the data with KMS and use it as-is
D) Store the data in a private S3 bucket with no additional processing

**Q19.** An ML engineer is working with a dataset where 30% of the values in a critical numerical feature are missing. Which imputation strategy is MOST appropriate?

A) Drop all rows with missing values
B) Fill missing values with the column mean or median
C) Fill missing values with zero
D) Drop the entire feature

**Q20.** A team needs to stream real-time sensor data from IoT devices, apply transformations, and write the results to S3 for ML training. The transformations include filtering, aggregation, and windowing. Which service should they use?

A) Amazon Kinesis Data Firehose
B) Amazon Managed Service for Apache Flink
C) Amazon SQS with Lambda
D) AWS IoT Core rules engine writing directly to S3

**Q21.** An ML engineer needs to tokenize text data for input to a transformer-based model. Which tokenization approach is MOST commonly used with models like BERT?

A) Character-level tokenization
B) Word-level tokenization with whitespace splitting
C) WordPiece or Byte-Pair Encoding (BPE) subword tokenization
D) Sentence-level tokenization

**Q22.** A data engineer needs to ensure that an S3-based data lake used for ML training has proper access controls, data encryption, and audit logging. Which AWS service provides centralized governance for the data lake?

A) Amazon S3 Access Points
B) AWS Lake Formation
C) AWS IAM alone
D) Amazon Macie

**Q23.** An ML engineer has a dataset with 500 features. Many features are highly correlated. They want to reduce dimensionality while preserving the most variance. Which technique should they use?

A) L1 regularization
B) Principal Component Analysis (PCA)
C) One-hot encoding
D) Feature hashing

**Q24.** A team is building an ML pipeline that requires data validation at each stage. They need to automatically check for schema changes, missing values, and statistical drift in incoming data. Which approach is MOST appropriate?

A) Write custom Lambda functions for each check
B) Use Amazon SageMaker Processing jobs with a data validation library like Great Expectations
C) Use Amazon Athena queries to spot-check data
D) Rely on SageMaker training job error messages

**Q25.** An ML engineer needs to join data from an Amazon RDS MySQL database with data in S3 to create a training dataset. Which approach is MOST efficient?

A) Export RDS data to CSV, upload to S3, and join with AWS Glue
B) Use Amazon Athena federated queries to join RDS and S3 data
C) Copy S3 data into RDS and join with SQL
D) Use a SageMaker notebook to read from both sources and join with pandas

**Q26.** A dataset contains images of varying sizes and formats (JPEG, PNG, BMP). An ML engineer needs to prepare them for training a convolutional neural network. Which preprocessing steps are necessary? (Select TWO)

A) Resize all images to a consistent dimension
B) Convert all images to CSV format
C) Normalize pixel values to a standard range (e.g., 0-1)
D) Apply one-hot encoding to pixel values
E) Store images in Apache Parquet format

**Q27.** An ML engineer is using Amazon SageMaker and needs to provide training data in a format optimized for sequential reading with high throughput. Which data format should they use?

A) CSV
B) JSON Lines
C) Amazon SageMaker RecordIO (protobuf)
D) Apache Avro

**Q28.** A company wants to track the lineage of their ML training data — knowing which raw datasets were used, what transformations were applied, and which model version consumed the data. Which AWS capability supports this?

A) Amazon S3 versioning
B) Amazon SageMaker ML Lineage Tracking
C) AWS CloudTrail
D) AWS Config

---

## Domain 2: ML Model Development (Questions 29–54)

**Q29.** A company wants to build a chatbot that can answer questions about their internal documentation. Which AWS service allows them to use foundation models with retrieval-augmented generation (RAG)?

A) Amazon SageMaker JumpStart
B) Amazon Bedrock with Knowledge Bases
C) Amazon Lex
D) Amazon Comprehend

**Q30.** An ML engineer needs to train a custom image classification model but has limited labeled data (only 500 images per class). Which approach is MOST likely to produce good results?

A) Train a deep CNN from scratch
B) Use transfer learning with a pre-trained model and fine-tune on the small dataset
C) Use a simple logistic regression model
D) Augment the data to 1 million images and train from scratch

**Q31.** A data scientist is training a model and observes that training accuracy is 99% but validation accuracy is 65%. What is the MOST likely problem?

A) Underfitting
B) Overfitting
C) The learning rate is too low
D) The dataset is too large

**Q32.** An ML engineer needs to automatically find the best hyperparameters for a SageMaker training job. Which SageMaker feature should they use?

A) SageMaker Autopilot
B) SageMaker Automatic Model Tuning (Hyperparameter Optimization)
C) SageMaker Debugger
D) SageMaker Experiments

**Q33.** A team needs to classify customer support tickets into 15 categories using text data. They want to use a SageMaker built-in algorithm. Which algorithm is MOST appropriate?

A) XGBoost
B) BlazingText in supervised mode
C) K-Means
D) Random Cut Forest

**Q34.** An ML engineer is training a neural network and notices the loss function is oscillating and not converging. What should they try FIRST?

A) Increase the number of epochs
B) Reduce the learning rate
C) Add more layers to the network
D) Increase the batch size to the maximum

**Q35.** A company wants to extract entities (names, dates, locations) from unstructured text documents without training a custom model. Which AWS service should they use?

A) Amazon Textract
B) Amazon Comprehend
C) Amazon Translate
D) Amazon Kendra

**Q36.** An ML engineer is evaluating a binary classification model for disease detection. False negatives (missing a disease) are much more costly than false positives. Which metric should they prioritize?

A) Accuracy
B) Precision
C) Recall (Sensitivity)
D) Specificity

**Q37.** A team is training a model using SageMaker and wants to track experiments, compare metrics across runs, and organize trial components. Which SageMaker feature should they use?

A) SageMaker Model Registry
B) SageMaker Experiments
C) SageMaker Debugger
D) SageMaker Pipelines

**Q38.** An ML engineer needs to detect anomalies in time-series data from manufacturing sensors. They have no labeled anomaly data. Which SageMaker built-in algorithm is designed for this?

A) XGBoost
B) DeepAR
C) Random Cut Forest
D) Linear Learner

**Q39.** A data scientist is training a deep learning model and wants to prevent overfitting. Which techniques should they apply? (Select TWO)

A) Increase the model complexity
B) Apply dropout regularization
C) Use early stopping based on validation loss
D) Remove the validation set
E) Train for more epochs

**Q40.** An ML engineer needs to build a recommendation system for an e-commerce platform. They have user-item interaction data. Which approach should they consider?

A) Use Amazon Personalize
B) Train a linear regression model
C) Use Amazon Comprehend
D) Use Amazon Forecast

**Q41.** A team is fine-tuning a large language model on Amazon Bedrock for their specific domain. They want to customize the model's responses using their proprietary data. Which Bedrock feature should they use?

A) Bedrock Knowledge Bases
B) Bedrock Model Fine-tuning
C) Bedrock Agents
D) Bedrock Guardrails

**Q42.** An ML engineer is evaluating a multi-class classification model. They need a single metric that balances precision and recall across all classes. Which metric should they use?

A) Accuracy
B) Macro-averaged F1 score
C) ROC-AUC
D) Mean Squared Error

**Q43.** A company needs to forecast product demand for the next 30 days using historical sales data. Which AWS service is purpose-built for time-series forecasting?

A) Amazon SageMaker with DeepAR
B) Amazon Forecast
C) Amazon Lookout for Metrics
D) Amazon QuickSight ML Insights

**Q44.** An ML engineer is training a model on SageMaker and wants to detect issues like vanishing gradients, exploding tensors, or poor weight initialization during training. Which tool should they use?

A) Amazon CloudWatch Logs
B) SageMaker Debugger
C) SageMaker Clarify
D) SageMaker Model Monitor

**Q45.** A data scientist needs to quickly build and compare multiple ML models on a tabular dataset without writing extensive code. Which SageMaker feature automates the end-to-end ML workflow?

A) SageMaker Studio
B) SageMaker Autopilot
C) SageMaker Canvas
D) SageMaker JumpStart

**Q46.** An ML engineer is training an XGBoost model and wants to reduce overfitting. Which hyperparameter should they adjust?

A) Increase max_depth
B) Decrease min_child_weight
C) Increase the regularization parameter (lambda)
D) Increase the number of boosting rounds without early stopping

**Q47.** A team needs to detect visual defects in products on a manufacturing line using images. They want a managed service that requires minimal ML expertise and minimal training data. Which AWS service should they use?

A) Amazon Rekognition Custom Labels
B) Amazon SageMaker with a custom YOLO model
C) Amazon Textract
D) Amazon Lookout for Vision

**Q48.** An ML engineer is using SageMaker Clarify to analyze a trained model. They want to understand which features contribute most to individual predictions. Which explainability method does Clarify use?

A) LIME
B) SHAP (SHapley Additive exPlanations)
C) Grad-CAM
D) Permutation importance

**Q49.** A data scientist is training a regression model and needs to evaluate its performance. The target variable has a wide range of values. Which metric is MOST appropriate?

A) Accuracy
B) F1 Score
C) Root Mean Squared Error (RMSE)
D) AUC-ROC

**Q50.** An ML engineer needs to train a model on a dataset that is too large to fit in memory on a single instance. Which SageMaker feature allows distributed training across multiple instances?

A) SageMaker Inference Pipelines
B) SageMaker managed distributed training
C) SageMaker Batch Transform
D) SageMaker Multi-Model Endpoints

**Q51.** A company wants to generate realistic synthetic tabular data to augment their small training dataset. Which approach should they consider?

A) Use Amazon SageMaker Data Wrangler to duplicate existing rows
B) Use a generative adversarial network (GAN) or variational autoencoder
C) Use Amazon Mechanical Turk to create data manually
D) Use Amazon Comprehend to generate text data

**Q52.** An ML engineer is training a model and wants to manage different versions of the model, track which dataset was used, and register approved models for deployment. Which SageMaker feature should they use?

A) SageMaker Experiments
B) SageMaker Model Registry
C) SageMaker Debugger
D) SageMaker Feature Store

**Q53.** A team is building a sentiment analysis solution. They need to classify text as positive, negative, or neutral. They want to avoid training a custom model. Which AWS service provides this out of the box?

A) Amazon Translate
B) Amazon Comprehend
C) Amazon Lex
D) Amazon Polly

**Q54.** An ML engineer is training a deep learning model on SageMaker using GPU instances. Training is taking too long. Which approach can reduce training time? (Select TWO)

A) Use SageMaker distributed data parallel training
B) Switch to a CPU instance type
C) Use mixed-precision training (FP16)
D) Reduce the number of training instances to 1
E) Decrease the learning rate to a very small value

---

## Domain 3: Deployment and Orchestration of ML Workflows (Questions 55–76)

**Q55.** An ML engineer needs to deploy a model that serves real-time predictions with low latency for a web application. Which SageMaker deployment option should they use?

A) SageMaker Batch Transform
B) SageMaker Real-time Inference Endpoint
C) SageMaker Asynchronous Inference
D) SageMaker Serverless Inference

**Q56.** A company needs to run inference on 10 million records stored in S3 once per day. Latency is not a concern. Which SageMaker deployment option is MOST cost-effective?

A) SageMaker Real-time Inference Endpoint
B) SageMaker Batch Transform
C) SageMaker Serverless Inference
D) SageMaker Multi-Model Endpoint

**Q57.** An ML engineer wants to deploy a new model version to production but needs to gradually shift traffic from the old model to the new one. Which deployment strategy should they use?

A) Blue/green deployment with all-at-once traffic shifting
B) Canary deployment with SageMaker endpoint traffic shifting
C) Rolling deployment
D) Recreate deployment

**Q58.** A team needs to deploy multiple ML models on a single SageMaker endpoint to reduce costs. Each model serves a different customer segment. Which feature should they use?

A) SageMaker Inference Pipeline
B) SageMaker Multi-Model Endpoint
C) SageMaker Serverless Inference
D) SageMaker Batch Transform

**Q59.** An ML engineer needs to orchestrate an end-to-end ML workflow that includes data processing, model training, evaluation, and conditional deployment. Which AWS service is purpose-built for this?

A) AWS Step Functions
B) Amazon SageMaker Pipelines
C) AWS CodePipeline
D) Amazon MWAA (Managed Workflows for Apache Airflow)

**Q60.** A company wants to deploy a trained model to edge devices with limited compute resources. The model needs to be optimized for the target hardware. Which SageMaker feature should they use?

A) SageMaker Neo
B) SageMaker Debugger
C) SageMaker Clarify
D) SageMaker Ground Truth

**Q61.** An ML engineer needs to define their SageMaker infrastructure (endpoints, training jobs, processing jobs) as code for repeatable deployments. Which approach should they use?

A) Manually create resources through the SageMaker console
B) Use AWS CloudFormation or AWS CDK with SageMaker constructs
C) Use shell scripts with AWS CLI commands
D) Use SageMaker Studio notebooks

**Q62.** A team has deployed a SageMaker real-time endpoint. Traffic varies significantly — high during business hours and near zero at night. They want to minimize costs while maintaining availability. What should they configure?

A) Use a fixed instance count sized for peak traffic
B) Configure auto-scaling policies based on InvocationsPerInstance
C) Use SageMaker Batch Transform during off-hours
D) Manually scale the endpoint each day

**Q63.** An ML engineer needs to chain multiple containers in sequence for inference — first a data preprocessing container, then a model inference container, then a post-processing container. Which SageMaker feature supports this?

A) SageMaker Multi-Model Endpoint
B) SageMaker Inference Pipeline
C) SageMaker Batch Transform
D) SageMaker Processing Jobs

**Q64.** A team wants to automate model retraining whenever new data arrives in S3. The pipeline should trigger automatically, retrain the model, evaluate it, and deploy if the metrics meet a threshold. Which combination of services should they use?

A) S3 event notification → Lambda → SageMaker Pipelines
B) Amazon CloudWatch scheduled rule → EC2 instance → manual training
C) AWS Glue trigger → SageMaker notebook
D) Amazon SNS → SageMaker Batch Transform

**Q65.** An ML engineer needs to deploy a model for inference but the payload size can be up to 1 GB and processing can take up to 15 minutes per request. Which SageMaker inference option should they use?

A) SageMaker Real-time Inference
B) SageMaker Serverless Inference
C) SageMaker Asynchronous Inference
D) SageMaker Batch Transform

**Q66.** A company wants to use CI/CD practices for their ML models. They need to automatically build, test, and deploy model artifacts. Which AWS services should they combine? (Select TWO)

A) AWS CodePipeline
B) AWS CodeBuild
C) Amazon Kinesis
D) Amazon DynamoDB
E) Amazon CloudFront

**Q67.** An ML engineer is deploying a PyTorch model on SageMaker. They need to customize the inference logic (preprocessing input, running the model, and formatting output). What should they provide?

A) A SageMaker built-in algorithm configuration
B) A custom inference script with model_fn, input_fn, predict_fn, and output_fn
C) A CloudFormation template
D) A Lambda function

**Q68.** A team needs to deploy a model that handles sporadic, unpredictable traffic with long idle periods. They want to pay only when the endpoint is processing requests. Which SageMaker option is MOST appropriate?

A) SageMaker Real-time Inference with auto-scaling
B) SageMaker Serverless Inference
C) SageMaker Multi-Model Endpoint
D) SageMaker Batch Transform

**Q69.** An ML engineer needs to run a SageMaker training job in a private VPC with no internet access. The job needs to access training data in S3. How should they configure network access?

A) Attach an internet gateway to the VPC
B) Configure a VPC endpoint for S3
C) Use a NAT gateway
D) Disable network isolation

**Q70.** A company is using SageMaker Pipelines and wants to add a manual approval step before deploying a model to production. Which pipeline step type should they use?

A) ProcessingStep
B) TrainingStep
C) ConditionStep
D) CallbackStep or a Lambda step that waits for approval

**Q71.** An ML engineer needs to deploy a model to both AWS and an on-premises Kubernetes cluster. Which approach provides the MOST portability?

A) Use SageMaker Real-time Endpoints for both
B) Package the model in a Docker container and deploy to Amazon EKS and on-premises Kubernetes
C) Use SageMaker Neo for both environments
D) Use AWS Lambda for both environments

**Q72.** A team is using SageMaker Pipelines and wants to conditionally deploy a model only if the evaluation metric (F1 score) exceeds 0.85. Which pipeline step should they use?

A) TrainingStep
B) ProcessingStep
C) ConditionStep
D) TransformStep

**Q73.** An ML engineer needs to serve a model that requires a GPU for inference. The model processes video frames in real-time. Which SageMaker instance type should they choose?

A) ml.m5.xlarge
B) ml.c5.xlarge
C) ml.g4dn.xlarge
D) ml.t3.medium

**Q74.** A company wants to A/B test two model versions in production by sending 90% of traffic to the current model and 10% to the new model. How should they configure this on SageMaker?

A) Deploy two separate endpoints and use Route 53 weighted routing
B) Use a SageMaker endpoint with production variants and set traffic weights
C) Use a Lambda function to randomly route requests
D) Deploy both models on the same instance

**Q75.** An ML engineer needs to reduce the size of a trained deep learning model for faster inference without significant accuracy loss. Which technique should they apply?

A) Increase the model's layer count
B) Apply model quantization (e.g., FP32 to INT8)
C) Retrain with more data
D) Use a larger instance type

**Q76.** A team is building an ML pipeline with SageMaker Pipelines. They want to cache the results of expensive processing steps so they are not re-executed if the inputs haven't changed. Which feature supports this?

A) SageMaker Experiments
B) SageMaker Pipeline step caching
C) Amazon ElastiCache
D) S3 versioning

---

## Domain 4: ML Solution Monitoring, Maintenance, and Security (Questions 77–100)

**Q77.** An ML engineer has a model in production that is showing degraded prediction accuracy over time. The input data distribution has shifted from what the model was trained on. What is this phenomenon called?

A) Concept drift
B) Data drift
C) Model decay
D) Feature leakage

**Q78.** A team needs to continuously monitor a deployed SageMaker model for data quality issues, model quality degradation, and bias drift. Which service should they use?

A) Amazon CloudWatch
B) SageMaker Model Monitor
C) SageMaker Debugger
D) AWS Config

**Q79.** An ML engineer needs to set up alerts when the distribution of incoming inference data deviates significantly from the training data baseline. Which SageMaker Model Monitor type should they configure?

A) Model Quality Monitor
B) Data Quality Monitor
C) Bias Drift Monitor
D) Feature Attribution Drift Monitor

**Q80.** A company needs to ensure that their ML model does not discriminate based on protected attributes (age, gender, race) in production. Which tool should they use for ongoing bias monitoring?

A) SageMaker Clarify bias monitoring
B) Amazon Macie
C) AWS Trusted Advisor
D) Amazon Inspector

**Q81.** An ML engineer needs to troubleshoot high latency on a SageMaker real-time endpoint. Which AWS service should they use to view invocation metrics like ModelLatency and OverheadLatency?

A) AWS X-Ray
B) Amazon CloudWatch
C) AWS CloudTrail
D) SageMaker Debugger

**Q82.** A team wants to track how feature importance changes over time for a deployed model. Which SageMaker Model Monitor type provides this capability?

A) Data Quality Monitor
B) Model Quality Monitor
C) Bias Drift Monitor
D) Feature Attribution Drift Monitor

**Q83.** An ML engineer needs to ensure that SageMaker training jobs and endpoints can only be accessed by authorized IAM roles. Which AWS service controls this access?

A) Amazon Cognito
B) AWS IAM policies and roles
C) Amazon GuardDuty
D) AWS Shield

**Q84.** A company requires that all data used for ML training and inference is encrypted at rest and in transit. Which combination of AWS features should they use? (Select TWO)

A) AWS KMS for encryption at rest
B) TLS/SSL for encryption in transit
C) Amazon Macie for encryption
D) AWS WAF for encryption
E) Amazon Inspector for encryption

**Q85.** An ML engineer needs to restrict a SageMaker notebook instance so it cannot access the internet. How should they configure this?

A) Remove the IAM role from the notebook instance
B) Launch the notebook instance in a VPC with no internet gateway or NAT gateway
C) Disable the root access on the notebook instance
D) Use a smaller instance type

**Q86.** A team needs to audit all API calls made to SageMaker services for compliance purposes. Which AWS service provides this audit trail?

A) Amazon CloudWatch Logs
B) AWS CloudTrail
C) AWS Config
D) Amazon GuardDuty

**Q87.** An ML engineer notices that a production model's accuracy has dropped below an acceptable threshold. They need to automatically trigger a retraining pipeline. Which approach should they use?

A) Manually check metrics daily and retrain when needed
B) Configure a CloudWatch alarm on model quality metrics that triggers a Lambda function to start a SageMaker Pipeline
C) Increase the endpoint instance size
D) Roll back to the previous model version permanently

**Q88.** A company needs to manage IAM roles for SageMaker users with different permission levels — data scientists should only train models, while ML engineers can also deploy. Which SageMaker feature simplifies role management?

A) SageMaker Studio user profiles
B) SageMaker Role Manager
C) AWS Organizations
D) Amazon Cognito user pools

**Q89.** An ML engineer needs to ensure that a SageMaker endpoint runs within a private VPC and does not have any public internet access. Which configuration is required?

A) Deploy the endpoint with VPC configuration specifying private subnets and security groups
B) Use a public subnet with a restrictive security group
C) Enable network isolation on the endpoint only
D) Use SageMaker Serverless Inference

**Q90.** A team is using SageMaker and needs to ensure that training data in S3 is only accessible from within their VPC. Which AWS feature should they configure?

A) S3 bucket policy with VPC endpoint condition
B) S3 Transfer Acceleration
C) S3 Cross-Region Replication
D) S3 Object Lock

**Q91.** An ML engineer wants to optimize the cost of running SageMaker training jobs that are fault-tolerant and can be interrupted. Which pricing option should they use?

A) On-Demand instances
B) SageMaker Managed Spot Training
C) Reserved Instances
D) Dedicated Hosts

**Q92.** A company needs to monitor the cost of their ML workloads across multiple SageMaker training jobs, endpoints, and processing jobs. Which AWS tool should they use?

A) SageMaker Debugger
B) AWS Cost Explorer with resource-level tags
C) Amazon CloudWatch
D) SageMaker Experiments

**Q93.** An ML engineer needs to ensure that model artifacts stored in S3 are versioned and that previous versions can be restored if a new model performs poorly. Which S3 feature should they enable?

A) S3 Lifecycle Policies
B) S3 Versioning
C) S3 Object Lock
D) S3 Intelligent-Tiering

**Q94.** A team is deploying a model that processes sensitive healthcare data. They need to ensure the model endpoint is compliant with HIPAA. Which steps should they take? (Select TWO)

A) Use a HIPAA-eligible SageMaker instance type
B) Ensure a BAA (Business Associate Agreement) is in place with AWS
C) Store data in a public S3 bucket with encryption
D) Use Amazon Lightsail for deployment
E) Disable CloudTrail logging to reduce data exposure

**Q95.** An ML engineer needs to detect when the statistical properties of incoming inference requests change significantly compared to the training data. Which metric type should they monitor?

A) Infrastructure utilization metrics
B) Data distribution metrics (e.g., KL divergence, KS test)
C) API error rates
D) Training loss curves

**Q96.** A company wants to implement model governance — tracking which models are approved for production, who approved them, and when. Which SageMaker feature supports this?

A) SageMaker Experiments
B) SageMaker Model Registry with approval status
C) SageMaker Feature Store
D) SageMaker Debugger

**Q97.** An ML engineer needs to log all inference requests and responses for a SageMaker endpoint for debugging and compliance. Which SageMaker feature should they enable?

A) SageMaker Model Monitor
B) SageMaker Data Capture
C) AWS CloudTrail
D) Amazon CloudWatch Logs

**Q98.** A team is running multiple SageMaker endpoints and wants to right-size the instances to reduce costs without impacting latency. Which approach should they take?

A) Use the largest available instance type for all endpoints
B) Analyze CloudWatch metrics (CPUUtilization, MemoryUtilization, ModelLatency) and adjust instance types accordingly
C) Switch all endpoints to Serverless Inference
D) Use Spot Instances for real-time endpoints

**Q99.** An ML engineer needs to ensure that a SageMaker training job cannot access any network resources — it should only use the data provided in the input channels. Which SageMaker feature enforces this?

A) VPC configuration
B) Network isolation (internet-free mode)
C) IAM role restrictions
D) Security group rules

**Q100.** A company is building an ML platform and needs to implement least-privilege access. Data scientists should be able to run training jobs but not create or delete endpoints. How should they implement this?

A) Give all users the SageMakerFullAccess managed policy
B) Create custom IAM policies that allow sagemaker:CreateTrainingJob but deny sagemaker:CreateEndpoint and sagemaker:DeleteEndpoint
C) Use a single shared IAM role for all users
D) Restrict access at the VPC level only
