use libafl::{
    corpus::Corpus,
    mutators::{MutationResult, Mutator},
    state::{HasCorpus, HasRand, State},
    HasMetadata,
};
use libafl_bolts::{AsSlice, Named};
use std::{borrow::Cow, cell::RefCell, collections::VecDeque, marker::PhantomData, rc::Rc};
use thesis::Node;
use thesis::Visitor;

use crate::context::Context;

pub struct ThesisSpliceAppendMutator<I> {
    visitor: Rc<RefCell<Visitor>>,
    phantom: PhantomData<I>,
}

impl<I, S> Mutator<I, S> for ThesisSpliceAppendMutator<I>
where
    I: Node,
    S: State + HasCorpus + HasRand + HasMetadata,
    S::Corpus: Corpus<Input = I>,
{
    fn mutate(&mut self, state: &mut S, input: &mut I) -> Result<MutationResult, libafl::Error> {
        let metadata = state.metadata::<Context>().unwrap();
        input.nodes(&mut self.visitor.borrow_mut(), 0);
        let mut nodes = self.visitor.borrow_mut().nodes();
        let field_splice_index = self.visitor.borrow_mut().random_range(0, nodes.len() - 1);
        let field = &nodes[field_splice_index];
        let ((id, node_ty), ty) = field.last().unwrap();
        if let thesis::NodeType::Iterable(field_len, inner_ty) = node_ty {
            if let Some(possible_splices) = metadata.get_inputs_for_type(&inner_ty) {
                if *field_len > 200 {
                    return Ok(MutationResult::Skipped);
                }
                // calculate subsplice size
                let path = VecDeque::from_iter(field.iter().map(|(i, ty)| i.0));
                let random_splice = possible_splices
                    .get(
                        self.visitor
                            .borrow_mut()
                            .random_range(0, possible_splices.len() - 1),
                    )
                    .unwrap();
                // TODO: cache this in memory
                let data = std::fs::read(random_splice).unwrap();
                #[cfg(debug_assertions)]
                println!("splice | splice_append | {:?}", (&field, &path));
                input.__mutate(
                    &mut thesis::MutationType::SpliceAppend(&mut data.as_slice()),
                    &mut self.visitor.borrow_mut(),
                    path,
                );
            } else {
                return Ok(MutationResult::Skipped);
            }
        }
        Ok(MutationResult::Skipped)
    }

    fn post_exec(
        &mut self,
        _state: &mut S,
        _new_corpus_id: Option<libafl::corpus::CorpusId>,
    ) -> Result<(), libafl::Error> {
        Ok(())
    }
}

impl<I> Named for ThesisSpliceAppendMutator<I> {
    fn name(&self) -> &std::borrow::Cow<'static, str> {
        &Cow::Borrowed("ThesisSpliceAppendMutator")
    }
}
impl<I> ThesisSpliceAppendMutator<I> {
    pub fn new(visitor: Rc<RefCell<Visitor>>) -> Self {
        Self {
            visitor,
            phantom: PhantomData,
        }
    }
}
