use sulaymaniyah_forge::{Forge, initialize};
use std::env;

fn main() {
    // 1. تهيئة النظام
    initialize();

    // 2. قراءة المدخلات من سطر الأوامر (التي تأتي من ملف الـ YAML)
    let args: Vec<String> = env::args().collect();
    
    let mut topic = String::from("The AI Empire");
    let mut lang = String::from("en");

    for i in 0..args.len() {
        if args[i] == "--topic" && i + 1 < args.len() {
            topic = args[i + 1].clone();
        }
        if args[i] == "--lang" && i + 1 < args.len() {
            lang = args[i + 1].clone();
        }
    }

    println!("🚀 Starting Forge Engine...");
    println!("📖 Topic: {}", topic);
    println!("🌐 Language: {}", lang);

    // 3. إنشاء محرك الكتابة وتمرير الموضوع
    let my_forge = Forge::new(&topic);
    
    // 4. تشغيل عملية التوليد وتمرير اللغة
    my_forge.run(&lang);

    println!("✅ Process completed successfully.");
}
