use crate::actor::Actor;
use keyframe::{ease, functions::*};
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub enum EasingFunction {
  EaseIn,
  EaseInCubic,
  EaseInOut,
  EaseInOutCubic,
  EaseInOutQuad,
  EaseInOutQuart,
  EaseInOutQuint,
  EaseInQuad,
  EaseInQuart,
  EaseInQuint,
  EaseOut,
  EaseOutCubic,
  EaseOutQuad,
  EaseOutQuart,
  EaseOutQuint,
  Linear,
  Step,
}

// CoreAnimation-style timing function (alias for EasingFunction)
pub type CAMediaTimingFunction = EasingFunction;

pub struct Animation {
  animation_time_instance: Instant,
  translation_x_running: bool,
  translation_x_starting_time: u128,
  translation_x_time_duration: f32,
  translation_x_from_value: i32,
  translation_x_to_value: i32,
  translation_x_ease: EasingFunction,

  translation_y_running: bool,
  translation_y_starting_time: u128,
  translation_y_time_duration: f32,
  translation_y_from_value: i32,
  translation_y_to_value: i32,
  translation_y_ease: EasingFunction,

  scale_running: bool,
  scale_starting_time: u128,
  scale_time_duration: f32,
  scale_from_value: f32,
  scale_to_value: f32,
  scale_ease: EasingFunction,

  rotation_running: bool,
  rotation_starting_time: u128,
  rotation_time_duration: f32,
  rotation_from_value: i32,
  rotation_to_value: i32,
  rotation_ease: EasingFunction,

  opacity_running: bool,
  opacity_starting_time: u128,
  opacity_time_duration: f32,
  opacity_from_value: f32,
  opacity_to_value: f32,
  opacity_ease: EasingFunction,

  // CoreAnimation-style properties
  pub duration: f32,
  pub timing_function: Option<EasingFunction>,
  pub repeat_count: f32,
  pub autoreverses: bool,
}

impl Animation {
  pub fn new() -> Animation {
    Animation {
      animation_time_instance: Instant::now(),
      translation_x_running: false,
      translation_x_starting_time: 0,
      translation_x_time_duration: 0.0,
      translation_x_from_value: 0,
      translation_x_to_value: 0,
      translation_x_ease: EasingFunction::Linear,

      translation_y_running: false,
      translation_y_starting_time: 0,
      translation_y_time_duration: 0.0,
      translation_y_from_value: 0,
      translation_y_to_value: 0,
      translation_y_ease: EasingFunction::Linear,

      scale_running: false,
      scale_starting_time: 0,
      scale_time_duration: 0.0,
      scale_from_value: 0.0,
      scale_to_value: 0.0,
      scale_ease: EasingFunction::Linear,

      rotation_running: false,
      rotation_starting_time: 0,
      rotation_time_duration: 0.0,
      rotation_from_value: 0,
      rotation_to_value: 0,
      rotation_ease: EasingFunction::Linear,

      opacity_running: false,
      opacity_starting_time: 0,
      opacity_time_duration: 0.0,
      opacity_from_value: 0.0,
      opacity_to_value: 0.0,
      opacity_ease: EasingFunction::Linear,

      duration: 0.0,
      timing_function: None,
      repeat_count: 0.0,
      autoreverses: false,
    }
  }

  fn easing_function(easing: EasingFunction, from: f32, to: f32, duration: f32) -> f32 {
    match easing {
      EasingFunction::EaseIn => ease(EaseIn, from, to, duration),
      EasingFunction::EaseInCubic => ease(EaseInCubic, from, to, duration),
      EasingFunction::EaseInOut => ease(EaseInOut, from, to, duration),
      EasingFunction::EaseInOutCubic => ease(EaseInOutCubic, from, to, duration),
      EasingFunction::EaseInOutQuad => ease(EaseInOutQuad, from, to, duration),
      EasingFunction::EaseInOutQuart => ease(EaseInOutQuart, from, to, duration),
      EasingFunction::EaseInOutQuint => ease(EaseInOutQuint, from, to, duration),
      EasingFunction::EaseInQuad => ease(EaseInQuad, from, to, duration),
      EasingFunction::EaseInQuart => ease(EaseInQuart, from, to, duration),
      EasingFunction::EaseInQuint => ease(EaseInQuint, from, to, duration),
      EasingFunction::EaseOut => ease(EaseOut, from, to, duration),
      EasingFunction::EaseOutCubic => ease(EaseOutCubic, from, to, duration),
      EasingFunction::EaseOutQuad => ease(EaseOutQuad, from, to, duration),
      EasingFunction::EaseOutQuart => ease(EaseOutQuart, from, to, duration),
      EasingFunction::EaseOutQuint => ease(EaseOutQuint, from, to, duration),
      EasingFunction::Linear => ease(Linear, from, to, duration),
      EasingFunction::Step => ease(Step, from, to, duration),
    }
  }

  pub fn apply_translation_x(
    &mut self,
    from_value: i32,
    to_value: i32,
    time: f32,
    easing: EasingFunction,
  ) {
    self.translation_x_running = true;
    self.translation_x_ease = easing;
    self.translation_x_from_value = from_value;
    self.translation_x_to_value = to_value;
    self.translation_x_time_duration = time * 1000.0; // msec.
  }

  pub fn apply_translation_y(
    &mut self,
    from_value: i32,
    to_value: i32,
    time: f32,
    easing: EasingFunction,
  ) {
    self.translation_y_running = true;
    self.translation_y_ease = easing;
    self.translation_y_from_value = from_value;
    self.translation_y_to_value = to_value;
    self.translation_y_time_duration = time * 1000.0; // msec.
  }

  pub fn apply_rotation(
    &mut self,
    from_value: i32,
    to_value: i32,
    time: f32,
    easing: EasingFunction,
  ) {
    self.rotation_running = true;
    self.rotation_ease = easing;
    self.rotation_from_value = from_value;
    self.rotation_to_value = to_value;
    self.rotation_time_duration = time * 1000.0; // msec.
  }

  pub fn apply_scale(&mut self, from_value: f32, to_value: f32, time: f32, easing: EasingFunction) {
    self.scale_running = true;
    self.scale_ease = easing;
    self.scale_from_value = from_value;
    self.scale_to_value = to_value;
    self.scale_time_duration = time * 1000.0; // msec.
  }

  pub fn apply_opacity(&mut self, from_value: f32, to_value: f32, time: f32, easing: EasingFunction) {
    self.opacity_running = true;
    self.opacity_ease = easing;
    self.opacity_from_value = from_value;
    self.opacity_to_value = to_value;
    self.opacity_time_duration = time * 1000.0; // msec.
  }

  // CoreAnimation-style API: Create basic animation with keyPath
  pub fn with_key_path(_key_path: &str) -> Animation {
    let mut animation = Animation::new();
    // Set default duration
    animation.duration = 1.0;
    animation.timing_function = Some(EasingFunction::Linear);
    animation
  }

  // CoreAnimation-style API: Set from value for position.x
  pub fn set_from_value_position_x(&mut self, value: i32) {
    self.translation_x_from_value = value;
  }

  // CoreAnimation-style API: Set to value for position.x
  pub fn set_to_value_position_x(&mut self, value: i32) {
    self.translation_x_to_value = value;
    self.translation_x_running = true;
    if let Some(timing) = self.timing_function {
      self.translation_x_ease = timing;
    }
    self.translation_x_time_duration = self.duration * 1000.0;
  }

  // CoreAnimation-style API: Set from value for position.y
  pub fn set_from_value_position_y(&mut self, value: i32) {
    self.translation_y_from_value = value;
  }

  // CoreAnimation-style API: Set to value for position.y
  pub fn set_to_value_position_y(&mut self, value: i32) {
    self.translation_y_to_value = value;
    self.translation_y_running = true;
    if let Some(timing) = self.timing_function {
      self.translation_y_ease = timing;
    }
    self.translation_y_time_duration = self.duration * 1000.0;
  }

  // CoreAnimation-style API: Set from value for opacity
  pub fn set_from_value_opacity(&mut self, value: f32) {
    self.opacity_from_value = value;
  }

  // CoreAnimation-style API: Set to value for opacity
  pub fn set_to_value_opacity(&mut self, value: f32) {
    self.opacity_to_value = value;
    self.opacity_running = true;
    if let Some(timing) = self.timing_function {
      self.opacity_ease = timing;
    }
    self.opacity_time_duration = self.duration * 1000.0;
  }

  // CoreAnimation-style API: Set from value for transform.scale
  pub fn set_from_value_scale(&mut self, value: f32) {
    self.scale_from_value = value;
  }

  // CoreAnimation-style API: Set to value for transform.scale
  pub fn set_to_value_scale(&mut self, value: f32) {
    self.scale_to_value = value;
    self.scale_running = true;
    if let Some(timing) = self.timing_function {
      self.scale_ease = timing;
    }
    self.scale_time_duration = self.duration * 1000.0;
  }

  // CoreAnimation-style API: Set from value for transform.rotation
  pub fn set_from_value_rotation(&mut self, value: i32) {
    self.rotation_from_value = value;
  }

  // CoreAnimation-style API: Set to value for transform.rotation
  pub fn set_to_value_rotation(&mut self, value: i32) {
    self.rotation_to_value = value;
    self.rotation_running = true;
    if let Some(timing) = self.timing_function {
      self.rotation_ease = timing;
    }
    self.rotation_time_duration = self.duration * 1000.0;
  }

  pub fn run(&mut self, actor: &mut Actor) {
    if self.translation_x_running == true {
      if self.translation_x_starting_time == 0 {
        self.translation_x_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
      let cur_time = (self.animation_time_instance.elapsed().as_millis()
        - self.translation_x_starting_time) as f32
        / self.translation_x_time_duration;
      if cur_time <= 1.0 {
        actor.x = Animation::easing_function(
          self.translation_x_ease,
          self.translation_x_from_value as f32,
          self.translation_x_to_value as f32,
          cur_time,
        ) as i32;
      } else {
        self.translation_x_running = false;
        self.translation_x_starting_time = 0;
        actor.x = self.translation_x_to_value;
      }
    }

    if self.translation_y_running == true {
      if self.translation_y_starting_time == 0 {
        self.translation_y_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
      let cur_time = (self.animation_time_instance.elapsed().as_millis()
        - self.translation_y_starting_time) as f32
        / self.translation_y_time_duration;
      if cur_time <= 1.0 {
        actor.y = Animation::easing_function(
          self.translation_y_ease,
          self.translation_y_from_value as f32,
          self.translation_y_to_value as f32,
          cur_time,
        ) as i32;
      } else {
        self.translation_y_running = false;
        self.translation_y_starting_time = 0;
        actor.y = self.translation_y_to_value;
      }
    }

    if self.rotation_running == true {
      if self.rotation_starting_time == 0 {
        self.rotation_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis()
        - self.rotation_starting_time) as f32
        / self.rotation_time_duration as f32;
      if cur_time <= 1.0 {
        actor.rotation = Animation::easing_function(
          self.rotation_ease,
          self.rotation_from_value as f32,
          self.rotation_to_value as f32,
          cur_time,
        ) as i32;
      } else {
        self.rotation_running = false;
        self.rotation_starting_time = 0;
        actor.rotation = self.rotation_to_value;
      }
    }

    if self.scale_running == true {
      if self.scale_starting_time == 0 {
        self.scale_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis() - self.scale_starting_time)
        as f32
        / self.scale_time_duration as f32;
      if cur_time <= 1.0 {
        actor.scale_x = Animation::easing_function(
          self.scale_ease,
          self.scale_from_value,
          self.scale_to_value,
          cur_time,
        ) as f32;
        actor.scale_y = Animation::easing_function(
          self.scale_ease,
          self.scale_from_value,
          self.scale_to_value,
          cur_time,
        ) as f32;
      } else {
        self.scale_running = false;
        self.scale_starting_time = 0;
        actor.scale_x = self.scale_to_value;
        actor.scale_y = self.scale_to_value;
      }
    }

    if self.opacity_running == true {
      if self.opacity_starting_time == 0 {
        self.opacity_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis() - self.opacity_starting_time)
        as f32
        / self.opacity_time_duration as f32;
      if cur_time <= 1.0 {
        actor.opacity = Animation::easing_function(
          self.opacity_ease,
          self.opacity_from_value,
          self.opacity_to_value,
          cur_time,
        );
      } else {
        self.opacity_running = false;
        self.opacity_starting_time = 0;
        actor.opacity = self.opacity_to_value;
      }
    }

    if self.translation_x_running == true
      || self.translation_y_running == true
      || self.rotation_running == true
      || self.scale_running == true
      || self.opacity_running == true
    {
      actor.animated = true;
    } else {
      actor.animated = false;
    }
  }
}
