use anyhow::anyhow;

/// A command to change to a new scene, either by pushign a new one,
/// popping one or replacing the current scene (pop and then push).
pub enum SceneSwitch<C> {
    None,
    Push(Box<dyn Scene<C>>),
    Replace(Box<dyn Scene<C>>),
    Reinit(Box<dyn Scene<C>>),
    Pop,
}

/// A trait for you to implement on a scene.
/// Defines the callbacks the scene uses:
/// a common context type `C`, and an input event type `Ev`.
pub trait Scene<C> {
    fn poll_input(&mut self, resources: &mut C) -> anyhow::Result<()>;
    fn update(&mut self, resources: &mut C) -> SceneSwitch<C>;
    fn draw(&mut self, resources: &mut C) -> anyhow::Result<()>;
    /// This returns whether or not to draw the next scene down on the
    /// stack as well; this is useful for layers or GUI stuff that
    /// only partially covers the screen.
    fn draw_previous(&self) -> bool {
        false
    }
}

impl<C> SceneSwitch<C> {
    /// Convenient shortcut function for boxing scenes.
    ///
    /// Slightly nicer than writing
    /// `SceneSwitch::Replace(Box::new(x))` all the damn time.
    pub fn replace<S>(scene: S) -> Self
    where
        S: Scene<C> + 'static,
    {
        SceneSwitch::Replace(Box::new(scene))
    }

    /// Same as `replace()` but returns SceneSwitch::Push
    pub fn push<S>(scene: S) -> Self
    where
        S: Scene<C> + 'static,
    {
        SceneSwitch::Push(Box::new(scene))
    }
}

/// A stack of `Scene`'s, together with a context object.
pub struct SceneStack<C> {
    pub resources: C,
    scenes: Vec<Box<dyn Scene<C>>>,
}

impl<C> SceneStack<C> {
    pub fn new(global_state: C) -> Self {
        Self {
            resources: global_state,
            scenes: Vec::new(),
        }
    }

    /// Reinitialize the SceneStack with a single scene (eg. return to main menu)
    pub fn reinit(&mut self, scene: Box<dyn Scene<C>>) {
        self.scenes.clear();
        self.scenes.push(scene)
    }

    /// Add a new scene to the top of the stack.
    pub fn push(&mut self, scene: Box<dyn Scene<C>>) {
        self.scenes.push(scene)
    }

    /// Remove the top scene from the stack and returns it;
    /// panics if there is none.
    pub fn pop(&mut self) -> Box<dyn Scene<C>> {
        self.scenes
            .pop()
            .expect("ERROR: Popped an empty scene stack.")
    }

    /// Returns the current scene; panics if there is none.
    pub fn current(&self) -> &dyn Scene<C> {
        &**self
            .scenes
            .last()
            .expect("ERROR: Tried to get current scene of an empty scene stack.")
    }

    /// Executes the given SceneSwitch command; if it is a pop or replace
    /// it returns `Some(old_scene)`, otherwise `None`
    pub fn switch(&mut self, next_scene: SceneSwitch<C>) -> Option<Box<dyn Scene<C>>> {
        match next_scene {
            SceneSwitch::None => None,
            SceneSwitch::Pop => {
                let s = self.pop();
                Some(s)
            }
            SceneSwitch::Push(s) => {
                self.push(s);
                None
            }
            SceneSwitch::Replace(s) => {
                let old_scene = self.pop();
                self.push(s);
                Some(old_scene)
            }
            SceneSwitch::Reinit(s) => {
                self.reinit(s);
                None
            }
        }
    }

    /// Feeds the given input event to the current scene.
    pub fn poll_input(&mut self) -> anyhow::Result<()> {
        let current_scene = &mut **self
            .scenes
            .last_mut()
            .ok_or_else(|| anyhow!("Tried to poll input for empty scene stack"))?;

        current_scene.poll_input(&mut self.resources)
    }

    // These functions must be on the SceneStack because otherwise
    // if you try to get the current scene and the world to call
    // update() on the current scene it causes a double-borrow.  :/
    pub fn update(&mut self) -> anyhow::Result<()> {
        let next_scene = {
            let current_scene = &mut **self
                .scenes
                .last_mut()
                .expect("Tried to update empty scene stack");
            current_scene.update(&mut self.resources)
        };
        self.switch(next_scene);

        Ok(())
    }

    /// Draw the current scene.
    pub fn draw(&mut self) -> anyhow::Result<()> {
        SceneStack::draw_scenes(&mut self.scenes, &mut self.resources)
    }

    /// We walk down the scene stack until we find a scene where we aren't
    /// supposed to draw the previous one, then draw them from the bottom up.
    ///
    /// This allows for layering GUI's and such.
    fn draw_scenes(scenes: &mut [Box<dyn Scene<C>>], resources: &mut C) -> anyhow::Result<()> {
        assert!(!scenes.is_empty());
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, resources);
            }
            current.draw(resources)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Thing {
        scenes: Vec<SceneStack<u32>>,
    }

    #[test]
    fn test1() {
        let _x = Thing { scenes: vec![] };
    }
}
