---
id: GitSettings
title: GitSettings
---

# Struct: GitSettings

Defined in: [`packages/project/src/project_settings.rs:459`](../../../../packages/project/src/project_settings.rs#L459)

## Definition

```rust
pub struct GitSettings
{
    // Whether or not git integration is enabled. Default: true
    pub enabled: GitEnabledSettings,
    // Whether or not to show the git gutter. Default: tracked_files
    pub git_gutter: GitGutterSetting,
    // Sets the debounce threshold (in milliseconds) after which changes are reflected in the git gutter. Default: 0
    pub gutter_debounce: u64,
    // Whether or not to show git blame data inline in the currently focused line. Default: on
    pub inline_blame: InlineBlameSettings,
    // Git blame settings.
    pub blame: BlameSettings,
    // Which information to show in the branch picker. Default: on
    pub branch_picker: BranchPickerSettings,
    // How hunks are displayed visually in the editor. Default: staged_hollow
    pub hunk_style: GitHunkStyleSetting,
    // Which base git features diff against. Default: head
    pub diff_base: GitDiffBaseSetting,
    // How file paths are displayed in the git gutter. Default: file_name_first
    pub path_style: GitPathStyle,
    // Whether to show the stage and restore buttons on diff hunks. Default: true
    pub show_stage_restore_buttons: bool,
    // Directory where git worktrees are created, relative to the repository working directory. When the resolved directory is outside the project root, the project's directory name is automatically appended so that sibling repos don't collide. Default: ../worktrees
    pub worktree_directory: String,
}
```

## Fields

### enabled

Defined in: [`packages/project/src/project_settings.rs:463`](../../../../packages/project/src/project_settings.rs#L463)

Whether or not git integration is enabled.

Default: true


***

### git_gutter

Defined in: [`packages/project/src/project_settings.rs:467`](../../../../packages/project/src/project_settings.rs#L467)

Whether or not to show the git gutter.

Default: tracked_files


***

### gutter_debounce

Defined in: [`packages/project/src/project_settings.rs:471`](../../../../packages/project/src/project_settings.rs#L471)

Sets the debounce threshold (in milliseconds) after which changes are reflected in the git gutter.

Default: 0


***

### inline_blame

Defined in: [`packages/project/src/project_settings.rs:476`](../../../../packages/project/src/project_settings.rs#L476)

Whether or not to show git blame data inline in
the currently focused line.

Default: on


***

### blame

Defined in: [`packages/project/src/project_settings.rs:478`](../../../../packages/project/src/project_settings.rs#L478)

Git blame settings.


***

### branch_picker

Defined in: [`packages/project/src/project_settings.rs:482`](../../../../packages/project/src/project_settings.rs#L482)

Which information to show in the branch picker.

Default: on


***

### hunk_style

Defined in: [`packages/project/src/project_settings.rs:486`](../../../../packages/project/src/project_settings.rs#L486)

How hunks are displayed visually in the editor.

Default: staged_hollow


***

### diff_base

Defined in: [`packages/project/src/project_settings.rs:490`](../../../../packages/project/src/project_settings.rs#L490)

Which base git features diff against.

Default: head


***

### path_style

Defined in: [`packages/project/src/project_settings.rs:494`](../../../../packages/project/src/project_settings.rs#L494)

How file paths are displayed in the git gutter.

Default: file_name_first


***

### show_stage_restore_buttons

Defined in: [`packages/project/src/project_settings.rs:498`](../../../../packages/project/src/project_settings.rs#L498)

Whether to show the stage and restore buttons on diff hunks.

Default: true


***

### worktree_directory

Defined in: [`packages/project/src/project_settings.rs:505`](../../../../packages/project/src/project_settings.rs#L505)

Directory where git worktrees are created, relative to the repository
working directory. When the resolved directory is outside the project
root, the project's directory name is automatically appended so that
sibling repos don't collide.

Default: ../worktrees

## Implementations

### inline_blame_delay()

```rust
pub fn inline_blame_delay(&self) -> Option<Duration>
```

Defined in: [`packages/project/src/project_settings.rs:592`](../../../../packages/project/src/project_settings.rs#L592)

#### Returns

`Option<Duration>`

## Trait Implementations

- `impl Borrow for GitSettings`
- `impl BorrowMut for GitSettings`
- `impl CloneToUninit for GitSettings`
- `impl Into for GitSettings`
- `impl From for GitSettings`
- `impl TryInto for GitSettings`
- `impl TryFrom for GitSettings`
- `impl Any for GitSettings`
- `impl ToOwned for GitSettings`
- `impl DynClone for GitSettings`
- `impl VZip for GitSettings`
- `impl CastableFrom for GitSettings`
- `impl CastableFrom for GitSettings`
- `impl Read for GitSettings`
- `impl IntoEither for GitSettings`
- `impl ErasedDestructor for GitSettings`
- `impl Same for GitSettings`
- `impl Pointable for GitSettings`
- `impl Instrument for GitSettings`
- `impl WithSubscriber for GitSettings`
- `impl FromAngle for GitSettings`
- `impl IntoAngle for GitSettings`
- `impl IntoCam16Unclamped for GitSettings`
- `impl Cam16IntoUnclamped for GitSettings`
- `impl ArraysFrom for GitSettings`
- `impl ArraysInto for GitSettings`
- `impl ComponentsFrom for GitSettings`
- `impl TryComponentsInto for GitSettings`
- `impl UintsFrom for GitSettings`
- `impl UintsInto for GitSettings`
- `impl AdaptIntoUnclamped for GitSettings`
- `impl AdaptInto for GitSettings`
- `impl IntoColor for GitSettings`
- `impl IntoColorUnclamped for GitSettings`
- `impl TryIntoColor for GitSettings`
- `impl FromStimulus for GitSettings`
- `impl IntoStimulus for GitSettings`
- `impl Clone for GitSettings`
- `impl Debug for GitSettings`

## Auto Trait Implementations

`Freeze` `RefUnwindSafe` `Send` `Sync` `Unpin` `UnsafeUnpin` `UnwindSafe`

