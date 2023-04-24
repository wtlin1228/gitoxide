use std::path::PathBuf;

use bstr::ByteSlice;
use gix_glob::pattern::Case;

use crate::{cache::State, PathIdMapping};

type AttributeMatchGroup = gix_attributes::Search;
type IgnoreMatchGroup = gix_ignore::Search;

/// State related to attributes associated with files in the repository.
#[derive(Default, Clone)]
pub struct Attributes {
    /// Attribute patterns which aren't tied to the repository root, hence are global, they contribute first.
    globals: AttributeMatchGroup,
    /// Attribute patterns that match the currently set directory (in the stack).
    ///
    /// Note that the root-level file is always loaded, if present, followed by, the `$GIT_DIR/info/attributes`, if present, based
    /// on the location of the `info_attributes` file.
    stack: AttributeMatchGroup,
    /// The first time we push the root, we have to load additional information from this file if it exists along with the root attributes
    /// file if possible, and keep them there throughout.
    info_attributes: Option<PathBuf>,
    /// A lookup table to accelerate searches.
    collection: gix_attributes::search::MetadataCollection,
    /// The case to use when matching directories as they are pushed onto the stack. We run them against the exclude engine
    /// to know if an entire path can be ignored as a parent directory is ignored.
    case: Case,
    /// Where to read `.gitattributes` data from.
    source: attributes::Source,
}

///
pub mod attributes;
mod ignore;
pub use ignore::Ignore;

/// Initialization
impl State {
    /// Configure a state to be suitable for checking out files, which only needs access to attribute files read from the index.
    pub fn for_checkout(unlink_on_collision: bool, attributes: Attributes) -> Self {
        State::CreateDirectoryAndAttributesStack {
            unlink_on_collision,
            #[cfg(debug_assertions)]
            test_mkdir_calls: 0,
            attributes,
        }
    }

    /// Configure a state for adding files, with support for ignore files and attribute files.
    pub fn for_add(attributes: Attributes, ignore: Ignore) -> Self {
        State::AttributesAndIgnoreStack { attributes, ignore }
    }

    /// Configure a state for status retrieval, which needs access to ignore files only.
    pub fn for_status(ignore: Ignore) -> Self {
        State::IgnoreStack(ignore)
    }
}

/// Utilities
impl State {
    /// Returns a vec of tuples of relative index paths along with the best usable blob OID for
    /// either *ignore* or *attribute* files or both. This allows files to be accessed directly from
    /// the object database without the need for a worktree checkout.
    ///
    /// Note that this method…
    /// - ignores entries which aren't blobs.
    /// - ignores ignore entries which are not skip-worktree.
    /// - within merges, picks 'our' stage both for *ignore* and *attribute* files.
    ///
    /// * `index` is where we look for suitable files by path in order to obtain their blob hash.
    /// * `paths` is the indices storage backend for paths.
    /// * `case` determines if the search for files should be case-sensitive or not.
    pub fn id_mappings_from_index(
        &self,
        index: &gix_index::State,
        paths: &gix_index::PathStorageRef,
        case: Case,
    ) -> Vec<PathIdMapping> {
        let a1_backing;
        let a2_backing;
        let names = match self {
            State::IgnoreStack(v) => {
                a1_backing = [(v.exclude_file_name_for_directories.as_bytes().as_bstr(), true)];
                a1_backing.as_ref()
            }
            State::AttributesAndIgnoreStack { ignore, .. } => {
                a2_backing = [
                    (ignore.exclude_file_name_for_directories.as_bytes().as_bstr(), true),
                    (".gitattributes".into(), false),
                ];
                a2_backing.as_ref()
            }
            State::CreateDirectoryAndAttributesStack { .. } => {
                a1_backing = [(".gitattributes".into(), true)];
                a1_backing.as_ref()
            }
        };

        index
            .entries()
            .iter()
            .filter_map(move |entry| {
                let path = entry.path_in(paths);

                // Stage 0 means there is no merge going on, stage 2 means it's 'our' side of the merge, but then
                // there won't be a stage 0.
                if entry.mode == gix_index::entry::Mode::FILE && (entry.stage() == 0 || entry.stage() == 2) {
                    let basename = path
                        .rfind_byte(b'/')
                        .map(|pos| path[pos + 1..].as_bstr())
                        .unwrap_or(path);
                    let is_ignore = names.iter().find_map(|t| {
                        match case {
                            Case::Sensitive => basename == t.0,
                            Case::Fold => basename.eq_ignore_ascii_case(t.0),
                        }
                        .then_some(t.1)
                    })?;
                    // See https://github.com/git/git/blob/master/dir.c#L912:L912
                    if is_ignore && !entry.flags.contains(gix_index::entry::Flags::SKIP_WORKTREE) {
                        return None;
                    }
                    Some((path.to_owned(), entry.id))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn ignore_or_panic(&self) -> &Ignore {
        match self {
            State::IgnoreStack(v) => v,
            State::AttributesAndIgnoreStack { ignore, .. } => ignore,
            State::CreateDirectoryAndAttributesStack { .. } => {
                unreachable!("BUG: must not try to check excludes without it being setup")
            }
        }
    }

    pub(crate) fn attributes_or_panic(&self) -> &Attributes {
        match self {
            State::AttributesAndIgnoreStack { attributes, .. }
            | State::CreateDirectoryAndAttributesStack { attributes, .. } => attributes,
            State::IgnoreStack(_) => {
                unreachable!("BUG: must not try to check excludes without it being setup")
            }
        }
    }
}