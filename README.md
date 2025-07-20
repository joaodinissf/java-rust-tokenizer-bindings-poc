# Java-Rust Tokenizer Bindings PoC

A Proof of Concept demonstrating how to integrate Rust tokenization libraries with Java applications using JNI (Java Native Interface). This project showcases calling Rust tokenization functions from Java, specifically using the HuggingFace tokenizers library.wh

## Overview

This PoC consists of:

- **Java Application**: `java-hello/` - Java code that calls native Rust functions for tokenization
- **Rust Library**: `rust-hello/` - Rust library implementing tokenization using HuggingFace tokenizers
- **Eclipse Workspace**: `java-hello-ws/` - Eclipse IDE workspace for Java development

## Features

- **Native Tokenization**: Fast tokenization using Rust's HuggingFace tokenizers library
- **JNI Integration**: Seamless communication between Java and Rust
- **Multiple Models**: Support for different tokenizer models (currently Qwen/Qwen2.5-Coder-14B)
- **JSON Output**: Structured tokenization results with tokens, token IDs, and model information

## Prerequisites

- **Java**: JDK 8 or higher
- **Rust**: Latest stable version with Cargo
- **System**: macOS, Linux, or Windows with appropriate toolchains

## Project Structure

```
├── java-hello/           # Java application
│   ├── src/
│   │   └── TokenizerPoC.java
│   └── bin/
├── rust-hello/           # Rust tokenization library
│   ├── src/
│   │   ├── lib.rs        # JNI bindings and tokenization logic
│   │   └── main.rs       # Standalone Rust application
│   ├── tokenizers/       # Tokenizer model files
│   └── Cargo.toml
└── java-hello-ws/        # Eclipse workspace
```

## Getting Started

### 1. Build the Rust Library

```bash
cd rust-hello
cargo build --release
```

This will create the native library that Java will load.

### 2. Set Library Path

Ensure the Rust library is in your system's library path or specify it when running Java:

```bash
# macOS/Linux
export LD_LIBRARY_PATH=/path/to/rust-hello/target/release:$LD_LIBRARY_PATH

# Or specify when running Java
java -Djava.library.path=/path/to/rust-hello/target/release HelloWorld
```

### 3. Compile and Run Java

```bash
cd java-hello
javac src/main/java/TokenizerPoC.java -d bin
java -cp bin TokenizerPoC
```

## Usage Example

The Java application demonstrates:

1. **Basic Greeting**: Simple string passing between Java and Rust
2. **Arithmetic**: Integer operations in Rust called from Java  
3. **Tokenization**: Text tokenization using HuggingFace tokenizers

```java
// Tokenize text using Rust
String tokenResult = tokenizeText("Hello, how are you?", "Qwen/Qwen2.5-Coder-14B");
System.out.println("Tokenization result: " + tokenResult);
```

Expected output:

```json
{
  "tokens": ["Hello", ",", " how", " are", " you", "?"],
  "token_ids": [9906, 11, 1293, 527, 499, 30],
  "model": "Qwen/Qwen2.5-Coder-14B"
}
```

## Supported Models

Currently supported tokenizer models:

- **Qwen/Qwen2.5-Coder-14B**: Code-focused language model tokenizer

## Technical Details

### JNI Function Signatures

The Rust library exports these JNI functions:

- `Java_TokenizerPoC_greetFromRust`: String manipulation example
- `Java_TokenizerPoC_addNumbers`: Integer arithmetic example  
- `Java_TokenizerPoC_tokenizeText`: Main tokenization functionality

### Dependencies

**Rust (Cargo.toml):**

- `jni`: JNI bindings for Rust
- `tokenizers`: HuggingFace tokenizers library
- `serde_json`: JSON serialization

**Java:**

- Standard JDK (no external dependencies)

## Development

### Adding New Models

1. Add tokenizer files to `rust-hello/tokenizers/`
2. Update the model matching logic in `tokenizeText` function
3. Rebuild the Rust library

### IDE Support

The project includes an Eclipse workspace (`java-hello-ws/`) for Java development with pre-configured build settings.

## Troubleshooting

### Common Issues

1. **Library Loading Errors**: Ensure the Rust library is built and in the correct path
2. **Model Not Found**: Verify tokenizer files exist in the expected location
3. **JNI Errors**: Check that function signatures match between Java and Rust

### Building for Different Platforms

The library needs to be built for the target platform:

- **macOS**: Creates `.dylib` files
- **Linux**: Creates `.so` files  
- **Windows**: Creates `.dll` files

## Future Enhancements

- [ ] Support for additional tokenizer models
- [ ] Batch tokenization for improved performance
- [ ] Configuration file for model paths
- [ ] Error handling improvements
- [ ] Performance benchmarking tools

## License

This is a Proof of Concept project for demonstration purposes.
