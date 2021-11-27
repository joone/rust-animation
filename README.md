# Introduction
rust-animation is an OpenGL based graphics library for creating hardware-accelerated user interfaces written in Rust.
It allows us to implement a simple animated UI for embedded devices.

Note: this project is in early development stage so there would be a lot of changes without any notice.

# Installation
rust-animation is written in Rust so you need to install Rust:
* https://www.rust-lang.org/tools/install

If you build rust-animation in Windows, you have to install cmake first.

Note: rust-animation is tested in Ubuntu 20.04, Windows10, and Mac OSX.

There are several examples so you can build them as follows:

```
$ cargo build --example ani
$ cargo build --example picture_viewer
```

# Run
```
$ target/debug/examples/ani
$ target/debug/examples/picture_viewer
```

# Code examples
```
  let mut play = Play::new("Animation test".to_string());
  play.initialize();
  let mut stage = Stage::new("stage".to_string(), 1920, 1080, None);
  stage.set_visible(true);

  let mut actor = Actor::new("actor_1".to_string(), 400, 225, None);
  actor.x = 100;
  actor.y = 100;
  actor.rotation = 5;
  actor.set_image("examples/splash.png".to_string());
  
  // 1X -> 2X for 5 sec.
  actor.apply_scale_animation(1.0, 2.0, 5.0);
  actor.apply_translation_x_animation(100, 600, 5.0);
  actor.apply_translation_y_animation(100, 200, 5.0);

  let mut actor_2 = Play::new_actor("actor_2".to_string(), 120, 120, None);
  actor_2.x = 100;
  actor_2.y = 100;
  actor_2.scale_x = 1.5;
  actor_2.scale_y = 1.5;
  actor_2.set_color(0.0, 0.0, 1.0);
  // 0 degree -> 360 degree for 5 sec
  actor_2.apply_rotation_animation(0, 360, 5.0); 

  stage.add_actor(actor);
  stage.add_actor(actor_2);
  play.add_stage(stage);


```

The main loop of glfw:
```
  while !window.should_close() {
    // events
    process_events(&mut window, &events);

    play.render();

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    window.swap_buffers();
    glfw.poll_events();
  }
```
