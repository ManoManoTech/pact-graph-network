
# Pact graph network
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![build status](https://github.com/ManoManoTech/pact-graph-network/workflows/CI/badge.svg)](https://github.com/ManoManoTech/pact-graph-network/actions)
[![rust](https://img.shields.io/badge/rust-FA7343?logo=rust&logoColor=white)](https://www.rust-lang.org/)


Generates a schema of dependencies between microservices using pact-broker data.


# Table of contents

- [Screenshots](#screenshots)
- [Tech Stack](#tech-stack)
- [Features](#features)
- [How to install](#how-to-install)
- [Usage](#usage)
- [Environment Variables](#environment-variables)
- [Feedback](#feedback)
- [License](#license)

## Screenshots

<div>
  <img src="docs/edge-bundling.png" width="280" />
  <img src="docs/force-directed.png" width="280" />
</div>

## Tech Stack

This project is created with:

- [rust](https://www.rust-lang.org/) 🦀
- [vega](https://vega.github.io/vega/)

## Features

- [x] generate an html report
- [x] generate an edge bundling chart
- [x] generate a force directed layout chart

## How to install

```bash
# Download the binary
VERSION=0.1.0-alpha.2
curl -L -o /usr/local/bin/pact-graph-network \
  https://github.com/ManoManoTech/pact-graph-network/releases/download/v${VERSION}/pact-graph-network_x86_64-darwin

# Make it executable
chmod a+x /usr/local/bin/pact-graph-network

# And run it
pact-graph-network \
  --url http://your.pact.broker \
  --output ./report
```

## Usage



~~~bash
  pact-graph-network --url https://pact-brocker.your.com/ --output report
~~~

## Environment Variables

| Key                        | Description                                   | Alowed values               |
| -------------------------- | --------------------------------------------- | --------------------------- |
| **PACT_NETWORK_LOG**       | Adds filters to the logger.                   | error,warn,info,debug,trace |
| **PACT_NETWORK_LOG_STYLE** | Whether or not to print styles to the target. | auto, always, never         |


## Feedback

If you have any feedback, please open an issue.

## License

[MIT](https://choosealicense.com/licenses/mit/)
