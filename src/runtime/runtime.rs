use crate::runtime::msg::{Action, Event, Msg};
use crate::runtime::{Effect, Effects, Environment, Model};
use derivative::Derivative;
use enclose::enclose;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::FutureExt;
use serde::Serialize;
use std::marker::PhantomData;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard};

#[derive(Serialize)]
#[serde(tag = "name", content = "args")]
pub enum RuntimeEvent {
    NewState,
    Event(Event),
}

#[derive(Derivative)]
#[derivative(Clone(bound = ""))]
pub struct Runtime<Env: Environment, M: Model> {
    model: Arc<RwLock<M>>,
    tx: Sender<RuntimeEvent>,
    env: PhantomData<Env>,
}

impl<Env, M> Runtime<Env, M>
where
    Env: Environment + 'static,
    M: Model + 'static,
{
    pub fn new(model: M, buffer: usize) -> (Self, Receiver<RuntimeEvent>) {
        let (tx, rx) = channel(buffer);
        let model = Arc::new(RwLock::new(model));
        (
            Runtime {
                model,
                tx,
                env: PhantomData,
            },
            rx,
        )
    }
    pub fn model(&self) -> LockResult<RwLockReadGuard<M>> {
        self.model.read()
    }
    pub fn dispatch(&self, action: Action) {
        let effects = self
            .model
            .write()
            .expect("model write failed")
            .update(&Msg::Action(action));
        self.handle_effects(effects);
    }
    pub fn dispatch_to_field(&self, field: &M::Field, action: Action) {
        let effects = self
            .model
            .write()
            .expect("model write failed")
            .update_field(&field, &Msg::Action(action));
        self.handle_effects(effects);
    }
    fn emit(&self, event: RuntimeEvent) {
        self.tx.clone().try_send(event).expect("emit event failed");
    }
    fn handle_effects(&self, effects: Effects) {
        if effects.has_changed {
            self.emit(RuntimeEvent::NewState);
        };
        effects
            .into_iter()
            .for_each(enclose!((self.clone() => runtime) move |effect| {
                match effect {
                    Effect::Msg(msg) => {
                        runtime.handle_effect_output(msg);
                    }
                    Effect::Future(future) => {
                        Env::exec(future.then(enclose!((runtime) move |msg| async move {
                            runtime.handle_effect_output(msg);
                        })))
                    }
                }
            }));
    }
    fn handle_effect_output(&self, msg: Msg) {
        match msg {
            Msg::Event(event) => {
                self.emit(RuntimeEvent::Event(event));
            }
            Msg::Internal(_) => {
                let effects = self.model.write().expect("model write failed").update(&msg);
                self.handle_effects(effects);
            }
            Msg::Action(_) => {
                panic!("effects are not allowed to resolve with action");
            }
        }
    }
}
