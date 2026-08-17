# ML Model Training in VelinScript

VelinScript bietet native Unterstützung für Machine Learning Model Training mit ONNX Runtime und TensorFlow.

## TrainingService

Der `TrainingService` ermöglicht es, ML-Models zu trainieren:

```velin
let service = TrainingService.new();

// Training-Daten hinzufügen
service.add_example("input1", "output1");
service.add_example("input2", "output2");
```

## ONNX Training

### Basis-Training

```velin
let service = TrainingService.new();

// Training-Daten vorbereiten
service.add_example("feature1,feature2", "label1");
service.add_example("feature3,feature4", "label2");

// ONNX Training-Konfiguration
let config = ONNXTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "CrossEntropy"
};

// Model trainieren
let result = service.train_with_onnx("my_model", config);

match (result) {
    Ok(training_result) => {
        // Training erfolgreich
        // training_result.accuracy, training_result.loss, etc.
    },
    Error(err) => {
        // Fehlerbehandlung
    }
}
```

### Erweiterte ONNX-Konfiguration

```velin
let config = ONNXTrainingConfig {
    epochs: 200,
    batch_size: 64,
    learning_rate: 0.0001,
    optimizer: "SGD",
    loss_function: "MSE"
};

let result = service.train_with_onnx("regression_model", config);
```

## TensorFlow Training

### Basis-Training

```velin
let service = TrainingService.new();

// Training-Daten
for (example in trainingData) {
    service.add_example(example.input, example.output);
}

// TensorFlow Training-Konfiguration
let config = TensorFlowTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "SparseCategoricalCrossentropy",
    validation_split: 0.2
};

// Model trainieren
let result = service.train_with_tensorflow("tf_model", config);
```

### Erweiterte TensorFlow-Konfiguration

```velin
let config = TensorFlowTrainingConfig {
    epochs: 150,
    batch_size: 128,
    learning_rate: 0.0005,
    optimizer: "RMSprop",
    loss_function: "BinaryCrossentropy",
    validation_split: 0.3
};

let result = service.train_with_tensorflow("classification_model", config);
```

## Model Evaluation

Nach dem Training kannst du das Model evaluieren:

```velin
let testData = [
    TrainingExample { input: "test1", output: "expected1" },
    TrainingExample { input: "test2", output: "expected2" }
];

let evalResult = service.evaluate_model("my_model", testData);

match (evalResult) {
    Ok(metrics) => {
        // metrics.accuracy, metrics.precision, metrics.recall, metrics.f1_score
    },
    Error(err) => {
        // Fehlerbehandlung
    }
}
```

## Training Results

Das `ModelTrainingResult` enthält:

- `model_name`: Name des trainierten Models
- `framework`: "ONNX" oder "TensorFlow"
- `accuracy`: Genauigkeit des Models
- `loss`: Loss-Wert
- `epochs`: Anzahl der Epochen
- `training_time_seconds`: Trainingszeit in Sekunden

## Evaluation Results

Das `ModelEvaluationResult` enthält:

- `model_name`: Name des evaluierten Models
- `accuracy`: Genauigkeit
- `precision`: Präzision
- `recall`: Recall
- `f1_score`: F1-Score
- `test_samples`: Anzahl der Test-Samples

## Beispiel: Vollständiges Training

```velin
// Training Service initialisieren
let mut service = TrainingService.new();

// Training-Daten sammeln
for (data in trainingDataset) {
    service.add_example(data.input, data.label);
}

// ONNX Training
let onnxConfig = ONNXTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "CrossEntropy"
};

let trainingResult = service.train_with_onnx("sentiment_model", onnxConfig);

match (trainingResult) {
    Ok(result) => {
        // Model erfolgreich trainiert
        // result.accuracy, result.loss, etc.
        
        // Model evaluieren
        let evalResult = service.evaluate_model("sentiment_model", testData);
        match (evalResult) {
            Ok(metrics) => {
                // Evaluation erfolgreich
                // metrics.accuracy, metrics.precision, etc.
            },
            Error(err) => {
                // Evaluation fehlgeschlagen
            }
        }
    },
    Error(err) => {
        // Training fehlgeschlagen
    }
}
```

## Best Practices

1. **Daten vorbereiten**: Stelle sicher, dass Training-Daten korrekt formatiert sind
2. **Hyperparameter-Tuning**: Experimentiere mit verschiedenen Learning Rates und Batch Sizes
3. **Validation Split**: Verwende Validation Split für bessere Generalisierung
4. **Model Evaluation**: Evaluiere immer auf separaten Test-Daten
5. **Logging**: Nutze den VelinLogger für Training-Logs

## Integration mit VelinLogger

Der TrainingService nutzt automatisch VelinLogger:

```velin
let service = TrainingService.new();
// Logging erfolgt automatisch beim Training
let result = service.train_with_onnx("model", config);
// Logs enthalten: model_name, framework, training_examples, etc.
```

## Integration mit Metrics

Der TrainingService sammelt automatisch Metrics:

```velin
let service = TrainingService.new();
// Metrics werden automatisch gesammelt
// service.metrics.get_metrics() für alle Metrics
```
