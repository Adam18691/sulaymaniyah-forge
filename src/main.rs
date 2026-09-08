use sulaymaniyah_forge::{Forge, initialize};

fn main() {
    // استدعاء دالة من المكتبة (lib.rs)
    initialize();

    // إنشاء كائن من Forge وتشغيله
    let my_forge = Forge::new("Sulaymaniyah Main Forge");
    my_forge.run();

    println!("Process completed successfully.");
}
