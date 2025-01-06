use std::{collections::VecDeque, fmt::Debug};

use crate::{deserialize, serialize, MutationType, Node, Visitor};

#[cfg(feature = "bincode")]
macro_rules! impl_node_serde_array {
    ($n: literal) => {
        #[cfg(feature = "bincode")]
        impl<T> Node for [T; $n]
        where
            // TODO can we remove the debug clause?
            T: Node + Debug,
        {
            fn generate(visitor: &mut Visitor, depth: &mut usize, cur_depth: &mut usize) -> Self {
                // TODO: optimize?
                (0..$n)
                    .map(|_| T::generate(visitor, depth, cur_depth))
                    .collect::<Vec<T>>()
                    .try_into()
                    .expect("invariant;")
            }

            #[cfg(debug_assertions)]
            fn serialized(&self) -> Option<Vec<(Vec<u8>, String)>> {
                let mut vector = self
                    .iter()
                    .map(|i| (serialize(i), T::id()))
                    .collect::<Vec<_>>();
                for item in self.iter() {
                    if let Some(inner) = item.serialized() {
                        vector.extend(inner)
                    }
                }
                Some(vector)
            }

            fn __len(&self) -> usize {
                $n
            }

            #[cfg(not(debug_assertions))]
            fn serialized(&self) -> Option<Vec<(Vec<u8>, u128)>> {
                let mut vector = self
                    .iter()
                    .map(|i| (serialize(i), T::id()))
                    .collect::<Vec<_>>();
                for item in self.iter() {
                    if let Some(inner) = item.serialized() {
                        vector.extend(inner)
                    }
                }
                Some(vector)
            }
            fn __mutate(
                &mut self,
                ty: &mut MutationType,
                visitor: &mut Visitor,
                mut path: VecDeque<usize>,
            ) {
                if let Some(popped) = path.pop_front() {
                    self.get_mut(popped)
                        .expect("mdNWnhI6____")
                        .__mutate(ty, visitor, path);
                } else {
                    match ty {
                        MutationType::Splice(other) => {
                            *self = deserialize(other);
                        }
                        MutationType::GenerateReplace(ref mut bias) => {
                            *self = Self::generate(visitor, bias, &mut 0)
                        }
                        _ => {
                            // TODO: FIX: cause our length is fixed, we cannot append but we cannot be unreachable
                            // since we are recursive, we may still get called
                        }
                    }
                }
            }
            fn nodes(&self, visitor: &mut Visitor, index: usize) {
                for (index, child) in self.iter().enumerate() {
                    visitor
                        .register_field_stack((((index, crate::NodeType::NonRecursive)), T::id()));
                    child.nodes(visitor, 0);
                    visitor.pop_field();
                }
            }

            fn cmps(&self, visitor: &mut Visitor, index: usize, val: (u64, u64)) {
                for (index, child) in self.iter().enumerate() {
                    visitor
                        .register_field_stack((((index, crate::NodeType::NonRecursive)), T::id()));
                    child.cmps(visitor, index, val);
                    visitor.pop_field();
                }
            }
        }
    };
}

#[cfg(feature = "bincode")]
impl_node_serde_array!(1usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(2usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(3usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(4usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(5usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(6usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(7usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(8usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(9usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(10usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(11usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(12usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(13usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(14usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(15usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(16usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(17usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(18usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(19usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(20usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(21usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(22usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(23usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(24usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(25usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(26usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(27usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(28usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(29usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(30usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(31usize);
#[cfg(feature = "bincode")]
impl_node_serde_array!(32usize);
