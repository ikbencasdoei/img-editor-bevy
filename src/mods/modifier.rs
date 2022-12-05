use uuid::Uuid;

use crate::prelude::{Image, *};

pub struct Modification {
    pub index: ModifierIndex,
    pub id: Uuid,
    modifier: Box<dyn Modifier + Send + Sync>,
    selection: Vec<Selection>,
    cache: Option<Image>,
}

impl Modification {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Default + Send + Sync + 'static,
    {
        Self {
            index: M::get_index(),
            id: Uuid::new_v4(),
            modifier: Box::new(modifier),
            selection: Vec::new(),
            cache: None,
        }
    }

    pub fn add_selection<S>(&mut self, selection: S)
    where
        S: Selector + Default + Send + Sync + 'static,
    {
        self.selection.push(Selection {
            selector: Box::new(selection),
            index: S::get_index(),
        });
    }

    pub fn apply(&mut self, output: &mut Image) {
        if let Some(cached) = &self.cache {
            *output = cached.clone();
        } else {
            let mut modifier_state = dyn_clone::clone_box(&self.modifier);
            for selection in self.selection.iter() {
                for position in selection.selector.get_pixels(output) {
                    if let Some(color) = modifier_state.get_pixel(position, output) {
                        output.set_pixel(position, color).unwrap();
                    }
                }
            }

            self.cache = Some(output.clone());
        }
    }

    pub fn get_selection(&self) -> &Vec<Selection> {
        &self.selection
    }

    pub fn remove_selection(&mut self, index: usize) {
        self.selection.remove(index);
        self.cache = None;
    }
}
