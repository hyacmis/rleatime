use anyhow::Error;
use indicatif::{MultiProgress, ProgressStyle};
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    sync::{Arc, Mutex},
};

/// 使用二叉树对线程下载的文件块进行排序
/// 用于排序的数据块包装器
#[derive(Eq, PartialEq, Debug)]
pub struct SortWrapper {
    pub id: usize,
    pub data: Vec<u8>,
}

impl SortWrapper {
    pub fn new(id: usize, data: Vec<u8>) -> Self {
        Self { id, data }
    }
}

impl Ord for SortWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
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
            data.extend_from_slice(&wrapper.data);
        }
        Ok(data)
    }
}

pub fn get_pb() -> (MultiProgress, ProgressStyle) {
    // TUI 的声明与显示配置
    let mp = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("{prefix} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) ({elapsed}) {msg}")
        .unwrap()
        .progress_chars("#>-");
    (mp, sty)
}

#[test]
fn test_sort() {
    let queue: HeapSortWrapper = Arc::new(Mutex::new(BinaryHeap::new()));

    let list = [
        SortWrapper::new(3, vec![13, 14, 15, 16, 17, 18]),
        SortWrapper::new(2, vec![7, 8, 9, 10, 11, 12]),
        SortWrapper::new(1, vec![1, 2, 3, 4, 5, 6]),
    ];
    for i in list {
        queue.lock().unwrap().push(Reverse(i))
    }

    let data = queue.handle().unwrap();

    assert_eq!(
        data,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,]
    );
}
