use std::fs::File;
use std::io::Write;
use std::env;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub struct Forge {
    pub topic: String,
}

impl Forge {
    pub fn new(topic: &str) -> Self {
        Forge {
            topic: topic.to_string(),
        }
    }

    pub fn run(&self, lang: &str) {
        println!("🌐 Connecting to Groq AI Engine...");

        // جلب المفتاح من الـ Secrets في GitHub
        let api_key = match env::var("GROQ_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("❌ Error: GROQ_API_KEY not found in environment variables.");
                return;
            }
        };

        // بناء أمر كتابة احترافي (Prompt)
        let prompt = format!(
            "You are a world-class professional author. Write a comprehensive, detailed, and high-quality e-book about the following topic: '{}'. \
            The book must be written entirely in the '{}' language. \
            Structure the book with: \
            1. A compelling Title. \
            2. A detailed Table of Contents. \
            3. An engaging Introduction. \
            4. At least 5 detailed chapters with sub-headings, practical tips, and examples. \
            5. A strong Conclusion. \
            Use professional Markdown formatting (H1, H2, H3, bold, lists) for a polished look.",
            self.topic, lang
        );

        let request_body = GroqRequest {
            model: "llama3-70b-8192".to_string(), 
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let client = Client::new();
        let response = client.post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send();

        match response {
            Ok(res) => {
                if let Ok(json_res) = res.json::<GroqResponse>() {
                    if let Some(choice) = json_res.choices.first() {
                        let book_content = &choice.message.content;
                        self.save_to_file(book_content);
                        println!("✅ Book generated and saved to book.md");
                    }
                } else {
                    println!("❌ Error: Failed to parse AI response.");
                }
            }
            Err(e) => println!("❌ Connection Error: {}", e),
        }
    }

    fn save_to_file(&self, content: &str) {
        let mut file = File::create("book.md").expect("Unable to create file");
        file.write_all(content.as_bytes()).expect("Unable to write data");
    }
}

pub fn initialize() {
    println!("🚀 Sulaymaniyah Forge V2 System Initialized...");
}
