# lockpick-rs
`lockpick-rs` is tool for managing [lockpick](https://github.com/wnkbll/lockpick).
It ensures that `lockpick` is up-to-date and
provide a control for Windows Service layer.

## Requirements
`lockpick-rs` using `git >=2.49.0` for versioning `lockpick`.
Also, `git` should be added to `PATH` variables.
If you want to build by yourself, `rustc >=1.87.0` is required.

> [!NOTE]
> Lower versions of this dependencies can work but they are not tested.
> Some Rust code marked as unstable in `rustc <1.87.0`,
> so building with versions lower then `1.87.0` requires `+nightly` flag.

## Usage
This section will contain information about using `lockpick-rs` tool.
While there are no releases `Usage` section will only contain information
about building `lockpick-rs`
### How to build

## Roadmap
- [ ] Add feature to clone repository into configurable directory
- [ ] Add feature to set up Windows Service
- [ ] Add feature to install git and/or other dependencies
- [ ] Add feature to build release in full auto mod
- [ ] Add CLI interface to use it with `.bat` scripts 
- [ ] \(Optional\) Add feature to find and check strategies
- [ ] \(Optional\) Add web service to store and share dpi bypass strategies
- [ ] \(Optional\) Add GUI or TUI for management of `lockpick-rs`
