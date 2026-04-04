use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Clone, Copy)]
pub enum UiElementSize {
    #[default]
    Small,
    Medium,
    Large
}

impl UiElementSize {
    pub fn index(self) -> usize {
        match self {
            UiElementSize::Small => 0,
            UiElementSize::Medium => 1,
            UiElementSize::Large => 2,
        }
    }
}

pub struct UiElementSizeHeights {
    pub sizes: [f32; 3],
}

impl Index<UiElementSize> for UiElementSizeHeights {
    type Output = f32;
     
    fn index(&self, layer: UiElementSize) -> &Self::Output {
        &self.sizes[layer.index()] 
    } 
}

impl IndexMut<UiElementSize> for UiElementSizeHeights {
    fn index_mut(&mut self, layer: UiElementSize) -> &mut Self::Output {
        &mut self.sizes[layer.index()] 
    }
}

impl UiElementSizeHeights {
    pub fn new(small: f32, medium: f32, large: f32) -> Self {
        Self {
            sizes: [small, medium, large],
        }
    }
    pub fn small(&self) -> f32 { self[UiElementSize::Small] }
    pub fn medium(&self) -> f32 { self[UiElementSize::Medium] }
    pub fn large(&self) -> f32 { self[UiElementSize::Large] }
}
