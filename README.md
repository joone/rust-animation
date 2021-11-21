# rust-animation
OpenGL based Animation Toolkit written in Rust

# examples
```
  let mut play = Play::new("Animation test".to_string());
  play.initialize();
  let mut stage = Stage::new(1920, 1080);

  let mut actor = Actor::new("actor_1".to_string(), 100, 100);
  actor.x = 700;
  actor.y = 100;
  actor.rotation = 45;
  actor.set_color(1.0, 0.0, 0.0);
  actor.apply_scale_animation(1.0, 2.0, 0.01);
  actor.apply_translation_x_animation(700, 1000, 5);

  // Another way to create a Actor
  let mut actor_2 = play.new_actor("actor_2".to_string(), 100, 100);
  actor_2.x = 0;
  actor_2.y = 0;
  actor_2.scale_x = 1.5;
  actor_2.scale_y = 1.5;
  actor_2.set_color(0.0, 0.0, 1.0);
  actor_2.apply_rotation_animation(0, 360, 1);

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
