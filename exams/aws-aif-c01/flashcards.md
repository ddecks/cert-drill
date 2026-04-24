## Amazon Bedrock

Q: Amazon Bedrock
A: Fully managed service for building generative AI applications. Provides access to foundation models from Amazon (Titan), Anthropic (Claude), Meta (Llama), Stability AI, and others through a single API. No infrastructure management required.

Q: Amazon Bedrock Knowledge Bases
A: Implements RAG (retrieval-augmented generation). Chunks your documents, creates vector embeddings, stores them in a vector database, and retrieves relevant context at query time. Grounds model responses in your data.

Q: Amazon Bedrock Agents
A: AI agents that can decompose complex tasks, maintain conversation context, call external APIs and Lambda functions, and take actions on behalf of users. Example: an assistant that looks up orders and processes returns.

Q: Amazon Bedrock Guardrails
A: Configurable safety controls for generative AI. Content filters (hate, violence, sexual), denied topic filters, PII redaction, word filters, and contextual grounding checks. Applied to any Bedrock model.

Q: Amazon Bedrock Model Fine-tuning
A: Customize a foundation model using your proprietary data. Adapts the model's responses to your domain without training from scratch. Requires labeled training data in the model's expected format.

Q: Amazon Bedrock Model Evaluation
A: Compare and benchmark different foundation models for your use case. Evaluate quality, latency, and cost across models to select the best fit.

Q: Amazon Titan (foundation models)
A: Amazon's own family of foundation models available through Bedrock. Includes Titan Text (language), Titan Embeddings (vector representations), and Titan Image Generator (text-to-image).

## Amazon SageMaker

Q: Amazon SageMaker
A: Fully managed ML platform for building, training, and deploying custom ML models. Provides notebooks, training infrastructure, model hosting, MLOps pipelines, and monitoring.

Q: SageMaker Canvas
A: No-code ML tool for business analysts. Visual interface to build, train, and generate predictions without writing code. Supports tabular, time-series, and image data.

Q: SageMaker JumpStart
A: Pre-trained models, solution templates, and example notebooks. One-click deployment of foundation models. Fine-tune models on custom datasets.

Q: SageMaker Autopilot
A: AutoML service that automatically explores algorithms, tunes hyperparameters, and generates candidate models for tabular data. Provides full visibility into generated code.

Q: SageMaker Clarify
A: Detects bias in training data (pre-training) and model predictions (post-training). Provides explainability using SHAP values. Monitors bias drift in production.

Q: SageMaker Ground Truth
A: Managed data labeling service. Human annotators (Mechanical Turk, private teams, vendors) label data for ML training. Active learning reduces labeling costs.

Q: Amazon Augmented AI (A2I)
A: Human review workflows for ML predictions. Routes low-confidence predictions to human reviewers. Integrates with Textract, Rekognition, and custom models.

## AWS AI Services — Language

Q: Amazon Comprehend
A: Pre-trained NLP service. Sentiment analysis (positive/negative/neutral/mixed), entity recognition (people, places, dates), key phrase extraction, language detection, and topic modeling. No ML expertise required.

Q: Amazon Translate
A: Neural machine translation for 75+ languages. Real-time and batch translation. Custom terminology for domain-specific terms. Integrates with other AWS services.

Q: Amazon Lex
A: Conversational AI for chatbots and voice assistants. Natural language understanding (NLU), intent recognition, slot filling, multi-turn dialog. Powers Alexa. Integrates with Lambda for fulfillment.

Q: Amazon Kendra
A: Intelligent enterprise search powered by ML. Understands natural language questions and returns precise answers from documents. Connectors for S3, SharePoint, databases, and more.

Q: Amazon Q Developer
A: AI-powered code assistant (formerly CodeWhisperer). Generates code, completes functions, explains code, finds bugs, and suggests fixes. Supports multiple languages and IDEs.

Q: Amazon Q Business
A: AI assistant for business users. Answers questions using company data, summarizes documents, and generates content. Connects to enterprise data sources.

## AWS AI Services — Speech

Q: Amazon Transcribe
A: Automatic speech recognition (ASR). Converts speech to text. Real-time and batch transcription. Custom vocabularies, speaker identification, content redaction, and subtitle generation.

Q: Amazon Polly
A: Text-to-speech (TTS) service. Converts text to natural-sounding speech in 60+ voices and 30+ languages. Neural TTS for the most natural output. Supports SSML for speech control.

## AWS AI Services — Vision

Q: Amazon Rekognition
A: Computer vision service. Detects objects, scenes, faces, text, and inappropriate content in images and videos. Face comparison for identity verification. Custom Labels for domain-specific detection.

Q: Amazon Textract
A: Document intelligence service. Extracts text, tables, forms, and key-value pairs from scanned documents. Goes beyond OCR by understanding document structure. Supports queries for targeted extraction.

Q: Amazon Lookout for Vision
A: Automated visual inspection for manufacturing. Detects defects in images with minimal training data (as few as 30 normal + 20 anomalous images). No ML expertise required.

## AWS AI Services — Other

Q: Amazon Personalize
A: Fully managed recommendation service. Real-time personalized product, content, and search recommendations from user interaction data. No ML expertise required.

Q: Amazon Forecast
A: Fully managed time-series forecasting. Automatically selects algorithms, handles preprocessing, and produces probabilistic forecasts. Used for demand planning, inventory, and resource allocation.

Q: Amazon Lookout for Metrics
A: Automatically detects anomalies in business metrics and identifies root causes. Connects to S3, RDS, Redshift, CloudWatch, and other data sources.

## AWS Security and Governance Services

Q: AWS IAM (Identity and Access Management)
A: Controls access to AWS resources. Policies define who can access which resources and what actions they can perform. Supports roles, users, groups, and permission boundaries.

Q: AWS KMS (Key Management Service)
A: Manages encryption keys for data at rest. Used by S3, EBS, SageMaker, and other services for server-side encryption. Supports customer-managed and AWS-managed keys.

Q: AWS CloudTrail
A: Logs all API calls to AWS services. Provides audit trail of who did what, when, and from where. Essential for compliance, security investigations, and governance.

Q: Amazon CloudWatch
A: Monitoring and observability service. Collects metrics, logs, and events. Set alarms and dashboards. Used to monitor AI/ML service performance and costs.

Q: Amazon Macie
A: ML-powered sensitive data discovery. Automatically finds and classifies PII, financial data, and credentials in S3. Generates findings for compliance and data protection.

Q: AWS Artifact
A: On-demand access to AWS compliance reports and agreements (SOC, ISO, PCI, HIPAA BAA). Central resource for compliance documentation.

Q: AWS Config
A: Tracks AWS resource configurations and changes over time. Evaluates compliance against rules. Provides configuration history for auditing.
