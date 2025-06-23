use std::borrow::Cow;

use crate::{UnityVersion, objects::ClassId, tpk::TpkTypeTreeBlob};

use super::TypeTreeNode;

pub trait TypeTreeProvider {
    fn get_typetree_node(
        &self,
        class_id: ClassId,
        target_version: UnityVersion,
    ) -> Option<Cow<'_, TypeTreeNode>>;
}

impl TypeTreeProvider for TpkTypeTreeBlob {
    fn get_typetree_node(
        &self,
        class_id: ClassId,
        target_version: UnityVersion,
    ) -> Option<Cow<'_, TypeTreeNode>> {
        TpkTypeTreeBlob::get_typetree_node(self, class_id, target_version).map(Cow::Owned)
    }
}
impl<T: TypeTreeProvider> TypeTreeProvider for &T {
    fn get_typetree_node(
        &self,
        class_id: ClassId,
        target_version: UnityVersion,
    ) -> Option<Cow<'_, TypeTreeNode>> {
        (*self).get_typetree_node(class_id, target_version)
    }
}
