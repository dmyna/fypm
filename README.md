# Four Years Productivity Manager

Fypm (*Four Years Productivity Manager*) is the productivity manager for Four Years Workflow. It is based on [Taskwarrior]() and aims to create a flexible and extensible productivity environment.


## Installation
```bash
git clone https://github.com/dmyna/fypm.git
cd fypm
cargo build --release
sudo cp target/release/fypm /usr/bin
```

## Documentation
All documentation is written in an Obsidian vault and stored in the [docs](https://github.com/dmyna/fypm/tree/develop/docs/fypm) directory. In the future you will also see this on this project's Wiki.


## Testing
```bash
git clone https://github.com/dmyna/fypm.git
cd fypm
sudo docker-compose up -d
./scripts/test.sh
```