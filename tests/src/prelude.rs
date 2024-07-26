use std::env;

use ckb_testtool::{
    ckb_error::Error,
    ckb_types::core::{Cycle, TransactionView},
    context::Context,
};

// This helper method runs Context::verify_tx, but in case error happens,
// it also dumps current transaction to failed_txs folder.
pub trait ContextExt {
    fn should_be_passed(&self, tx: &TransactionView, max_cycles: Cycle) -> Result<Cycle, Error>;
    fn should_be_failed(&self, tx: &TransactionView, max_cycles: Cycle) -> Result<Cycle, Error>;

    fn should_be_passed_without_limit(&self, tx: &TransactionView) -> Result<Cycle, Error> {
        self.should_be_passed(tx, Cycle::MAX)
    }
    fn should_be_failed_without_limit(&self, tx: &TransactionView) -> Result<Cycle, Error> {
        self.should_be_failed(tx, Cycle::MAX)
    }
}

impl ContextExt for Context {
    fn should_be_passed(&self, tx: &TransactionView, max_cycles: Cycle) -> Result<Cycle, Error> {
        let result = self.verify_tx(tx, max_cycles);
        if let Err(err) = result {
            let mut path = env::current_dir().expect("current dir");
            path.push("failed_txs");
            std::fs::create_dir_all(&path).expect("create failed_txs dir");
            let mock_tx = self.dump_tx(tx).expect("dump failed tx");
            let json = serde_json::to_string_pretty(&mock_tx).expect("json");
            path.push(format!("0x{:x}.json", tx.hash()));
            println!("Failed tx written to {:?}", path);
            std::fs::write(path, json).expect("write");
            panic!("should be passed, but failed since {err}");
        }
        result
    }

    fn should_be_failed(&self, tx: &TransactionView, max_cycles: Cycle) -> Result<Cycle, Error> {
        let result = self.verify_tx(tx, max_cycles);
        if result.is_ok() {
            let mut path = env::current_dir().expect("current dir");
            path.push("failed_txs");
            std::fs::create_dir_all(&path).expect("create failed_txs dir");
            let mock_tx = self.dump_tx(tx).expect("dump failed tx");
            let json = serde_json::to_string_pretty(&mock_tx).expect("json");
            path.push(format!("0x{:x}.json", tx.hash()));
            println!("Failed tx written to {:?}", path);
            std::fs::write(path, json).expect("write");
            panic!("should be failed");
        }
        result
    }
}
