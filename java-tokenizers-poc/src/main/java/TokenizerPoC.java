package main.java;


import java.io.*;
import java.nio.file.*;

public class TokenizerPoC {
    static {
        loadNativeLibrary();
    }
    
    private static void loadNativeLibrary() {
        try {
            String osName = System.getProperty("os.name").toLowerCase();
            String libName;
            
            if (osName.contains("mac")) {
                libName = "libtokenizers_poc.dylib";
            } else if (osName.contains("linux")) {
                libName = "libtokenizers_poc.so";
            } else if (osName.contains("windows")) {
                libName = "tokenizers_poc.dll";
            } else {
                throw new UnsupportedOperationException("Unsupported OS: " + osName);
            }
            
            InputStream in = TokenizerPoC.class.getResourceAsStream("/native/" + libName);
            if (in == null) {
                throw new FileNotFoundException("Native library not found in classpath: /native/" + libName);
            }
            
            Path tempFile = Files.createTempFile("tokenizers_poc", libName.substring(libName.lastIndexOf('.')));
            Files.copy(in, tempFile, StandardCopyOption.REPLACE_EXISTING);
            in.close();
            
            System.load(tempFile.toAbsolutePath().toString());
            
            tempFile.toFile().deleteOnExit();
            
        } catch (Exception e) {
            System.err.println("Failed to load native library: " + e.getMessage());
            e.printStackTrace();
            throw new RuntimeException("Could not load native library", e);
        }
    }
    
    public static native String greetFromRust(String name);
    public static native int addNumbers(int a, int b);
    public static native String tokenizeText(String text, String modelName);
    
    public static void main(String[] args) {
        System.out.println("Java-Rust Tokenizer Bindings PoC");
        System.out.println("==================================");
        
        // Test basic JNI functionality
        String greeting = greetFromRust("Java Tokenizers PoC");
        System.out.println(greeting);
        
        // Test arithmetic operations
        int result = addNumbers(5, 3);
        System.out.println("Arithmetic test: 5 + 3 = " + result);
        
        // Main tokenization demonstration
        System.out.println("\nTokenization Examples:");
        System.out.println("---------------------");
        
        String[] testTexts = {
            "Hello, how are you?",
            "This is a tokenization test for the PoC.",
            "function main() { return 'code tokenization'; }"
        };
        
        for (String text : testTexts) {
            System.out.println("Input: " + text);
            String tokenResult = tokenizeText(text, "Qwen/Qwen2.5-Coder-14B");
            System.out.println("Tokens: " + tokenResult);
            System.out.println();
        }
    }
}