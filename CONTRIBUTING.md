# Contributing to Rust Practice

Thank you for considering contributing to Rust Practice! To ensure a smooth and collaborative process, please take a moment to review the following guidelines.

## Branch Structure and Rules

We use multiple branches for different purposes. Here’s a breakdown of the branch structure:

1. **Main Branches:**
   - **main**: This branch contains the stable version of the repository.
   - **main-dev**: Development for features and updates related to the solution branch.

2. **Practice Branches:**
   - **practice**: Contains exercises for users to practice. This is a stable branch.
   - **practice-dev**: Development for the practice branch.

### General Rules:
- **Never push directly to the `main` or `practice` branches**. All work should be done in the respective `dev` branches.
- Always submit your changes via a Pull Request (PR) to the relevant `dev` branch (either `main-dev` or `practice-dev`).
- Your PR will be reviewed, and if it follows the guidelines, it will be merged into the appropriate branch.

## Pull Request Guidelines

Before creating a PR, ensure the following:

1. **Create an Issue First**: 
   - For new features or bug fixes, please open an issue first to discuss your proposed changes.
   - Reference the issue number in your PR (e.g., `Fixes #issue_number`).

2. **Update Your Branch**:
   - Make sure your branch is up-to-date with the latest changes from the target `dev` branch (either `main-dev` or `practice-dev`).
   - Pull the latest commits before submitting your PR:
     ```bash
     git pull origin main-dev
     # or
     git pull origin practice-dev
     ```

3. **Follow the PR Template**:
   - When creating a PR, fill out the [Pull Request Template](PULL_REQUEST_TEMPLATE.md).
   - Make sure your PR provides a clear and concise explanation of what you are adding or fixing.

4. **Review and Testing**:
   - Ensure that all tests pass on your local machine.
   - If applicable, add or update any tests related to your changes.

5. **No Rules, No PR**:
   - PRs that do not follow the guidelines will be marked as invalid and closed immediately.

## Issue Reporting Guidelines

When reporting bugs or requesting features, please:

1. **Check Existing Issues**:
   - Before creating a new issue, search the issue tracker to see if the issue has already been reported or is being worked on.

2. **Follow the Issue Template**:
   - When opening a new issue, please follow the [Issue Template](ISSUE_TEMPLATE.md) provided in the repository.

## Submitting Code

1. **Commit Messages**:
   - Use meaningful commit messages that describe the purpose of each change. Example:
   - If you are confused then look into the commit history, and you will get an idea of how to write a good commit message.

2. **Test Your Changes**:
   - Before submitting a PR, ensure that all changes are thoroughly tested.

3. **Keep Your PR Small**:
   - If you are adding multiple features, break them into separate PRs.

## Review Process

- After submitting a PR, the repository maintainers will review your changes.
- If changes are requested, please update your PR promptly.
- Only after all rules are met will the PR be merged.
