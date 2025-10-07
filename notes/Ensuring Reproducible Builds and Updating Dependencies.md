Links to the resource
[Ensuring Reproducible Builds with the _Cargo.lock_ File](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#ensuring-reproducible-builds-with-the-cargolock-file)
[Updating a Crate to Get a New Version](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#updating-a-crate-to-get-a-new-version)


Cargo.lock is a file which contains all the information of the dependencies when we build the project for the first time

- therefore it always takes and works with the versions that we have mentioned and not take any updates versions
- Therefore we can reproduce the same build automatically

Only if we change the versions in the Cargo.toml file , the Cargo.lock will get updated according to the new version which is added


## Updating

```bash
cargo update
```

this will update the dependencies to its latest version but still following the rule of the SemVer mentioned in the Cargo.toml file

After running this , we will get an updated Cargo.lock