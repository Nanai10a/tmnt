# tmnt

**t**er**m**i**n**a**t**e your heart, hold your breath.

## about

This repository is automated by GitHub Actions. All processes shouldn't be work by yourself.\
Actions has 2 processes: build, and execute.

build process ( `build.yml` , `name: "build executable"` ) is build this cargo project and cache ( `actions/cache` ).

execute process ( `execute.yml` , `name: "execute"` ) is execute this project's executable **from cache (prebuilt)**.

Processes are automated on **11:45** (JST) of build process, and **12:00** (JST) of execute process.\
2 processes has 15 minutes interval, can hold _clean_ (successfully build) states.

## development / manually execute

this project has cargo project, so always like you used to:

```sh
git clone https://github.com/Nanai10a/tmnt.git
cd tmnt

# check: highly recommended use `clippy` alternate `check`
cargo clippy

# build
cargo build

# execute: needs environment values
export SECRET_TOKEN='<Discord Bot Token>'
export SECRET_URL='<Discord Webhook URL>'
cargo run
```

## misc

This automations aren't work after *my 2022 summer vacation* .\
When I develop this project, I feel like I'm *holding my breath* , seriously. ahhhh
