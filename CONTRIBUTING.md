# Contributing to Traveling Salesman Project

We welcome contributions from the community to improve the *Traveling Salesman Project*. This document outlines the guidelines for contributing, including how to submit bug reports, feature requests, and code changes.

## How to Contribute

### 1. Reporting Issues

If you encounter a bug or have a question about the project, please open an issue on GitHub. Before submitting, please check if a similar issue already exists to avoid duplicates.

- **Bug Reports:** Provide as much detail as possible, including steps to reproduce the issue, your environment (OS, Rust version, etc.), and any relevant error messages.
- **Feature Requests:** Clearly describe the feature you would like to see, including its purpose and how it would benefit the project.

### 2. Code Contributions

We welcome code contributions in the form of bug fixes, new features, and improvements. To contribute code, follow these steps:

#### Step 1: Fork the Repository

Fork the project repository by clicking the "Fork" button at the top of the GitHub page. This will create a copy of the repository in your GitHub account.

#### Step 2: Clone Your Fork

Clone the forked repository to your local machine:

```sh
git clone https://github.com/your-username/traveling_salesman_project.git
cd traveling_salesman_project
```

#### Step 3: Create a New Branch

Create a new branch to work on your changes. Use a descriptive name for your branch:

```sh
git checkout -b feature-your-feature-name
```

#### Step 4: Make Your Changes

Make your changes to the codebase. Follow the existing coding style and ensure that your code is well-documented. 

#### Step 5: Test Your Changes

Before submitting your changes, ensure that everything works correctly:

- **Run Tests:** Run any existing tests to make sure your changes do not break the project.
- **Add New Tests:** If you’ve added new functionality, add corresponding tests to cover your changes.

#### Step 6: Commit Your Changes

Commit your changes with a clear and descriptive commit message:

```sh
git add .
git commit -m "Add feature: Your feature description"
```

#### Step 7: Push Your Changes

Push your changes to your forked repository:

```sh
git push origin feature-your-feature-name
```

#### Step 8: Submit a Pull Request

Go to the original repository on GitHub and submit a pull request (PR) from your forked repository. Provide a detailed description of your changes, including why the changes are necessary and any relevant issues they address.

### 3. Code Style Guidelines

To maintain code consistency, please adhere to the following guidelines:

- **Rust Code:** Follow the Rust API guidelines and the official Rust style guide.
- **Python Code:** Follow PEP 8 for Python code.
- **Documentation:** Write clear and concise documentation for any new functionality, ensuring that it is easy for others to understand.

### 4. Reviewing Pull Requests

All pull requests will be reviewed by the maintainers before being merged into the main branch. The review process includes checking for:

- **Code Quality:** Ensure that the code is clean, well-organized, and follows the project’s coding standards.
- **Correctness:** Verify that the changes function as expected and do not introduce new bugs.
- **Tests:** Confirm that adequate tests have been added and that all tests pass.

### 5. Adding New Features

When adding a new feature, please ensure that it aligns with the project’s goals and provides a clear benefit. Major new features should be discussed with the maintainers via an issue or discussion thread before development begins.

### 6. Documentation Contributions

We also welcome contributions to improve the project’s documentation. This includes fixing typos, clarifying explanations, and adding new content to help users and contributors.

## License

By contributing to the *Traveling Salesman Project*, you agree that your contributions will be licensed under the MIT License.

## Conclusion

Thank you for considering contributing to the *Traveling Salesman Project*. We appreciate your time and effort in improving this project. If you have any questions or need further guidance, feel free to reach out through GitHub issues or discussions.
