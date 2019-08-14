<p align="center"><img width=45.5% src="https://github.com/ThomasMeier/factomd-rs/blob/master/doc/media/Factom-Protocol_Logo.svg"></p>

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
[![Build Status](https://travis-ci.org/FactomProject/factomd.svg?branch=develop)](https://travis-ci.org/FactomProject/factomd)
[![Contributions welcome](https://img.shields.io/badge/contributions-welcome-orange.svg)](https://github.com/ThomasMeier/factomd-rewrite/blob/master/CONTRIBUTING.md)
![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-brightgreen.svg)
[![Discord](https://img.shields.io/badge/discord-join%20chat-blue.svg)](https://discord.gg/SxPePjQ)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)



## Basic Overview

The Factom® Protocol is a blockchain utilized by the U.S Department of Homeland Security, the Bill and Melinda Gates Foundation and other leading enterprise. The technology provides high throughput and easy integration into legacy systems, without the need to handle cryptocurrency. The protocol allows fixed costs to accurately budget projects, whilst securing unlimited data via simple API integrations.

### Get the Stable Release

You can get the current release by visiting the Releases page and picking the latest release marked as stable.

### Usage

After getting a copy of `factomd` you can start it to connect to the live test network. At present, we have not launched a main network. To connect to the test network, use the following command:

```
factomd --dev
```

Feel free to check out the other options by using the `--help` flag. Additionally, generate shell completion scripts by using the `completions` option like so: `factomd --completions bash` (also available are zsh, bash, fish, powershell, elvish).

### Building

See [`docs/building-factom.md`](https://github.com/ThomasMeier/factom-rewrite/blob/master/doc/building-factom.md) to see how you can build `factomd`.

### Contributing

Thank you for considering contributing! We welcome a wide range of contributions. Please see the [CONTRIBUTING](https://github.com/ThomasMeier/factom-rewrite/blob/master/CONTRIBUTING.md) guide to see how you can help out. 
