# Purpose
This is a Rust starter repository, this branch shows a pretty basic REST api done with rocket. You can play with the api right in vscode using the `examples.rest` file if you have [this REST client plugin](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) installed 

## Setup Steps
1. Follow steps from the `master` banch README.md
2. Set Rust to nightly `rustup default nightly`

At this point you are all set to be able to debug! Either navigate to the debug panel in vscode and press the play button at the top. Or press F5 to compile and run.

Try setting a breakpoint just inside the `list` and navigating to http://localhost:8000/Matthew 

Check out the [Branches](#branches) section to see examples and setups doing different things

# TODO
 * Make The add api accept a json object for the todo

# Branches
 * master - Helps set up the most basic rust project with no special cargo dependencies
 * *rocket-REST* - This Banch, Shows a basic hello world REST api done with Rocket
