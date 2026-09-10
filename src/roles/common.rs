//! RTIC-ignorant types shared by role logic — no dependency on `rtic`,
//! unit-testable off target.

pub struct CaptureBuffer {
    edges: u8,
}

impl CaptureBuffer {
    pub const fn new() -> Self {
        Self { edges: 0 }
    }

    pub fn push_edge(&mut self) {
        self.edges = self.edges.wrapping_add(1);
    }

    pub fn try_decode(&mut self) -> Option<RcFrame> {
        if self.edges >= 8 {
            self.edges = 0;
            Some(RcFrame(1))
        } else {
            None
        }
    }
}

pub struct RcFrame(pub u32);
