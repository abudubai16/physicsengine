#[allow(dead_code)]
pub trait BoundingBox {
    // (left, right, top, bottom)
    fn get_bounding_box(&self) -> (f32, f32, f32, f32);

    fn simple_collision(&self, other: &Self) -> bool {
        let (left1, right1, top1, bottom1) = self.get_bounding_box();
        let (left2, right2, top2, bottom2) = other.get_bounding_box();

        !(right1 < left2 || left1 > right2 || bottom1 < top2 || top1 > bottom2)
    }
}
