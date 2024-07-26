use std::sync::{Arc, Mutex};
use std::{thread, thread::sleep, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressIterator, ProgressStyle};

fn update_pb(pb: &ProgressBar) {
    for _ in 0..100 {
        sleep(Duration::from_millis(50));
        pb.inc(1);
        pb.tick()
    }
}

pub fn base_bar() {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40} {percent}%")
            .unwrap()
            .progress_chars("#>-"),
    );
    update_pb(&pb);
    pb.finish_with_message("done");
}

// 主线程输出多进度条,一个渲染完后才渲染另一个
pub fn multi_bar() {
    let mp = MultiProgress::new();
    let psy = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}",
    )
    .unwrap();

    for _ in 0..2 {
        let pb = mp.add(ProgressBar::new(100));
        pb.set_style(psy.clone());
        let pd_clone = pb.clone();
        update_pb(&pd_clone);
        pb.finish_with_message("done");
    }
}

// 多线程中使用
pub fn multi_progress() {
    let mp = MultiProgress::new();
    let psy = ProgressStyle::with_template(
        "{spinner:.red} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}",
    )
    .unwrap();

    let mut handles = Vec::new();
    for _ in 0..2 {
        let pb = Arc::new(Mutex::new(mp.add(ProgressBar::new(100))));
        pb.lock().unwrap().set_style(psy.clone());
        let pd_clone = pb.clone();

        handles.push(thread::spawn(move || {
            update_pb(&pd_clone.lock().unwrap());
            pd_clone.lock().unwrap().finish_with_message("done");
        }));
    }
    // 等待所有子线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}

// 使用方式类似 python 的 `tqdm` 库, 不过可以指定进度条样式
pub fn dyn_iter() {
    let items = [1, 2, 3, 4, 5];
    let psy = ProgressStyle::with_template("{bar:40.cyan/blue} {percent:>3}% | {pos}/{len}")
        .unwrap()
        .progress_chars("##-");

    for item in items.iter().progress_with_style(psy) {
        println!("{item}");
        sleep(Duration::from_secs(1));
    }
}

pub fn spinner() {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(50));

    sleep(Duration::from_secs(5));

    spinner.finish_with_message("done");
}

// 自定义`spinner`的样式
pub fn spinner_custom_bar() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Loading...");
    sleep(Duration::from_secs(5));
    spinner.finish_with_message("done");
}

#[cfg(test)]
mod test_indicatif {
    use super::*;

    #[test]
    fn test_base_bar() {
        base_bar()
    }

    #[test]
    fn test_multi_bar() {
        multi_bar()
    }

    #[test]
    fn test_multi_progress() {
        multi_progress()
    }

    #[test]
    fn test_dyn_iter() {
        dyn_iter()
    }

    #[test]
    fn test_spinner() {
        spinner()
    }

    #[test]
    fn test_custom_spinner() {
        spinner_custom_bar()
    }
}
