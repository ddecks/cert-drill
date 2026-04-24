# AWS AIF-C01 Practice Exam — 100 Questions

Weighted by exam domain:
- Domain 1: Fundamentals of AI and ML (Q1–Q20) — 20%
- Domain 2: Fundamentals of Generative AI (Q21–Q44) — 24%
- Domain 3: Applications of Foundation Models (Q45–Q72) — 28%
- Domain 4: Guidelines for Responsible AI (Q73–Q86) — 14%
- Domain 5: Security, Compliance, and Governance for AI Solutions (Q87–Q100) — 14%

Passing target: 700/1000 (~70%). Aim for 80%+ in practice.

---

## Domain 1: Fundamentals of AI and ML (Questions 1–20)

**Q1.** What is the primary difference between artificial intelligence (AI) and machine learning (ML)?

A) AI and ML are the same thing
B) ML is a subset of AI that learns from data without being explicitly programmed
C) AI is a subset of ML
D) ML requires human intervention for every decision

**Q2.** A company wants to predict whether a customer will churn based on historical data with labeled outcomes (churned/not churned). What type of ML problem is this?

A) Unsupervised learning
B) Reinforcement learning
C) Supervised learning — classification
D) Supervised learning — regression

**Q3.** Which type of machine learning is used to group customers into segments based on purchasing behavior without predefined labels?

A) Supervised learning — classification
B) Supervised learning — regression
C) Unsupervised learning — clustering
D) Reinforcement learning

**Q4.** A model trained on historical data performs well on the training set but poorly on new, unseen data. What is this problem called?

A) Underfitting
B) Overfitting
C) Data drift
D) Feature engineering

**Q5.** Which of the following is an example of a regression problem?

A) Classifying emails as spam or not spam
B) Predicting the price of a house based on its features
C) Grouping news articles by topic
D) Detecting fraudulent transactions

**Q6.** What is the correct order of the typical machine learning development lifecycle?

A) Deploy → Train → Collect data → Evaluate
B) Collect data → Train model → Deploy → Evaluate
C) Define problem → Collect and prepare data → Train model → Evaluate → Deploy → Monitor
D) Train model → Collect data → Define problem → Deploy

**Q7.** A company wants to build a system that learns to play a video game by receiving rewards for good actions and penalties for bad actions. Which type of ML is this?

A) Supervised learning
B) Unsupervised learning
C) Reinforcement learning
D) Transfer learning

**Q8.** What is a feature in the context of machine learning?

A) A prediction made by the model
B) An input variable used to make predictions
C) The output label in a training dataset
D) A type of neural network layer

**Q9.** Which AWS service provides a fully managed environment for building, training, and deploying ML models?

A) Amazon Bedrock
B) Amazon SageMaker
C) AWS Lambda
D) Amazon Comprehend

**Q10.** A data scientist splits their dataset into training, validation, and test sets. What is the purpose of the validation set?

A) To train the model
B) To tune hyperparameters and prevent overfitting during development
C) To provide the final unbiased evaluation of the model
D) To generate features

**Q11.** Which of the following BEST describes deep learning?

A) A type of ML that uses decision trees
B) A subset of ML that uses neural networks with multiple layers to learn complex patterns
C) A technique for cleaning data
D) A method for deploying models to production

**Q12.** A company has 10,000 product images and wants to automatically tag them by category. They have labeled examples for each category. Which ML approach should they use?

A) Unsupervised clustering
B) Supervised image classification
C) Reinforcement learning
D) Anomaly detection

**Q13.** What is the purpose of a confusion matrix in ML?

A) To visualize the architecture of a neural network
B) To show the counts of true positives, true negatives, false positives, and false negatives for a classification model
C) To display the training loss over time
D) To compare different feature engineering techniques

**Q14.** Which metric is MOST appropriate for evaluating a model that predicts continuous numerical values?

A) Accuracy
B) F1 Score
C) Root Mean Squared Error (RMSE)
D) AUC-ROC

**Q15.** A company wants to use AI to transcribe customer service phone calls into text. Which type of AI capability is this?

A) Natural language processing (NLP)
B) Computer vision
C) Recommendation systems
D) Anomaly detection

**Q16.** What is the difference between inference and training in ML?

A) Training and inference are the same process
B) Training is when the model learns from data; inference is when the trained model makes predictions on new data
C) Inference happens before training
D) Training uses production data; inference uses test data

**Q17.** Which of the following is an example of unsupervised learning?

A) Predicting house prices
B) Classifying images of cats and dogs
C) Detecting anomalies in network traffic without labeled examples
D) Training a chatbot with question-answer pairs

**Q18.** A company wants to add AI capabilities to their application quickly without building custom ML models. What type of AWS services should they consider?

A) Amazon EC2 instances with custom code
B) AWS AI services (pre-trained APIs like Comprehend, Rekognition, Translate)
C) Amazon RDS databases
D) AWS CloudFormation templates

**Q19.** What is a label in supervised machine learning?

A) A name given to a feature column
B) The known correct output or target value in the training data
C) A tag applied to an AWS resource
D) A description of the model architecture

**Q20.** Which of the following BEST describes the relationship between AI, ML, and deep learning?

A) They are three unrelated fields
B) AI is the broadest concept, ML is a subset of AI, and deep learning is a subset of ML
C) Deep learning is the broadest concept that contains ML and AI
D) ML is the broadest concept that contains AI and deep learning

---

## Domain 2: Fundamentals of Generative AI (Questions 21–44)

**Q21.** What is generative AI?

A) AI that only classifies existing data into categories
B) AI that can create new content such as text, images, code, and audio
C) AI that only works with structured databases
D) AI that requires manual rules for every output

**Q22.** What is a foundation model?

A) A small, task-specific model trained on a narrow dataset
B) A large model pre-trained on broad data that can be adapted to many downstream tasks
C) A model that can only perform one specific task
D) A model that does not require any training data

**Q23.** Which of the following is an example of a large language model (LLM)?

A) A linear regression model
B) A k-means clustering algorithm
C) Amazon Titan, Claude, or GPT
D) A random forest classifier

**Q24.** What is a prompt in the context of generative AI?

A) A training dataset used to build the model
B) The input text or instruction given to a generative AI model to produce a response
C) The model's internal weights
D) A deployment configuration file

**Q25.** What is prompt engineering?

A) Building the hardware infrastructure for AI models
B) The practice of crafting and refining input prompts to get better outputs from generative AI models
C) Training a model from scratch
D) Encrypting prompts for security

**Q26.** What is the difference between zero-shot, one-shot, and few-shot prompting?

A) They refer to different model sizes
B) Zero-shot provides no examples, one-shot provides one example, and few-shot provides several examples in the prompt to guide the model
C) They refer to the number of training epochs
D) They are different deployment strategies

**Q27.** What is a hallucination in the context of generative AI?

A) When the model takes too long to respond
B) When the model generates content that sounds plausible but is factually incorrect or fabricated
C) When the model refuses to answer a question
D) When the model produces an error message

**Q28.** What is retrieval-augmented generation (RAG)?

A) A technique to make models smaller
B) A method that retrieves relevant information from external sources and includes it in the prompt to ground the model's response in factual data
C) A way to train models faster
D) A deployment strategy for edge devices

**Q29.** What is fine-tuning in the context of foundation models?

A) Deploying a model to production
B) Further training a pre-trained model on a smaller, domain-specific dataset to adapt it for a particular task
C) Reducing the size of a model
D) Writing better prompts

**Q30.** What is the difference between a foundation model and a traditional ML model?

A) There is no difference
B) Foundation models are pre-trained on large, diverse datasets and can be adapted to many tasks; traditional ML models are typically trained for a single specific task
C) Traditional ML models are always larger than foundation models
D) Foundation models cannot be customized

**Q31.** Which AWS service provides access to foundation models from multiple providers (Amazon, Anthropic, Meta, etc.) through a single API?

A) Amazon SageMaker
B) Amazon Bedrock
C) Amazon Comprehend
D) AWS Lambda

**Q32.** What is a token in the context of large language models?

A) An authentication credential
B) A unit of text (word, subword, or character) that the model processes
C) A type of neural network layer
D) A billing unit for AWS services

**Q33.** What is the temperature parameter in generative AI?

A) The physical temperature of the GPU
B) A parameter that controls the randomness/creativity of the model's output — higher temperature means more random, lower means more deterministic
C) The speed of model inference
D) The size of the model

**Q34.** What is an embedding in the context of AI?

A) A way to deploy models on edge devices
B) A numerical vector representation of data (text, images) that captures semantic meaning, allowing similar items to be close together in vector space
C) A type of database
D) A security encryption method

**Q35.** What is the transformer architecture?

A) A type of database
B) The neural network architecture behind most modern LLMs, using self-attention mechanisms to process sequences in parallel
C) A data transformation tool
D) A deployment framework

**Q36.** What is the difference between text generation and text summarization in generative AI?

A) They are the same thing
B) Text generation creates new content from a prompt; text summarization condenses existing text into a shorter version while preserving key information
C) Text summarization generates longer text
D) Text generation only works with structured data

**Q37.** A company wants to use generative AI to create product descriptions from bullet-point features. Which generative AI task is this?

A) Image classification
B) Text generation / content creation
C) Anomaly detection
D) Clustering

**Q38.** What is the context window of a large language model?

A) The physical display showing the model's output
B) The maximum amount of text (measured in tokens) that the model can process in a single request, including both input and output
C) The training dataset size
D) The number of model parameters

**Q39.** What are model parameters in the context of foundation models?

A) Configuration settings for deployment
B) The internal weights learned during training that determine the model's behavior — larger models have more parameters (e.g., billions)
C) The input features of the training data
D) The hyperparameters set before training

**Q40.** What is the difference between a generative AI model and a discriminative AI model?

A) They are the same thing
B) Generative models create new data (text, images); discriminative models classify or distinguish between existing categories
C) Discriminative models are always larger
D) Generative models cannot be used for classification

**Q41.** What is chain-of-thought prompting?

A) Linking multiple models together
B) A prompting technique that encourages the model to show its reasoning step by step, improving accuracy on complex tasks
C) A method for training models
D) A way to chain API calls

**Q42.** What is a multi-modal foundation model?

A) A model that only processes text
B) A model that can process and generate multiple types of data (text, images, audio, video) within a single model
C) A model deployed across multiple regions
D) A model with multiple output endpoints

**Q43.** What is the primary advantage of using a managed AI service like Amazon Bedrock over training your own foundation model?

A) Managed services are always more accurate
B) You avoid the massive cost and complexity of training a foundation model from scratch, and can access multiple models through a single API
C) Managed services never have any limitations
D) Training your own model is always cheaper

**Q44.** What is the difference between pre-training and fine-tuning a foundation model?

A) They are the same process
B) Pre-training is the initial training on a large, general dataset to learn broad knowledge; fine-tuning is additional training on a smaller, task-specific dataset to specialize the model
C) Fine-tuning happens before pre-training
D) Pre-training uses labeled data; fine-tuning uses unlabeled data

---

## Domain 3: Applications of Foundation Models (Questions 45–72)

**Q45.** A company wants to build a customer service chatbot that answers questions using their internal knowledge base. Which approach should they use with Amazon Bedrock?

A) Train a foundation model from scratch
B) Use Amazon Bedrock Knowledge Bases with RAG to ground responses in company documents
C) Use Amazon Comprehend for chat
D) Build a rule-based chatbot with no AI

**Q46.** Which Amazon Bedrock feature allows you to create AI agents that can break down tasks, call APIs, and take actions on behalf of users?

A) Bedrock Knowledge Bases
B) Bedrock Guardrails
C) Bedrock Agents
D) Bedrock Model Fine-tuning

**Q47.** A company wants to automatically extract text, tables, and form data from scanned invoices. Which AWS AI service should they use?

A) Amazon Comprehend
B) Amazon Textract
C) Amazon Translate
D) Amazon Rekognition

**Q48.** A company needs to analyze customer reviews to determine whether the sentiment is positive, negative, or neutral. Which AWS service provides this capability without custom model training?

A) Amazon Lex
B) Amazon Comprehend
C) Amazon Polly
D) Amazon Kendra

**Q49.** A company wants to add real-time language translation to their global customer support application. Which AWS service should they use?

A) Amazon Comprehend
B) Amazon Translate
C) Amazon Transcribe
D) Amazon Polly

**Q50.** A company needs to convert text instructions into natural-sounding speech for an accessibility feature. Which AWS service should they use?

A) Amazon Transcribe
B) Amazon Lex
C) Amazon Polly
D) Amazon Translate

**Q51.** A company wants to build a voice-enabled virtual assistant that understands spoken commands and responds with actions. Which combination of AWS services would they need? (Select TWO)

A) Amazon Transcribe (speech-to-text)
B) Amazon Lex (conversational AI)
C) Amazon Macie
D) AWS CloudTrail
E) Amazon Redshift

**Q52.** A company wants to automatically detect inappropriate content in user-uploaded images on their platform. Which AWS service should they use?

A) Amazon Comprehend
B) Amazon Rekognition content moderation
C) Amazon Textract
D) Amazon Translate

**Q53.** A company wants to search their internal documents using natural language questions and get precise answers rather than keyword matches. Which AWS service is designed for this?

A) Amazon OpenSearch
B) Amazon Kendra
C) Amazon Athena
D) Amazon DynamoDB

**Q54.** Which Amazon Bedrock feature allows you to configure safety filters to block harmful content, denied topics, and PII in model inputs and outputs?

A) Bedrock Agents
B) Bedrock Knowledge Bases
C) Bedrock Guardrails
D) Bedrock Model Fine-tuning

**Q55.** A company wants to generate personalized product recommendations for their e-commerce website. Which AWS service is purpose-built for this?

A) Amazon Forecast
B) Amazon Personalize
C) Amazon Comprehend
D) Amazon Bedrock

**Q56.** A company needs to forecast inventory demand for the next quarter using historical sales data. Which AWS service should they use?

A) Amazon Personalize
B) Amazon Forecast
C) Amazon Lookout for Metrics
D) Amazon Bedrock

**Q57.** A developer wants to use a foundation model to generate code based on natural language descriptions. Which capability of generative AI does this represent?

A) Image classification
B) Code generation
C) Anomaly detection
D) Data labeling

**Q58.** A company wants to use Amazon Bedrock to summarize long legal documents into concise briefs. Which foundation model capability is being used?

A) Image generation
B) Text summarization
C) Speech recognition
D) Object detection

**Q59.** What is the primary benefit of using Amazon Bedrock Knowledge Bases compared to sending all context directly in the prompt?

A) It is always faster
B) It automatically retrieves only the most relevant information from large document collections, avoiding context window limitations
C) It eliminates the need for any prompting
D) It trains a new model for each query

**Q60.** A company wants to use generative AI to create marketing images from text descriptions. Which type of foundation model capability is this?

A) Text classification
B) Text-to-image generation
C) Speech synthesis
D) Anomaly detection

**Q61.** Which AWS service can automatically transcribe audio from meetings and phone calls into text?

A) Amazon Polly
B) Amazon Transcribe
C) Amazon Translate
D) Amazon Comprehend

**Q62.** A company wants to detect faces in images and compare them against a database for identity verification. Which AWS service should they use?

A) Amazon Textract
B) Amazon Comprehend
C) Amazon Rekognition
D) Amazon Kendra

**Q63.** What is the primary purpose of Amazon Augmented AI (A2I)?

A) To train foundation models
B) To add human review workflows for ML predictions when confidence is low
C) To generate synthetic data
D) To deploy models to edge devices

**Q64.** A company is using Amazon Bedrock and wants to compare the outputs of different foundation models (Claude, Titan, Llama) for their use case. What is this process called?

A) Fine-tuning
B) Model evaluation / benchmarking
C) Model deployment
D) Data labeling

**Q65.** A company wants to build an application that can answer questions about data stored in a vector database. Which concept is fundamental to this approach?

A) Relational database queries
B) Semantic search using embeddings
C) Keyword matching
D) Rule-based systems

**Q66.** Which of the following is a valid use case for Amazon Bedrock Agents?

A) Training a model from scratch
B) An AI assistant that can look up order status, process returns, and update customer records by calling backend APIs
C) Storing large datasets in S3
D) Managing IAM permissions

**Q67.** A company wants to use generative AI to help developers write and debug code. Which type of AI application is this?

A) Computer vision
B) AI-powered code assistant
C) Recommendation engine
D) Anomaly detection

**Q68.** What is the main advantage of using multiple foundation models through Amazon Bedrock rather than committing to a single model?

A) It is always cheaper
B) Different models have different strengths — you can choose the best model for each use case and avoid vendor lock-in
C) Multiple models always produce better results
D) It eliminates the need for prompt engineering

**Q69.** A company wants to extract key entities (people, places, organizations, dates) from news articles. Which AWS service should they use?

A) Amazon Textract
B) Amazon Comprehend entity recognition
C) Amazon Translate
D) Amazon Polly

**Q70.** What is a vector database and why is it important for generative AI applications?

A) A traditional relational database
B) A database optimized for storing and searching high-dimensional vector embeddings, enabling semantic similarity search for RAG applications
C) A database that only stores numbers
D) A database for storing model weights

**Q71.** A company wants to build a conversational chatbot that can handle multi-turn conversations, understand user intent, and trigger backend actions. Which AWS service is designed for this?

A) Amazon Comprehend
B) Amazon Lex
C) Amazon Polly
D) Amazon Translate

**Q72.** Which of the following BEST describes how Amazon Bedrock Knowledge Bases implements RAG?

A) It fine-tunes the model on your documents
B) It chunks your documents, creates vector embeddings, stores them in a vector database, and retrieves relevant chunks at query time to include in the model's context
C) It replaces the foundation model with your data
D) It sends all documents to the model in every request

---

## Domain 4: Guidelines for Responsible AI (Questions 73–86)

**Q73.** What is responsible AI?

A) AI that runs on renewable energy
B) The practice of developing and deploying AI systems that are fair, transparent, safe, and accountable
C) AI that is only used by government agencies
D) AI that does not use any data

**Q74.** What is AI bias and why is it a concern?

A) Bias is always intentional
B) AI bias occurs when a model produces systematically unfair outcomes for certain groups, often due to biased training data or flawed model design
C) Bias only affects image models
D) Bias improves model accuracy

**Q75.** A company's hiring AI model consistently ranks male candidates higher than equally qualified female candidates. What type of AI issue is this?

A) A performance optimization issue
B) Algorithmic bias / fairness issue
C) A latency problem
D) A cost optimization issue

**Q76.** What is explainability (interpretability) in the context of AI?

A) The speed at which a model makes predictions
B) The ability to understand and explain how an AI model arrives at its decisions or predictions
C) The cost of running a model
D) The size of the training dataset

**Q77.** Why is transparency important in AI systems?

A) It makes models run faster
B) It allows stakeholders to understand how decisions are made, builds trust, and enables accountability when things go wrong
C) It reduces the cost of AI
D) It eliminates all bias

**Q78.** A company is deploying an AI system that makes loan approval decisions. Regulators require the company to explain why each application was approved or denied. Which AI principle does this requirement address?

A) Scalability
B) Explainability and transparency
C) Cost optimization
D) Performance tuning

**Q79.** What is the purpose of human-in-the-loop (HITL) in AI systems?

A) To replace AI entirely with human workers
B) To include human oversight and review in AI workflows, especially for high-stakes decisions or low-confidence predictions
C) To speed up model training
D) To reduce the cost of AI

**Q80.** Which AWS service can be used to add human review workflows to AI predictions?

A) Amazon SageMaker Autopilot
B) Amazon Augmented AI (A2I)
C) Amazon Bedrock
D) AWS Lambda

**Q81.** What is the concept of fairness in AI?

A) All models should produce the same output
B) AI systems should treat all individuals and groups equitably, without discrimination based on protected attributes like race, gender, or age
C) Fairness means the model is always accurate
D) Fairness only applies to text models

**Q82.** A company wants to detect and measure bias in their ML model's predictions across different demographic groups. Which AWS tool should they use?

A) Amazon CloudWatch
B) Amazon SageMaker Clarify
C) AWS Cost Explorer
D) Amazon Macie

**Q83.** What is the risk of using AI-generated content without human review?

A) There is no risk
B) The content may contain hallucinations, biases, inaccuracies, or inappropriate material that could harm users or the company's reputation
C) It always produces perfect content
D) It only affects image generation

**Q84.** What is the concept of AI safety?

A) Physical security of AI hardware
B) Ensuring AI systems operate as intended without causing unintended harm, including preventing misuse, managing risks, and maintaining control over AI behavior
C) The speed of AI processing
D) The cost of AI development

**Q85.** A company is using generative AI to create customer-facing content. What safeguard should they implement to prevent harmful or inappropriate outputs?

A) Use the largest possible model
B) Implement content filtering and guardrails (e.g., Amazon Bedrock Guardrails) to block harmful, biased, or inappropriate content
C) Disable all safety features for faster responses
D) Only use the model during business hours

**Q86.** What is the importance of diverse and representative training data for AI fairness?

A) It has no impact on fairness
B) Training data that represents diverse populations helps prevent the model from learning and perpetuating biases against underrepresented groups
C) Diverse data always makes models slower
D) Only the model architecture affects fairness

---

## Domain 5: Security, Compliance, and Governance for AI Solutions (Questions 87–100)

**Q87.** A company is building an AI application that processes customer data. Which AWS service should they use to control who can access the AI resources?

A) Amazon Rekognition
B) AWS Identity and Access Management (IAM)
C) Amazon Polly
D) Amazon Translate

**Q88.** A company needs to ensure that data sent to Amazon Bedrock is encrypted in transit. Which protocol provides this encryption?

A) HTTP
B) FTP
C) TLS/SSL (HTTPS)
D) SMTP

**Q89.** A company wants to ensure that their AI training data stored in Amazon S3 is encrypted at rest. Which AWS service manages the encryption keys?

A) Amazon Bedrock
B) AWS Key Management Service (KMS)
C) Amazon Comprehend
D) AWS CloudTrail

**Q90.** A company needs to track all API calls made to their AI services for audit and compliance purposes. Which AWS service provides this capability?

A) Amazon CloudWatch
B) AWS CloudTrail
C) Amazon Macie
D) AWS Config

**Q91.** A company is using Amazon Bedrock and wants to ensure that no personally identifiable information (PII) is sent to the foundation model. Which Bedrock feature can help?

A) Bedrock Agents
B) Bedrock Knowledge Bases
C) Bedrock Guardrails with PII redaction
D) Bedrock Model Fine-tuning

**Q92.** What is data governance in the context of AI?

A) The speed of data processing
B) The policies, processes, and standards for managing data quality, access, privacy, and compliance throughout the AI lifecycle
C) The cost of storing data
D) The format of training data

**Q93.** A company operating in the healthcare industry needs to ensure their AI solution complies with HIPAA regulations. Which of the following is required? (Select TWO)

A) Use HIPAA-eligible AWS services
B) Have a Business Associate Agreement (BAA) with AWS
C) Store all data in public S3 buckets
D) Disable encryption for faster processing
E) Use only on-premises servers

**Q94.** What is the principle of least privilege in the context of AI security?

A) Giving all users full access to all AI resources
B) Granting users and services only the minimum permissions needed to perform their specific tasks
C) Using the smallest possible AI model
D) Reducing the amount of training data

**Q95.** A company wants to detect if sensitive data (PII, financial information) exists in their S3 data lake before using it for AI training. Which AWS service should they use?

A) Amazon Comprehend
B) Amazon Macie
C) Amazon Rekognition
D) Amazon Translate

**Q96.** What is model governance?

A) The speed of model inference
B) The policies and processes for managing the AI/ML model lifecycle, including version control, approval workflows, audit trails, and compliance documentation
C) The cost of training models
D) The size of the model

**Q97.** A company needs to ensure that their AI application meets industry-specific compliance requirements (SOC 2, ISO 27001). Where can they find AWS compliance documentation?

A) Amazon Bedrock console
B) AWS Artifact
C) Amazon CloudWatch
D) AWS Cost Explorer

**Q98.** What is the shared responsibility model for AI on AWS?

A) AWS is responsible for everything
B) The customer is responsible for everything
C) AWS secures the underlying infrastructure and services; the customer is responsible for their data, model inputs/outputs, access controls, and compliance with regulations
D) Neither party has any responsibility

**Q99.** A company wants to prevent their generative AI application from discussing certain prohibited topics (e.g., competitor products, political opinions). Which Amazon Bedrock feature should they use?

A) Bedrock Knowledge Bases
B) Bedrock Guardrails with denied topic filters
C) Bedrock Agents
D) Bedrock Model Fine-tuning

**Q100.** Why is it important to log and monitor AI model inputs and outputs in production?

A) It has no practical value
B) Logging enables auditing, debugging, detecting misuse, identifying bias, measuring performance, and meeting compliance requirements
C) It only matters for image models
D) Logging slows down the model and should be avoided
