use core::hash::{Hash, Hasher};
#[cfg(not(feature = "std"))]
use hashbrown::hash_map::Entry;
use ordered_float::OrderedFloat;
#[cfg(feature = "std")]
use std::collections::hash_map::Entry;
use std::hash::DefaultHasher;

use crate::{Font, HashMap};

/// Key for caching shape plans.
#[derive(Debug, Hash, PartialEq, Eq)]
struct ShapePlanKey {
    shape_plan_hash: u64,
}

impl ShapePlanKey {
    pub fn new(font: &Font, buffer: &rustybuzz::UnicodeBuffer) -> Self {
        let mut hasher = DefaultHasher::new();

        font.id().hash(&mut hasher);
        buffer.direction().hash(&mut hasher);
        buffer.script().hash(&mut hasher);
        buffer.language().hash(&mut hasher);
        for var in font.variations() {
            var.tag.hash(&mut hasher);
            OrderedFloat(var.value).hash(&mut hasher);
        }

        Self {
            shape_plan_hash: hasher.finish(),
        }
    }
}

/// A helper structure for caching rustybuzz shape plans.
#[derive(Default)]
pub struct ShapePlanCache(HashMap<ShapePlanKey, rustybuzz::ShapePlan>);

impl ShapePlanCache {
    pub fn get(&mut self, font: &Font, buffer: &rustybuzz::UnicodeBuffer) -> &rustybuzz::ShapePlan {
        let key = ShapePlanKey::new(font, buffer);

        match self.0.entry(key) {
            Entry::Occupied(occ) => occ.into_mut(),
            Entry::Vacant(vac) => {
                let plan = rustybuzz::ShapePlan::new(
                    font.rustybuzz(),
                    buffer.direction(),
                    Some(buffer.script()),
                    buffer.language().as_ref(),
                    &[],
                );
                vac.insert(plan)
            }
        }
    }
}

impl core::fmt::Debug for ShapePlanCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ShapePlanCache").finish()
    }
}
