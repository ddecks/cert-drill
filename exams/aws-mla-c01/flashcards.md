## Amazon SageMaker — Core

Q: SageMaker Studio
A: Integrated development environment (IDE) for ML. Provides notebooks, experiment tracking, debugging, model registry, pipelines, and feature store in a single web interface.

Q: SageMaker notebook instances
A: Managed Jupyter notebook instances for data exploration and prototyping. Can be launched in a VPC with or without internet access. Support lifecycle configurations for automated setup.

Q: SageMaker Training Jobs
A: Managed infrastructure for model training. Specify the algorithm/container, instance type, input data channels, and hyperparameters. SageMaker provisions instances, runs training, and stores artifacts in S3.

Q: SageMaker Processing Jobs
A: Run data preprocessing, postprocessing, and evaluation workloads. Support Spark, scikit-learn, and custom containers. Used for feature engineering, data validation, and model evaluation.

Q: SageMaker Managed Spot Training
A: Uses EC2 Spot Instances for training at up to 90% discount. SageMaker handles interruptions with automatic checkpointing. Ideal for fault-tolerant, long-running training jobs.

Q: SageMaker Autopilot
A: AutoML service that automatically explores algorithms, tunes hyperparameters, and generates candidate models for tabular data. Provides full visibility into generated code and model artifacts.

Q: SageMaker Canvas
A: No-code ML tool for business analysts. Provides a visual interface to build, train, and generate predictions without writing code. Supports tabular, time-series, and image data.

Q: SageMaker JumpStart
A: Provides pre-trained models, solution templates, and example notebooks. Supports one-click deployment of foundation models and fine-tuning on custom datasets.

## Amazon SageMaker — Built-in Algorithms

Q: XGBoost (SageMaker built-in)
A: Gradient boosted trees algorithm for classification and regression. Supports CSV and libsvm input. Key hyperparameters: max_depth, eta (learning rate), num_round, lambda (L2 regularization), alpha (L1 regularization).

Q: BlazingText
A: Word2Vec (unsupervised) and text classification (supervised). Supervised mode is based on fastText — efficient multi-class text classification. Supports single or multi-core CPU and single GPU.

Q: Linear Learner
A: Linear models for classification and regression. Supports automatic data normalization, multiple optimization algorithms, and built-in hyperparameter tuning. Input: RecordIO or CSV.

Q: K-Means (SageMaker)
A: Unsupervised clustering algorithm. Groups data into k clusters based on distance. Uses a modified k-means++ initialization. Input: RecordIO or CSV. GPU supported for large datasets.

Q: Random Cut Forest (RCF)
A: Unsupervised anomaly detection algorithm. Assigns anomaly scores to data points. Works on tabular data and time-series. Does not require labeled anomaly data.

Q: DeepAR
A: Recurrent neural network for time-series forecasting. Trains a single model across multiple related time series. Produces probabilistic forecasts (quantiles). Input: JSON Lines format.

Q: Image Classification (SageMaker)
A: CNN-based algorithm for image classification. Supports transfer learning with pre-trained ResNet models. Input: RecordIO or image files with augmented manifest. GPU recommended.

Q: Object Detection (SageMaker)
A: Detects and classifies objects in images with bounding boxes. Based on SSD (Single Shot MultiBox Detector) or VGG. Supports transfer learning. Input: RecordIO or augmented manifest.

Q: Semantic Segmentation (SageMaker)
A: Pixel-level image classification. Labels every pixel in an image with a class. Based on FCN, PSP, or DeepLab architectures. Requires GPU instances for training.

Q: Seq2Seq (SageMaker)
A: Sequence-to-sequence model for tasks like machine translation and text summarization. Encoder-decoder architecture with attention mechanism. Input: RecordIO protobuf with integer tokens.

## Amazon SageMaker — MLOps

Q: SageMaker Pipelines
A: Purpose-built CI/CD service for ML workflows. Supports steps for processing, training, evaluation, condition checks, registration, and deployment. Integrates with Model Registry and Experiments.

Q: SageMaker Pipeline step caching
A: Caches outputs of completed pipeline steps. If inputs haven't changed, the cached result is reused, avoiding redundant computation. Enabled per step with CacheConfig.

Q: SageMaker Model Registry
A: Central repository for managing model versions. Tracks model metadata, approval status (Approved/Rejected/PendingManualApproval), lineage, and deployment history. Supports model groups.

Q: SageMaker Experiments
A: Organizes, tracks, and compares ML experiments. Logs parameters, metrics, and artifacts for each trial. Enables systematic comparison across training runs.

Q: SageMaker ML Lineage Tracking
A: Automatically tracks relationships between datasets, processing jobs, training jobs, model artifacts, and endpoints. Provides end-to-end data and model lineage for governance.

Q: SageMaker Feature Store
A: Centralized repository for ML features. Online store provides low-latency GetRecord API for inference. Offline store is S3-backed for batch training. Ensures feature consistency.

Q: SageMaker Model Monitor
A: Continuously monitors deployed models for data quality, model quality, bias drift, and feature attribution drift. Runs on a schedule, compares against baselines, and sends CloudWatch alerts.

Q: SageMaker Data Capture
A: Logs inference requests and responses (input/output data) to S3 from real-time endpoints. Used for monitoring, debugging, auditing, and generating retraining datasets.

## Amazon SageMaker — Deployment

Q: SageMaker Real-time Inference
A: Persistent, always-on endpoints for low-latency predictions. Support auto-scaling, A/B testing with production variants, and traffic shifting for canary deployments.

Q: SageMaker Batch Transform
A: Offline inference on large datasets stored in S3. Provisions instances, runs inference, stores results in S3, and terminates. Cost-effective for periodic bulk predictions.

Q: SageMaker Asynchronous Inference
A: Handles large payloads (up to 1 GB) and long processing times (up to 15 minutes). Queues requests internally, processes them, and stores results in S3. Supports scale-to-zero.

Q: SageMaker Serverless Inference
A: Automatically scales to zero when idle and provisions compute on demand. Pay only for processing time. Ideal for sporadic, unpredictable traffic with cold start tolerance.

Q: SageMaker Multi-Model Endpoints
A: Host multiple models on a single endpoint. Dynamically loads/unloads models based on traffic. Reduces costs when serving many models with varying traffic patterns.

Q: SageMaker Inference Pipelines
A: Chain up to 5 containers in sequence on a single endpoint. Each container handles a step (preprocessing, inference, post-processing). Supports both real-time and batch.

Q: SageMaker Neo
A: Compiles trained models for specific target hardware (edge devices, mobile, cloud instances). Optimizes for faster inference with lower resource consumption. Supports TensorFlow, PyTorch, MXNet, XGBoost.

Q: SageMaker production variants
A: Deploy multiple model versions on a single endpoint with configurable traffic weights. Used for A/B testing (e.g., 90/10 split) and canary deployments.

Q: SageMaker Role Manager
A: Simplified interface for creating IAM roles with predefined ML activity-based permissions. Provides role templates for data scientists, MLOps engineers, and other ML personas.

## Amazon SageMaker — Responsible AI

Q: SageMaker Clarify — pre-training bias
A: Detects bias in training data before model training. Metrics include Class Imbalance (CI), Difference in Proportions of Labels (DPL), and Kullback-Leibler Divergence (KL).

Q: SageMaker Clarify — post-training bias
A: Detects bias in model predictions after training. Metrics include Disparate Impact (DI), Difference in Positive Proportions in Predicted Labels (DPPL), and Conditional Demographic Disparity (CDD).

Q: SageMaker Clarify — explainability
A: Uses SHAP (SHapley Additive exPlanations) to explain individual predictions. Shows feature-level contributions to model output. Supports both tabular and NLP models.

Q: SageMaker Debugger
A: Captures tensors during training to detect issues like vanishing gradients, exploding tensors, overfitting, and poor weight initialization. Provides built-in rules and custom rule support.

## Amazon SageMaker — Data Preparation

Q: SageMaker Data Wrangler
A: Visual data preparation tool within SageMaker Studio. Provides 300+ built-in transformations, data profiling, and visualization. Flows can be exported as Pipeline steps or Glue jobs.

Q: SageMaker Ground Truth
A: Managed data labeling service. Supports human annotators (Mechanical Turk, private teams, vendors) and active learning to reduce labeling costs. Handles image, text, video, and 3D point cloud data.

Q: SageMaker Ground Truth Plus
A: Fully managed data labeling service where AWS provides the labeling workforce and project management. No need to build labeling applications or manage annotators.

## AWS AI Services

Q: Amazon Bedrock
A: Fully managed service for building generative AI applications using foundation models (FMs). Supports model fine-tuning, RAG with Knowledge Bases, Agents for task automation, and Guardrails for safety.

Q: Amazon Bedrock Knowledge Bases
A: Enables RAG by connecting foundation models to your data sources. Automatically chunks, embeds, and indexes documents. Retrieves relevant context for each query to ground model responses.

Q: Amazon Bedrock Guardrails
A: Configurable safeguards for generative AI applications. Filter harmful content, block denied topics, redact PII, and control hallucinations. Applied to any Bedrock model.

Q: Amazon Comprehend
A: NLP service for entity recognition, sentiment analysis, key phrase extraction, language detection, and topic modeling. Pre-trained models — no ML expertise required. Supports custom entity and classification models.

Q: Amazon Rekognition
A: Computer vision service for image and video analysis. Detects objects, scenes, faces, text, and inappropriate content. Custom Labels feature allows training custom object detection models.

Q: Amazon Rekognition Custom Labels
A: Train custom object detection and image classification models with as few as 30 images. Handles the ML pipeline automatically. Useful for domain-specific visual inspection.

Q: Amazon Textract
A: Extracts text, handwriting, tables, and forms from scanned documents. Goes beyond OCR by understanding document structure. Supports queries for targeted data extraction.

Q: Amazon Translate
A: Neural machine translation service. Supports 75+ languages. Real-time and batch translation. Custom terminology for domain-specific translations.

Q: Amazon Polly
A: Text-to-speech service. Converts text to lifelike speech in 60+ voices and 30+ languages. Supports SSML for fine-grained speech control. Neural TTS for natural-sounding voices.

Q: Amazon Transcribe
A: Automatic speech recognition (ASR). Converts speech to text. Supports real-time and batch transcription, custom vocabularies, speaker identification, and content redaction.

Q: Amazon Lex
A: Conversational AI service for building chatbots and voice assistants. Uses ASR and NLU. Powers Alexa. Integrates with Lambda for fulfillment logic.

Q: Amazon Kendra
A: Intelligent enterprise search service powered by ML. Understands natural language queries and returns precise answers from documents. Supports connectors for S3, SharePoint, databases, and more.

Q: Amazon Personalize
A: Fully managed recommendation service. Handles user-item interaction data, trains models, and provides real-time personalized recommendations. No ML expertise required.

Q: Amazon Forecast
A: Fully managed time-series forecasting service. Automatically selects algorithms, handles preprocessing, and produces probabilistic forecasts. Supports related time series and item metadata.

Q: Amazon Lookout for Vision
A: Automated visual inspection service for manufacturing. Detects defects in images with as few as 30 normal and 20 anomalous images. No ML expertise required.

Q: Amazon Lookout for Metrics
A: Automatically detects anomalies in business metrics and identifies root causes. Connects to data sources like S3, RDS, Redshift, and CloudWatch.

Q: Amazon Augmented AI (A2I)
A: Human review workflows for ML predictions. Routes low-confidence predictions to human reviewers. Integrates with Textract, Rekognition, and custom ML models.

## AWS Data Services for ML

Q: AWS Glue
A: Serverless ETL service. Crawls data sources, infers schemas, and runs Spark-based ETL jobs. Integrates with S3, RDS, Redshift, and DynamoDB. Supports Python and Scala.

Q: AWS Glue Data Catalog
A: Centralized metadata repository for data lakes. Stores table definitions, schemas, and partition information. Used by Athena, EMR, Redshift Spectrum, and Glue ETL.

Q: AWS Glue DataBrew
A: Visual data preparation tool with 250+ built-in transformations. Profile data, detect anomalies, and apply transformations without writing code. Outputs to S3 in various formats.

Q: AWS Lake Formation
A: Centralized data lake governance. Provides fine-grained access control, data encryption, audit logging, and cross-account sharing. Builds on top of the Glue Data Catalog.

Q: Amazon Kinesis Data Streams
A: Real-time data streaming service. Producers write records, consumers read them. Data retained 24 hours (up to 365 days). Supports multiple consumers per shard. Requires custom consumer code.

Q: Amazon Kinesis Data Firehose
A: Fully managed delivery stream. Ingests data and delivers to S3, Redshift, OpenSearch, or Splunk. Near real-time (60-second minimum buffer). Supports Lambda transformations.

Q: Amazon Managed Service for Apache Flink
A: Managed stream processing with Apache Flink. Supports complex event processing, windowing, aggregation, and joins on streaming data. Integrates with Kinesis and Kafka.

Q: Amazon Athena
A: Serverless interactive query service. Runs SQL queries on data in S3. Supports Parquet, ORC, JSON, CSV. CTAS can convert formats. Federated queries join across RDS, DynamoDB, and other sources.

Q: Amazon EMR
A: Managed Hadoop/Spark cluster service. Runs distributed data processing at scale. Supports Spark, Hive, Presto, HBase. Used for large-scale feature engineering and data transformation.

Q: Amazon Macie
A: ML-powered service that discovers and protects sensitive data (PII, PHI) in S3. Automatically classifies data and generates findings for compliance.

## AWS Infrastructure for ML

Q: SageMaker VPC configuration
A: Training jobs and endpoints can run in a VPC with private subnets and security groups. Use VPC endpoints for S3 and other AWS services to avoid internet access.

Q: SageMaker network isolation
A: EnableNetworkIsolation=true prevents training/inference containers from making outbound network calls. Containers can only access data through SageMaker input channels.

Q: AWS CloudFormation for ML
A: Infrastructure as code for SageMaker resources. Define endpoints, training jobs, pipelines, and model registry entries in templates for repeatable, version-controlled deployments.

Q: AWS CDK for ML
A: Higher-level infrastructure as code using programming languages (Python, TypeScript). Provides SageMaker-specific constructs for endpoints, pipelines, and other ML resources.

Q: Amazon CloudWatch for ML
A: Monitors SageMaker metrics: ModelLatency, OverheadLatency, Invocations, InvocationsPerInstance, CPUUtilization, MemoryUtilization. Used for auto-scaling policies and alerting.

Q: AWS CloudTrail for ML
A: Logs all API calls to SageMaker and other AWS services. Provides audit trail of who created/modified/deleted ML resources. Essential for compliance and security.

Q: AWS Cost Explorer for ML
A: Analyze and visualize ML workload costs. Use resource-level tags on training jobs, endpoints, and processing jobs for granular cost attribution and optimization.
