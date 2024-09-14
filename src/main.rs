mod event;
mod character;

use character::Character;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // キャラクターの作成
    let alice = Arc::new(Mutex::new(Character::new("Alice")));
    let bob = Arc::new(Mutex::new(Character::new("Bob")));

    // Aliceのイベントを追加
    {
        let mut alice_locked = alice.lock().unwrap();
        let alice_event1 = alice_locked.add_event("Alice finds a mysterious key");
        let alice_event2 = alice_locked.add_event("Alice opens a hidden door");
        alice_locked.add_cause(&alice_event2, &alice_event1);
    }

    // Bobのイベントを追加
    {
        let mut bob_locked = bob.lock().unwrap();
        let bob_event1 = bob_locked.add_event("Bob hears a strange noise");
        let bob_event2 = bob_locked.add_event("Bob investigates the noise");
        bob_locked.add_cause(&bob_event2, &bob_event1);
    }

    // キャラクター同士の会話をスレッドで実行
    {
        let tx_alice = tx.clone();
        let alice_clone = Arc::clone(&alice);
        thread::spawn(move || {
            let alice = alice_clone.lock().unwrap();
            alice.speak("I found something interesting!", tx_alice);
            alice.print_causal_chains();
        });
    }

    {
        let tx_bob = tx.clone();
        let bob_clone = Arc::clone(&bob);
        thread::spawn(move || {
            let bob = bob_clone.lock().unwrap();
            bob.speak("Did you find anything, Alice?", tx_bob);
            bob.print_causal_chains();
        });
    }

    // メインスレッドでメッセージを受信して表示
    for received in rx.iter().take(4) { // スピーチと因果連鎖の出力
        println!("{}", received);
    }
}
