// 注意事项：
// thiserror是给lib使用的，
// 而anyhow是给bin程序使用，
// 当然bin程序可以使用thiserror+anyhow，
// 但是切记anyhow不要在lib里面使用。
pub mod anyhow;
pub mod thiserror;
