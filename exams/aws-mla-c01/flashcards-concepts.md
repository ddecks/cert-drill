## Data Preparation Concepts

Q: What is feature engineering and why does it matter for ML?
A: The process of creating, transforming, and selecting input variables (features) to improve model performance. Good features capture the underlying patterns in data. Techniques include scaling, encoding, binning, interaction features, and domain-specific transformations.

Q: What is the difference between normalization and standardization?
A: Normalization (min-max scaling) rescales values to [0, 1]. Standardization (z-score) transforms to zero mean and unit variance. Use normalization when the distribution is not Gaussian; use standardization when features follow a normal distribution or when using algorithms sensitive to feature scale.

Q: What is one-hot encoding and when should you avoid it?
A: Converts each categorical value into a binary column (1 if present, 0 otherwise). Avoid with high-cardinality features (hundreds of categories) as it creates very sparse, high-dimensional data. Use target encoding, embedding layers, or feature hashing instead.

Q: What is data leakage and how do you prevent it?
A: When information from outside the training set (e.g., future data, test set statistics) leaks into the model during training, causing overly optimistic performance. Prevent by splitting data before any preprocessing, using pipelines, and being careful with time-series data.

Q: What is the difference between data drift and concept drift?
A: Data drift: the statistical distribution of input features changes over time (e.g., customer demographics shift). Concept drift: the relationship between inputs and the target variable changes (e.g., what constitutes fraud evolves). Both degrade model performance.

Q: What are the common strategies for handling missing data?
A: Deletion (drop rows/columns), imputation (mean, median, mode, KNN, regression), indicator variables (add a binary flag for missingness), and model-based approaches (algorithms that handle missing values natively like XGBoost).

Q: What is the purpose of a feature store?
A: A centralized repository for storing, sharing, and serving ML features. Ensures consistency between training and inference features, reduces duplicate feature engineering, and provides feature versioning and lineage.

Q: What is the difference between structured, semi-structured, and unstructured data?
A: Structured: tabular data with fixed schema (CSV, databases). Semi-structured: has some organization but flexible schema (JSON, XML, Parquet). Unstructured: no predefined format (images, text, audio, video).

Q: What is the difference between Apache Parquet and Apache Avro?
A: Parquet is columnar — optimized for analytical queries and compression (read-heavy). Avro is row-based — optimized for write-heavy workloads and schema evolution. Use Parquet for ML training data; Avro for streaming ingestion.

Q: What is class imbalance and why is it a problem?
A: When one class significantly outnumbers others in the training data (e.g., 99% negative, 1% positive). Models tend to predict the majority class, achieving high accuracy but poor recall on the minority class. Address with resampling, SMOTE, cost-sensitive learning, or evaluation metrics like F1.

## Model Development Concepts

Q: What is the bias-variance tradeoff?
A: Bias: error from oversimplified assumptions (underfitting). Variance: error from sensitivity to training data fluctuations (overfitting). The goal is to find the sweet spot — a model complex enough to capture patterns but simple enough to generalize.

Q: What is overfitting and how do you detect it?
A: When a model memorizes training data instead of learning general patterns. Detected by a large gap between training performance (high) and validation performance (low). Address with regularization, dropout, early stopping, more data, or simpler models.

Q: What is underfitting and how do you detect it?
A: When a model is too simple to capture the underlying patterns. Detected by poor performance on both training and validation sets. Address with more complex models, more features, longer training, or reduced regularization.

Q: What is transfer learning?
A: Using a model pre-trained on a large dataset (e.g., ImageNet) as a starting point, then fine-tuning on a smaller, domain-specific dataset. Leverages learned features (edges, textures, shapes) to achieve good performance with limited data.

Q: What is the difference between supervised, unsupervised, and reinforcement learning?
A: Supervised: learns from labeled data (classification, regression). Unsupervised: finds patterns in unlabeled data (clustering, dimensionality reduction, anomaly detection). Reinforcement: learns by interacting with an environment and receiving rewards.

Q: What is regularization and why is it important?
A: Techniques that constrain model complexity to prevent overfitting. L1 (Lasso) adds absolute weight penalty, encouraging sparsity. L2 (Ridge) adds squared weight penalty, shrinking weights. Dropout randomly deactivates neurons during training.

Q: What is the difference between precision and recall?
A: Precision = TP / (TP + FP) — of all positive predictions, how many were correct. Recall = TP / (TP + FN) — of all actual positives, how many were found. Prioritize precision when false positives are costly; recall when false negatives are costly.

Q: What is the F1 score?
A: The harmonic mean of precision and recall: F1 = 2 × (precision × recall) / (precision + recall). Ranges from 0 to 1. Useful when you need a single metric that balances both precision and recall, especially with imbalanced classes.

Q: What is AUC-ROC?
A: Area Under the Receiver Operating Characteristic curve. Plots true positive rate vs false positive rate at various thresholds. AUC = 1.0 is perfect, 0.5 is random. Measures a model's ability to distinguish between classes regardless of threshold.

Q: What is RMSE and when do you use it?
A: Root Mean Squared Error — the square root of the average squared differences between predicted and actual values. Used for regression problems. Penalizes large errors more than MAE. Same units as the target variable.

Q: What is hyperparameter tuning?
A: The process of finding optimal hyperparameters (learning rate, batch size, regularization strength, tree depth) that are set before training. Methods: grid search (exhaustive), random search (sampled), Bayesian optimization (informed by previous results).

Q: What is early stopping?
A: Halting training when validation performance stops improving (or starts degrading). Prevents overfitting by finding the optimal number of training epochs. Requires monitoring a validation metric and setting a patience parameter.

Q: What is the difference between bagging and boosting?
A: Bagging (Bootstrap Aggregating): trains multiple models on random subsets of data in parallel, then averages predictions (e.g., Random Forest). Boosting: trains models sequentially, each correcting the errors of the previous one (e.g., XGBoost, AdaBoost).

Q: What is retrieval-augmented generation (RAG)?
A: A technique that enhances LLM responses by retrieving relevant documents from a knowledge base and including them as context in the prompt. Reduces hallucinations and grounds responses in factual data without fine-tuning the model.

Q: What is the difference between fine-tuning and prompt engineering?
A: Fine-tuning: updates model weights using domain-specific training data, permanently changing the model. Prompt engineering: crafts input prompts to guide model behavior without changing weights. Fine-tuning is more powerful but requires data and compute; prompt engineering is faster and cheaper.

## Deployment and Orchestration Concepts

Q: What is the difference between real-time, batch, and asynchronous inference?
A: Real-time: persistent endpoint, sub-second latency, always on. Batch: process large datasets offline, no latency requirement, cost-effective. Asynchronous: queues requests, handles large payloads and long processing times, results delivered to S3.

Q: What is a canary deployment?
A: A deployment strategy that routes a small percentage of traffic (e.g., 10%) to the new model version while the majority stays on the current version. If the new version performs well, traffic is gradually shifted. Minimizes risk of bad deployments.

Q: What is blue/green deployment?
A: Maintaining two identical environments (blue = current, green = new). Traffic is switched from blue to green all at once after validation. Enables instant rollback by switching back to blue if issues arise.

Q: What is A/B testing for ML models?
A: Running two or more model versions simultaneously with different traffic splits to compare real-world performance. Uses production variants on SageMaker endpoints with configurable traffic weights.

Q: What is model quantization?
A: Reducing the numerical precision of model weights and activations (e.g., FP32 to INT8 or FP16). Reduces model size, memory usage, and inference latency with minimal accuracy loss. Supported by SageMaker Neo.

Q: What is model pruning?
A: Removing unnecessary weights or neurons from a trained model that contribute little to predictions. Reduces model size and inference time. Can be structured (remove entire filters/layers) or unstructured (remove individual weights).

Q: What is an inference pipeline?
A: A sequence of containers that process data in order during inference. Typically: preprocessing → model inference → post-processing. Ensures the same transformations applied during training are applied during inference.

Q: What is CI/CD for ML (MLOps)?
A: Continuous Integration/Continuous Delivery applied to ML. Automates data validation, model training, evaluation, testing, and deployment. Ensures reproducibility, quality gates, and rapid iteration. Uses tools like SageMaker Pipelines, CodePipeline, and CodeBuild.

Q: What is infrastructure as code (IaC) for ML?
A: Defining ML infrastructure (endpoints, training jobs, pipelines) in code templates (CloudFormation, CDK, Terraform). Enables repeatable, version-controlled, and auditable deployments. Eliminates manual console configuration.

Q: What is auto-scaling for ML endpoints?
A: Automatically adjusting the number of instances behind an endpoint based on traffic metrics (e.g., InvocationsPerInstance). Scales up during peak traffic and down during idle periods to optimize cost and performance.

Q: What is the difference between data parallelism and model parallelism?
A: Data parallelism: splits the training data across multiple GPUs/instances, each running a copy of the full model. Model parallelism: splits the model itself across multiple GPUs when it's too large for a single GPU's memory.

## Monitoring, Maintenance, and Security Concepts

Q: What are the four types of SageMaker Model Monitor?
A: 1) Data Quality Monitor — detects data distribution drift. 2) Model Quality Monitor — tracks prediction accuracy. 3) Bias Drift Monitor — detects changes in bias metrics. 4) Feature Attribution Drift Monitor — tracks changes in feature importance (SHAP values).

Q: What is a model baseline in monitoring?
A: A statistical profile of the training data and model performance used as a reference point. Model Monitor compares incoming inference data and predictions against this baseline to detect drift and degradation.

Q: What is the principle of least privilege for ML?
A: Granting only the minimum IAM permissions needed for each role. Data scientists get training permissions but not endpoint management. ML engineers get deployment permissions. No one gets full admin access unless required.

Q: What is network isolation in SageMaker?
A: EnableNetworkIsolation=true prevents containers from making outbound network calls. Training containers can only access data through SageMaker input channels. Inference containers can only respond to incoming requests. Prevents data exfiltration.

Q: What is the difference between encryption at rest and encryption in transit?
A: At rest: data is encrypted when stored (S3, EBS, SageMaker volumes) using AWS KMS. In transit: data is encrypted during transmission between services using TLS/SSL. Both are required for comprehensive data protection.

Q: What is a VPC endpoint and why is it important for ML?
A: A private connection between your VPC and an AWS service (e.g., S3, SageMaker) that doesn't traverse the public internet. Keeps data within the AWS network, improving security and reducing latency. Gateway endpoints for S3/DynamoDB; interface endpoints for other services.

Q: What is the purpose of CloudTrail for ML workloads?
A: Logs all API calls to AWS services, providing an audit trail of who created, modified, or deleted ML resources. Essential for compliance, security investigations, and governance. Records the identity, time, source IP, and parameters of each call.

Q: What is model governance?
A: Policies and processes for managing the ML model lifecycle. Includes model approval workflows, version tracking, audit trails, bias monitoring, and compliance documentation. SageMaker Model Registry provides approval status and metadata tracking.

Q: What is the difference between SageMaker Managed Spot Training and On-Demand?
A: Spot Training uses spare EC2 capacity at up to 90% discount but can be interrupted. SageMaker handles checkpointing for fault tolerance. On-Demand provides guaranteed capacity at full price. Use Spot for fault-tolerant jobs; On-Demand for time-critical training.

Q: What is HIPAA compliance for ML on AWS?
A: Requires using HIPAA-eligible AWS services, having a Business Associate Agreement (BAA) with AWS, encrypting PHI at rest and in transit, implementing access controls, and maintaining audit logs. SageMaker is a HIPAA-eligible service.

Q: How do you implement automated model retraining?
A: Monitor model quality metrics with SageMaker Model Monitor → CloudWatch alarm triggers when metrics degrade → Lambda function starts a SageMaker Pipeline → Pipeline retrains, evaluates, and conditionally deploys the new model if it meets quality thresholds.

Q: What is the difference between model monitoring and model observability?
A: Monitoring: tracking predefined metrics (accuracy, latency, drift) against thresholds and alerting on violations. Observability: deeper understanding of model behavior through logs, traces, and metrics — enabling diagnosis of why a model is behaving a certain way, not just that it is.
