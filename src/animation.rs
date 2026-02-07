use crate::layer::RALayer;
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
  // Note: Currently key_path is for API compatibility only. In the future, this could
  // automatically configure the animation type based on the key path (e.g., "position.x",
  // "opacity", "transform.scale"). For now, callers should use the set_*_value methods
  // to configure the specific animation.
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

  pub fn run(&mut self, layer: &mut RALayer) {
    if self.translation_x_running {
      if self.translation_x_starting_time == 0 {
        self.translation_x_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
      let cur_time = (self.animation_time_instance.elapsed().as_millis()
        - self.translation_x_starting_time) as f32
        / self.translation_x_time_duration;
      if cur_time <= 1.0 {
        layer.x = Animation::easing_function(
          self.translation_x_ease,
          self.translation_x_from_value as f32,
          self.translation_x_to_value as f32,
          cur_time,
        ) as i32;
      } else {
        self.translation_x_running = false;
        self.translation_x_starting_time = 0;
        layer.x = self.translation_x_to_value;
      }
    }

    if self.translation_y_running {
      if self.translation_y_starting_time == 0 {
        self.translation_y_starting_time = self.animation_time_instance.elapsed().as_millis();
      }
      let cur_time = (self.animation_time_instance.elapsed().as_millis()
        - self.translation_y_starting_time) as f32
        / self.translation_y_time_duration;
      if cur_time <= 1.0 {
        layer.y = Animation::easing_function(
          self.translation_y_ease,
          self.translation_y_from_value as f32,
          self.translation_y_to_value as f32,
          cur_time,
        ) as i32;
      } else {
        self.translation_y_running = false;
        self.translation_y_starting_time = 0;
        layer.y = self.translation_y_to_value;
      }
    }

    if self.rotation_running {
      if self.rotation_starting_time == 0 {
        self.rotation_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis()
        - self.rotation_starting_time) as f32
        / self.rotation_time_duration as f32;
      if cur_time <= 1.0 {
        layer.rotation = Animation::easing_function(
          self.rotation_ease,
          self.rotation_from_value as f32,
          self.rotation_to_value as f32,
          cur_time,
        ) as i32;
      } else {
        self.rotation_running = false;
        self.rotation_starting_time = 0;
        layer.rotation = self.rotation_to_value;
      }
    }

    if self.scale_running {
      if self.scale_starting_time == 0 {
        self.scale_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis() - self.scale_starting_time)
        as f32
        / self.scale_time_duration as f32;
      if cur_time <= 1.0 {
        layer.scale_x = Animation::easing_function(
          self.scale_ease,
          self.scale_from_value,
          self.scale_to_value,
          cur_time,
        ) as f32;
        layer.scale_y = Animation::easing_function(
          self.scale_ease,
          self.scale_from_value,
          self.scale_to_value,
          cur_time,
        ) as f32;
      } else {
        self.scale_running = false;
        self.scale_starting_time = 0;
        layer.scale_x = self.scale_to_value;
        layer.scale_y = self.scale_to_value;
      }
    }

    if self.opacity_running {
      if self.opacity_starting_time == 0 {
        self.opacity_starting_time = self.animation_time_instance.elapsed().as_millis();
      }

      let cur_time = (self.animation_time_instance.elapsed().as_millis() - self.opacity_starting_time)
        as f32
        / self.opacity_time_duration as f32;
      if cur_time <= 1.0 {
        layer.opacity = Animation::easing_function(
          self.opacity_ease,
          self.opacity_from_value,
          self.opacity_to_value,
          cur_time,
        );
      } else {
        self.opacity_running = false;
        self.opacity_starting_time = 0;
        layer.opacity = self.opacity_to_value;
      }
    }

    if self.translation_x_running
      || self.translation_y_running
      || self.rotation_running
      || self.scale_running
      || self.opacity_running
    {
      layer.animated = true;
    } else {
      layer.animated = false;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_animation_with_key_path() {
    let animation = Animation::with_key_path("position.x");
    assert_eq!(animation.duration, 1.0);
    assert!(animation.timing_function.is_some());
  }

  #[test]
  fn test_animation_coreanimation_properties() {
    let mut animation = Animation::with_key_path("position.x");
    animation.duration = 3.5;
    animation.timing_function = Some(EasingFunction::EaseInOut);
    animation.repeat_count = 2.0;
    animation.autoreverses = true;
    
    assert_eq!(animation.duration, 3.5);
    assert_eq!(animation.repeat_count, 2.0);
    assert!(animation.autoreverses);
  }

  #[test]
  fn test_position_animation_setters() {
    let mut animation = Animation::with_key_path("position.x");
    animation.duration = 2.0;
    animation.timing_function = Some(EasingFunction::Linear);
    
    animation.set_from_value_position_x(100);
    animation.set_to_value_position_x(400);
    
    assert_eq!(animation.translation_x_from_value, 100);
    assert_eq!(animation.translation_x_to_value, 400);
    assert!(animation.translation_x_running);
  }

  #[test]
  fn test_opacity_animation_setters() {
    let mut animation = Animation::with_key_path("opacity");
    animation.duration = 1.5;
    
    animation.set_from_value_opacity(1.0);
    animation.set_to_value_opacity(0.5);
    
    assert_eq!(animation.opacity_from_value, 1.0);
    assert_eq!(animation.opacity_to_value, 0.5);
    assert!(animation.opacity_running);
  }

  #[test]
  fn test_scale_animation_setters() {
    let mut animation = Animation::with_key_path("transform.scale");
    animation.duration = 2.5;
    
    animation.set_from_value_scale(1.0);
    animation.set_to_value_scale(2.0);
    
    assert_eq!(animation.scale_from_value, 1.0);
    assert_eq!(animation.scale_to_value, 2.0);
    assert!(animation.scale_running);
  }

  #[test]
  fn test_rotation_animation_setters() {
    let mut animation = Animation::with_key_path("transform.rotation");
    animation.duration = 3.0;
    
    animation.set_from_value_rotation(0);
    animation.set_to_value_rotation(360);
    
    assert_eq!(animation.rotation_from_value, 0);
    assert_eq!(animation.rotation_to_value, 360);
    assert!(animation.rotation_running);
  }

  #[test]
  fn test_backward_compatibility_animation() {
    let mut animation = Animation::new();
    animation.apply_translation_x(0, 100, 1.0, EasingFunction::Linear);
    animation.apply_translation_y(0, 200, 1.0, EasingFunction::EaseIn);
    animation.apply_scale(1.0, 2.0, 1.0, EasingFunction::EaseOut);
    animation.apply_rotation(0, 180, 1.0, EasingFunction::EaseInOut);
    
    assert!(animation.translation_x_running);
    assert!(animation.translation_y_running);
    assert!(animation.scale_running);
    assert!(animation.rotation_running);
  }
}
