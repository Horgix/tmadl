# Contributing to TMA;DL

Thanks so much for considering contributing to **TMA;DL**!

I’m always happy to have new people help improve the project, whether it's
through fixing bugs, suggesting improvements, or adding new features. Your
contributions are very much appreciated.

---

## How to Contribute

### 1. Fork the GitHub repository

To contribute, you first need to fork the repository to your GitHub account:

- Go to the TMA;DL GitHub page.
- Click the **Fork** button in the top right corner.

### 2. Clone the git repository of your fork

After forking, clone the repository to your local machine.
You can use either `git` or GitHub's CLI tool:

**Using Git:**

```sh
git clone git@github.com:Horgix/tmadl.git
```

**Using the GitHub CLI:**

```sh
gh repo clone YOUR_USERNAME/tmadl
```

... or just look under the big `Code` button on your fork on GitHub for other options.

### 3. Set up Rust (nightly) toolchain

The project requires the **Rust's nightly toolchain** due to the usage of the
`clap` crate's derive feature (see <https://docs.rs/clap/4.5.34/clap/_features/index.html>).

To set up the project:

1. Install Rust (see [official instructions](https://www.rust-lang.org/tools/install))
2. Set up the nightly toolchain, e.g. when using `rustup` to manage your setup:

```sh
rustup install nightly
rustup override set nightly
```

This ensures the project always uses the nightly version of Rust when you're
working within the `tmadl` directory, while not impacting other projects like
`rustup default nightly` would.

### 4. Build TMA;DL

Navigate to the project directory:

```sh
cd tmadl
```

Build the project to ensure everything is set up correctly:

```sh
cargo build
```

### 5. Make changes

Create a new branch for your work:

```bash
git checkout -b my-new-feature
```

Make your changes. If you're adding a feature or fixing a bug, make sure to write clear and concise commit messages.

TODO: add explanation on project's structure and hexagonal architecture here.

### 6. Push your changes

Push your changes to your forked repository:

```bash
git push origin my-new-feature
```

### 7. Create a Pull Request

- Go to the repository page on GitHub.
- Click on the **Pull Request** button.

Alternatively, you can use the GitHub CLI to create a pull request:

```bash
gh pr create --base main --head YOUR_USERNAME:my-new-feature --fill
```

Provide a clear description of the changes you made and why they are needed.

### 8. Discuss and review

Once the pull request is created, [Horgix](https://github.com/Horgix) will
review your changes. Be prepared to discuss your implementation and make any
necessary changes.

---

## Additional Notes

### Testing

Currently, there are no automated tests in the repository.
If you wish to add tests in the future, feel free to do so, that would be very
welcome!

### License

By contributing, you agree that your contributions will be licensed under the project's **MIT License**.

---

Thanks again for contributing, and happy coding! 🎉
