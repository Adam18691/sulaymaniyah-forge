pub struct Forge {
    pub topic: String,
}

impl Forge {
    // تأكد أن الدالة تأخذ &str كمدخل
    pub fn new(topic: &str) -> Self {
        Forge {
            topic: topic.to_string(),
        }
    }

    // تأكد أن دالة run تأخذ &str (اللغة) كمدخل
    pub fn run(&self, lang: &str) {
        println!("Generating book about {} in language {}...", self.topic, lang);
        // هنا تضع منطق توليد الكتاب الخاص بك
    }
}

pub fn initialize() {
    println!("Initializing Sulaymaniyah Forge System...");
    // منطق التهيئة الخاص بك
}
