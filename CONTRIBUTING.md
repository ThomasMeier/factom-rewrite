# Contributing

Thank you for considering making a contribution to the development of Factom! We welcome contributions of all kinds:

* New code
* Bug fixes
* Code reviews
* Issue discussion and planning

We use Github Issues and Projects as our central source of information on development. Search the issues to find if any interest you, or file a new issue to begin a discussion or offer new code.

**Please report responsibly on Factomize Forums if you find a security concern!**

## Workflow

Create a fork of this repository and continue with these steps:

1. Find or open issue related to your concern
2. If you want assignment to work on the issue, contact the maintainer (Thomas Meier). This is an important step to avoid work being duplicated!
3. Create a new branch off of `master` and apply your changes.
4. Open a pull request targetting the `master` branch. (See Branching below for more information).

From there other core developers will review, maybe have questions, or offer stylistic changes. Once accepted, your work will be merged for the following biweekly release cycle.

### Branching

* Master is the working branch
* Stable is updated to the last stable build of factomd
* When working on an issue tagged as a bug, use this `bug-ID[/optional description]` where `ID` is the github issue number; and the `[/optional description]` helps add context. For example `bug-13/fix-typo-in-readme`.
* When working on an issue tagged as an enhancement, use this `feature-ID[/optional description]` where `ID` is the github issue number; and the `[/optional description]` helps add context. For example `feature-14/add-service-wrapper`.

### Other

* Please ensure you are writing [`rustdoc`](https://doc.rust-lang.org/1.30.0/book/first-edition/documentation.html) comments to your code.
* Also consider using a tool like [`Clippy`](https://github.com/rust-lang/rust-clippy) to find common code issues.