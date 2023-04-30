// 上位モジュール、下位モジュール双方が抽象に依存することで
// 依存の方向が逆転すること

trait Storage {
    fn store(&self);
}

struct SessionStorage;

impl Storage for SessionStorage {
    fn store(&self) {
        println!("Store Session");
    }
}

struct CloudStorage;

impl Storage for CloudStorage {
    fn store(&self) {
        println!("Cloud storage");
    }
}

// Storageトレイトという抽象に依存している
fn save(storage: &dyn Storage) {
    storage.store();
}
