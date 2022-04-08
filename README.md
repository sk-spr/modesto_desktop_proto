# modesto_desktop_proto
The Modesto Desktop environment; a prototype Desktop that will be the main shell for PlaygroundOS
## Plans
The Modesto Desktop is intended to mimic the aesthetics of classic macOS. It uses a widgets-all-the-way-down approach with recursive rendering and (future) layouting.
## Current tech
This prototype is designed to run on linux machines through the use of the minifb crate. It is only using software rendering currently, as I don't know what the playgroundOS graphics driver will be capable of. 
## Beginner warning
The creator and, as of now, sole maintainer of this project is not a rust expert. Please forgive any non-idiomatic code, bad practices, et cetera. If you are interested in improving the project, feel free to contribute. If you would like to contact the maintainer personally to suggest changes without implementing them, feel free to shoot an email to skye.sprung@gmail.com 

## How to run
```
cd modesto_desktop_proto
cargo run --release
```
