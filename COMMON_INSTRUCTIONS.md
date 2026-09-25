# TANTRA — COMMON INSTRUCTIONS

## 1. New Chat: Start

When a new Tantra chat begins and the user writes **“Start”**, follow this order:

1. Inspect the **entire current repository structure** of `darshandpatel63-prog/Tantra.In`.
2. Inspect **all files and all folders**, including nested folders and relevant hidden/configuration files available through the connected GitHub tooling.
3. Do not inspect only a few files and assume the rest.
4. Then read:
   - `TANTRA_BLUEPRINT.md`
   - `HOW_TO_WORK.md`
   - `COMMON_INSTRUCTIONS.md`
   - `AGENTS.md`
   - `HANDOVER.md`
   - the latest relevant file(s) in `handover/`
5. Then read the existing files/code/specifications/tests/workflows relevant to the requested task.
6. Determine the actual current project state and continue from that state.

The user should not have to reconstruct an old chat manually when the repository contains the required information.

## 2. Never Recreate Existing Files or Folders

**Before creating anything, verify the complete repository structure.**

Never create a new file or folder merely because its name is not currently in the assistant's immediate context.

If the location/name is uncertain:

1. inspect the complete repository;
2. search for the file/folder;
3. inspect related folders and existing implementation;
4. only then decide.

Existing work must be reused and extended where appropriate.

## 3. When a New File or Folder Is Allowed

Create a new file/folder only when:

- the required functionality is not already available in the repository; or
- the architecture genuinely requires a new file/folder; or
- adding to the existing file would be inappropriate or unsafe.

If an existing file/folder can correctly support the work, do **not** create a duplicate.

## 4. Existing Work Must Be Read First

Before changing or replacing existing:

- code
- files
- folders
- architecture
- specifications
- tests
- workflows
- configuration

read and understand the relevant existing work first.

Do not rebuild work that already exists.

## 5. Project State

After inspecting the repository and project-control files, determine:

- what is complete;
- what is incomplete;
- what is currently being worked on;
- known bugs/issues;
- current architecture/state;
- what was done in the previous stage;
- what the next correct work item is.

Then continue from the latest documented state.

## 6. Final Rule

**FIRST: inspect the complete repository.  
THEN: read the project-control files.  
THEN: read relevant existing implementation/specifications/tests.  
ONLY THEN: create or modify files/code.**

Never create a file or folder without first verifying that the required file/folder does not already exist.
