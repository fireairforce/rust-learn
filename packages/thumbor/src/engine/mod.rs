use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

// Engine trait: 用于未来图片引擎的替换
pub trait Engine {
    // 对 engine 按照 specs 进行一系列有序处理
    fn apply(&mut self, specs: &[Spec]);
    // 从 engine 中生成目标图片，这里用 self 而不是 self 的引用
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// SpecTransform: 未来添加 spec，只需要实现就行
pub trait SpecTransform<T> {
    // 对图片使用 op 做 transform
    fn transform(&mut self, op: T);
}
