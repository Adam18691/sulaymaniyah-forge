use sulaymaniyah_forge::{Forge, initialize};
use std::env;

fn main() {
    // 1. تهيئة النظام
    initialize();

    // 2. قراءة المدخلات من سطر الأوامر
    let args: Vec<String> = env::args().collect();
    
    // قيم افتراضية في حال عدم تمرير مدخلات
    let mut topic = String::from("The AI Empire");
    let mut lang = String::from("en");

    // تحليل المدخلات للبحث عن --topic و --lang
    for i in 0..args.len() {
        if args[i] == "--topic" && i + 1 < args.len() {
            topic = args[i + 1].clone();
        }
        if args[i] == "--lang" && i + 1 < args.len() {
            lang = args[i + 1].clone();
        }
    }

    println!("🚀 Starting Forge...");
    println!("📖 Topic: {}", topic);
    println!("🌐 Language: {}", lang);

    // 3. إنشاء الكائن وتمرير "الموضوع" إليه
    let my_forge = Forge::new(&topic);
    
    // 4. تشغيل العملية وتمرير "اللغة" إليها
    my_forge.run(&lang);

    println!("✅ Process completed successfully.");
}
