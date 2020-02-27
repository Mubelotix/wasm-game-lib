# wasm-game-lib

Welcome!

The goal of this crate is to help you to make great 2d games running in web browsers.
This crate is very similar to SFML.

## How to use

Please follow these instructions to save yourself a lot of pain.
To use this crate, you will need wasm-pack. You can install wasm-pack with:

```
cargo install wasm-pack
```

To create your game crate, the best way is to use:
```
wasm-pack new
```

Then, **put this crate and the wasm-bindgen-futures crate in Cargo.toml** as usual.
Here is the template you may want to use in lib.rs:

```rust
#[allow(unused_imports)]

use wasm_bindgen::{prelude::*, JsCast};
use wasm_game_lib::graphics::image::Image;
use wasm_game_lib::graphics::sprite::Sprite;
use wasm_game_lib::inputs::event::Event;
use wasm_game_lib::graphics::window::Window;
use wasm_game_lib::system::log;
use wasm_game_lib::inputs::event::types::*;
use wasm_game_lib::system::sleep;
use std::time::Duration;
use console_error_panic_hook::set_once;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    set_once(); // needed to see panic messages in the console of your web browser

    let (mut window, mut canvas) = Window::init_with_events(MOUSE_EVENT + KEYBOARD_EVENT + RESIZE_EVENT + FOCUS_EVENT);

    // load images and fonts here
    // you could make a progress bar

    loop {
        for event in window.poll_events() {
            // do something with events
            log(&format!("{:?}", event));
        }

        canvas.clear();
        // canvas.draw(&object);

        sleep(Duration::from_millis(16)).await;
    }

    Ok(())
}
```

You can compile your crate using:
```
wasm-pack build --target=web
```

A folder named pkg has been created in the folder of your project.
Only two files are important: project_name.js and project_name_bg.wasm.

To use these files on a web page, I recommend you to put the file index.html in the pkg folder, with this content:

```html
<html>
<head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
    <style>
        * {
            padding: 0;
            margin: 0;
        }
        canvas {
            width: 100%;
            height: 100%;
        }
    </style>
</head>
<body>
    <script type="module">
        import init from './project_name.js';

        async function run() {
            await init();
        }
        run();
    </script>
</body>
</html>
```

Make sure to modify the js file name in index.html depending on your project name.
Now, you should be able to run your game by opening index.html in a web browser.
In case this does not work, make sure you followed every instruction and [contact me](mailto:mubelotix@gmail.com).

License: MIT
