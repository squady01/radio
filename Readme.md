# Radio

Radio is a simple program to manage a list of radios and play them with mpv. 

## Prerequisites

* mpv must be installed on your system.

## Features

* Add a radio to the list
* Remove a radio from the list
* Play a radio
* Show the list of radios

## How to use

The following commands are available:

* `add`: add a new radio to the list
  * `<radio_name>`: name of the radio
  * `<radio_stream>`: stream url of the radio
* `del`: remove a radio from the list
  * `<radio_name>`: name of the radio
* `play`: play a radio
  * `<radio_name>`: name of the radio
* `list`: show the list of radios

## Example

```
$ cargo run add my_radio http://example.com/stream
Radio my_radio added
```
```
$ cargo run del my_radio
Radio my_radio removed
```
```
$ cargo run play my_radio
Playing my_radio...
```
```
$ cargo run list
[my_radio => http://example.com/stream]
```
