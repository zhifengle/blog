use image::ImageOutputFormat;

use crate::pb::Spec;
mod photon;
pub use photon::Photon;

pub trait Engine {
    // 应用 spec
    fn apply(&mut self, specs: &[Spec]);
    // 使用的 self; engine 生成图片
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

// 针对 apply 实现。每个 spec 有一个对于的处理
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
