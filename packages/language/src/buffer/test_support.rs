#![cfg(any(test, feature = "test-support"))]

use super::*;

use ::util::RandomCharIter;

#[cfg(any(test, feature = "test-support"))]
pub struct TestFile {
    pub path: Arc<RelPath>,
    pub root_name: String,
    pub local_root: Option<PathBuf>,
}

#[cfg(any(test, feature = "test-support"))]
impl File for TestFile {
    fn path(&self) -> &Arc<RelPath> { &self.path }

    fn full_path(&self, _: &gpui::App) -> PathBuf {
        PathBuf::from(self.root_name.clone()).join(self.path.as_std_path())
    }

    fn as_local(&self) -> Option<&dyn LocalFile> {
        if self.local_root.is_some() {
            Some(self)
        } else {
            None
        }
    }

    fn disk_state(&self) -> DiskState { unimplemented!() }

    fn file_name<'a>(&'a self, _: &'a gpui::App) -> &'a str {
        self.path().file_name().unwrap_or(self.root_name.as_ref())
    }

    fn worktree_id(&self, _: &App) -> WorktreeId { WorktreeId::from_usize(0) }

    fn to_proto(&self, _: &App) -> rpc::proto::File { unimplemented!() }

    fn is_private(&self) -> bool { false }

    fn path_style(&self, _cx: &App) -> PathStyle { PathStyle::local() }
}

#[cfg(any(test, feature = "test-support"))]
impl LocalFile for TestFile {
    fn abs_path(&self, _cx: &App) -> PathBuf {
        PathBuf::from(self.local_root.as_ref().unwrap())
            .join(&self.root_name)
            .join(self.path.as_std_path())
    }

    fn load(&self, _cx: &App) -> Task<Result<String>> { unimplemented!() }

    fn load_bytes(&self, _cx: &App) -> Task<Result<Vec<u8>>> { unimplemented!() }
}

#[cfg(any(test, feature = "test-support"))]
impl Buffer {
    pub fn edit_via_marked_text(
        &mut self,
        marked_string: &str,
        autoindent_mode: Option<AutoindentMode>,
        cx: &mut Context<Self>,
    ) {
        let edits = self.edits_for_marked_text(marked_string);
        self.edit(edits, autoindent_mode, cx);
    }

    pub fn randomly_edit<T>(&mut self, rng: &mut T, old_range_count: usize, cx: &mut Context<Self>)
    where
        T: rand::Rng,
    {
        let mut edits: Vec<(Range<usize>, String)> = Vec::new();
        let mut last_end = None;
        for _ in 0..old_range_count {
            if last_end.is_some_and(|last_end| last_end >= self.len()) {
                break;
            }

            let new_start = last_end.map_or(0, |last_end| last_end + 1);
            let mut range = self.random_byte_range(new_start, rng);
            if rng.random_bool(0.2) {
                mem::swap(&mut range.start, &mut range.end);
            }
            last_end = Some(range.end);

            let new_text_len = rng.random_range(0..10);
            let mut new_text: String = RandomCharIter::new(&mut *rng).take(new_text_len).collect();
            new_text = new_text.to_uppercase();

            edits.push((range, new_text));
        }
        log::info!("mutating buffer {:?} with {:?}", self.replica_id(), edits);
        self.edit(edits, None, cx);
    }

    pub fn randomly_undo_redo(&mut self, rng: &mut impl rand::Rng, cx: &mut Context<Self>) {
        let was_dirty = self.is_dirty();
        let old_version = self.version.clone();

        let ops = self.text.randomly_undo_redo(rng);
        if !ops.is_empty() {
            for op in ops {
                self.send_operation(Operation::Buffer(op), true, cx);
                self.did_edit(&old_version, was_dirty, BufferEditSource::User, cx);
            }
        }
    }
}
