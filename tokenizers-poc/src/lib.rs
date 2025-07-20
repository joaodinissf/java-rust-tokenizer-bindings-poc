use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use tokenizers::tokenizer::Tokenizer;
use serde_json;

#[unsafe(no_mangle)]
pub extern "system" fn Java_main_java_TokenizerPoC_greetFromRust<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
) -> jstring {
    let name: String = env
        .get_string(&name)
        .expect("Couldn't get java string!")
        .into();
    
    let greeting = format!("Greetings from Tokenizers PoC, {}!", name);
    
    let output = env
        .new_string(greeting)
        .expect("Couldn't create java string!");
    
    output.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_main_java_TokenizerPoC_addNumbers<'local>(
    _env: JNIEnv<'local>,
    _class: JClass<'local>,
    a: i32,
    b: i32,
) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_main_java_TokenizerPoC_tokenizeText<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    text: JString<'local>,
    model_name: JString<'local>,
) -> jstring {
    let text_str: String = match env.get_string(&text) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: Could not read text").unwrap().into_raw(),
    };
    
    let model_name_str: String = match env.get_string(&model_name) {
        Ok(s) => s.into(),
        Err(_) => return env.new_string("Error: Could not read model name").unwrap().into_raw(),
    };
    
    let tokenizer_path = match model_name_str.as_str() {
        "Qwen/Qwen2.5-Coder-14B" => "/Users/joao/Git/java-rust-bindings/tokenizers-poc/tokenizers/qwen2.5-coder-14b/tokenizer.json",
        _ => {
            let error_msg = format!("Unsupported model: {}. Only 'Qwen/Qwen2.5-Coder-14B' is available.", model_name_str);
            return env.new_string(error_msg).unwrap().into_raw();
        }
    };
    
    let tokenizer = match Tokenizer::from_file(tokenizer_path) {
        Ok(t) => t,
        Err(e) => {
            let error_msg = format!("Error loading tokenizer from '{}': {}", tokenizer_path, e);
            return env.new_string(error_msg).unwrap().into_raw();
        }
    };
    
    let encoding = match tokenizer.encode(text_str.as_str(), false) {
        Ok(enc) => enc,
        Err(e) => {
            let error_msg = format!("Error tokenizing text: {}", e);
            return env.new_string(error_msg).unwrap().into_raw();
        }
    };
    
    let tokens = encoding.get_tokens();
    let token_ids = encoding.get_ids();
    
    let result = serde_json::json!({
        "tokens": tokens,
        "token_ids": token_ids,
        "model": model_name_str
    });
    
    let result_str = result.to_string();
    env.new_string(result_str).unwrap().into_raw()
}