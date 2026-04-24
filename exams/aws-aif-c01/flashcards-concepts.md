## AI and ML Fundamentals

Q: What is artificial intelligence (AI)?
A: The broad field of creating machines that can perform tasks that typically require human intelligence — reasoning, learning, problem-solving, perception, and language understanding.

Q: What is machine learning (ML)?
A: A subset of AI where systems learn patterns from data and improve with experience, without being explicitly programmed. The model learns from examples rather than following hard-coded rules.

Q: What is deep learning?
A: A subset of ML that uses neural networks with multiple hidden layers. Deep networks can learn complex, hierarchical patterns in data (images, text, speech). Powers most modern AI breakthroughs.

Q: What is supervised learning?
A: ML where the model learns from labeled data (input-output pairs). Classification predicts categories (spam/not spam). Regression predicts continuous values (house prices). Requires labeled training data.

Q: What is unsupervised learning?
A: ML where the model finds patterns in unlabeled data. Clustering groups similar items. Dimensionality reduction compresses features. Anomaly detection finds outliers. No labels required.

Q: What is reinforcement learning?
A: ML where an agent learns by interacting with an environment, receiving rewards for good actions and penalties for bad ones. Used in robotics, game playing, and recommendation systems.

Q: What is the ML development lifecycle?
A: Define problem → Collect and prepare data → Choose algorithm → Train model → Evaluate performance → Deploy to production → Monitor and iterate. Each step may require revisiting earlier steps.

Q: What is a feature in ML?
A: An input variable used by the model to make predictions. Examples: age, income, purchase history. Good feature selection and engineering significantly impact model performance.

Q: What is a label in ML?
A: The known correct output (target value) in supervised learning training data. The model learns to predict labels from features. Examples: "spam"/"not spam", price values.

Q: What is training vs inference?
A: Training: the model learns patterns from data (computationally expensive, done periodically). Inference: the trained model makes predictions on new data (fast, done in production continuously).

Q: What is overfitting?
A: When a model memorizes training data instead of learning general patterns. High training accuracy but poor performance on new data. Fix with regularization, more data, simpler models, or early stopping.

Q: What is underfitting?
A: When a model is too simple to capture underlying patterns. Poor performance on both training and new data. Fix with more complex models, more features, or longer training.

Q: What is a confusion matrix?
A: A table showing true positives, true negatives, false positives, and false negatives for a classification model. Used to calculate accuracy, precision, recall, and F1 score.

Q: What is accuracy vs precision vs recall?
A: Accuracy: overall correct predictions / total predictions. Precision: correct positive predictions / all positive predictions. Recall: correct positive predictions / all actual positives. Use F1 to balance precision and recall.

Q: What is RMSE?
A: Root Mean Squared Error — measures average prediction error for regression models. Lower is better. Same units as the target variable. Penalizes large errors more than MAE.

## Generative AI Concepts

Q: What is generative AI?
A: AI that creates new content — text, images, code, audio, video — based on patterns learned from training data. Unlike discriminative AI which classifies existing data.

Q: What is a foundation model?
A: A large model pre-trained on massive, diverse datasets that learns general knowledge. Can be adapted to many downstream tasks through prompting or fine-tuning. Examples: GPT, Claude, Titan, Llama.

Q: What is a large language model (LLM)?
A: A type of foundation model specifically trained on text data. Uses billions of parameters to understand and generate human language. Based on the transformer architecture.

Q: What is a prompt?
A: The input text or instruction given to a generative AI model. The quality and structure of the prompt significantly affects the output quality. Includes the question, context, and any examples.

Q: What is prompt engineering?
A: The practice of crafting and refining prompts to get better outputs. Techniques include zero-shot, few-shot, chain-of-thought, and system prompts. No model retraining required.

Q: What is zero-shot vs few-shot prompting?
A: Zero-shot: give the model an instruction with no examples. Few-shot: provide several examples in the prompt to demonstrate the desired format and behavior. Few-shot generally improves output quality.

Q: What is chain-of-thought prompting?
A: A technique that asks the model to reason step by step before giving a final answer. Improves accuracy on math, logic, and complex reasoning tasks. Example: "Think step by step..."

Q: What is a hallucination?
A: When a generative AI model produces content that sounds plausible but is factually incorrect or fabricated. A key challenge with LLMs. Mitigated by RAG, grounding, and human review.

Q: What is RAG (retrieval-augmented generation)?
A: A technique that retrieves relevant information from external sources and includes it in the prompt context. Grounds the model's response in factual data, reducing hallucinations without fine-tuning.

Q: What is fine-tuning?
A: Further training a pre-trained foundation model on a smaller, domain-specific dataset. Adapts the model for a particular task while retaining general knowledge. More powerful than prompting but requires data and compute.

Q: What is a token?
A: A unit of text the model processes — can be a word, subword, or character. Models have token limits (context window). Pricing is often per-token. "ChatGPT" might be 2-3 tokens.

Q: What is the context window?
A: The maximum number of tokens (input + output) a model can process in a single request. Larger windows allow longer documents but cost more. Ranges from thousands to millions of tokens.

Q: What is temperature in generative AI?
A: Controls output randomness. Low temperature (0.1): deterministic, focused, factual. High temperature (0.9): creative, varied, surprising. Set based on use case — low for facts, high for creative writing.

Q: What is an embedding?
A: A numerical vector representation of data that captures semantic meaning. Similar items have similar vectors. Used for semantic search, recommendations, and RAG. Stored in vector databases.

Q: What is a vector database?
A: A database optimized for storing and searching high-dimensional vector embeddings. Enables fast similarity search for RAG applications. Examples: Amazon OpenSearch, Pinecone, pgvector.

Q: What is the transformer architecture?
A: The neural network architecture behind modern LLMs. Uses self-attention mechanisms to process sequences in parallel (not sequentially). Enables training on massive datasets. Powers GPT, BERT, Claude.

Q: What is multi-modal AI?
A: AI that can process and generate multiple data types (text, images, audio, video) within a single model. Example: a model that can describe images or generate images from text.

Q: What is the difference between pre-training and fine-tuning?
A: Pre-training: initial training on a large, general dataset (expensive, done once). Fine-tuning: additional training on a smaller, task-specific dataset (cheaper, done per use case). Pre-training creates the foundation; fine-tuning specializes it.

## Responsible AI Concepts

Q: What is responsible AI?
A: Developing and deploying AI that is fair, transparent, explainable, safe, private, and accountable. Ensures AI benefits users without causing harm or perpetuating discrimination.

Q: What is AI bias?
A: Systematic unfairness in AI outputs toward certain groups. Sources: biased training data, biased feature selection, historical inequalities in data. Can perpetuate discrimination in hiring, lending, healthcare.

Q: What is explainability in AI?
A: The ability to understand and explain how a model makes decisions. Critical for trust, debugging, regulatory compliance, and identifying bias. Ranges from simple feature importance to detailed decision paths.

Q: What is transparency in AI?
A: Openness about how AI systems work, what data they use, and how decisions are made. Builds trust, enables accountability, and supports regulatory compliance.

Q: What is fairness in AI?
A: AI systems treating all individuals and groups equitably without discrimination based on protected attributes (race, gender, age). Requires diverse training data, bias testing, and ongoing monitoring.

Q: What is human-in-the-loop (HITL)?
A: Including human oversight in AI workflows. Humans review high-stakes decisions, verify low-confidence predictions, and provide feedback. Essential for responsible AI in critical applications.

Q: What is AI safety?
A: Ensuring AI systems operate as intended without causing unintended harm. Includes preventing misuse, managing risks, maintaining human control, and ensuring predictable behavior.

Q: Why is diverse training data important?
A: Data representing diverse populations prevents the model from learning biases against underrepresented groups. Homogeneous data leads to poor performance and unfair outcomes for excluded populations.

## Security, Compliance, and Governance Concepts

Q: What is the AWS shared responsibility model for AI?
A: AWS secures the cloud infrastructure (hardware, networking, managed services). Customers secure their data, model inputs/outputs, access controls, and regulatory compliance. Responsibility is shared.

Q: What is the principle of least privilege?
A: Granting users and services only the minimum permissions needed for their specific tasks. Reduces the blast radius of compromised credentials. Applied through IAM policies.

Q: What is encryption at rest vs in transit?
A: At rest: data encrypted when stored (S3, EBS) using KMS keys. In transit: data encrypted during transmission using TLS/SSL (HTTPS). Both required for comprehensive data protection.

Q: What is data governance for AI?
A: Policies, processes, and standards for managing data quality, access, privacy, retention, and compliance throughout the AI lifecycle. Ensures data is properly handled from collection to deletion.

Q: What is model governance?
A: Managing the AI model lifecycle: version control, approval workflows, audit trails, lineage tracking, and compliance documentation. Ensures models are reviewed and approved before production.

Q: What is PII and why does it matter for AI?
A: Personally Identifiable Information — data that can identify an individual (name, SSN, email, address). AI systems must protect PII through encryption, access controls, redaction, and compliance with privacy regulations.

Q: What is a BAA (Business Associate Agreement)?
A: A contract between AWS and the customer required for HIPAA compliance. Defines each party's responsibilities for protecting PHI (Protected Health Information). Must be in place before processing healthcare data.

Q: What is AWS Artifact?
A: On-demand access to AWS compliance reports and agreements (SOC 2, ISO 27001, PCI DSS, HIPAA BAA). The central resource for demonstrating AWS compliance to auditors and regulators.
