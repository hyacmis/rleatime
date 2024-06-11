use anyhow::Error;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

// 用于排序的数据块包装器
#[derive(Eq, PartialEq)]
pub struct SortWrapper {
    pub order_id: usize,
    pub data: Vec<u8>,
}

impl SortWrapper {
    pub fn new(order_id: usize, data: Vec<u8>) -> Self {
        Self { order_id, data }
    }
}

impl Ord for SortWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_id.cmp(&other.order_id)
    }
}

impl PartialOrd for SortWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type HeapSortWrapper = Arc<Mutex<BinaryHeap<Reverse<SortWrapper>>>>;

/// 使用 `trait` 为其提供方法
pub trait Handle {
    fn handle(&self) -> Result<Vec<u8>, Error>;
}

impl Handle for HeapSortWrapper {
    fn handle(&self) -> Result<Vec<u8>, Error> {
        // 提取并合并数据块，注意：由于是最大堆，我们实际上是反向遍历
        let mut data = Vec::new();

        while let Some(Reverse(wrapper)) = self.lock().unwrap().pop() {
            println!("ord {}", wrapper.order_id);
            data.extend_from_slice(&wrapper.data);
        }
        Ok(data)
    }
}
