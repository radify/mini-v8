use crate::*;
use v8;
// use std::fmt;
// use std::marker::PhantomData;

#[derive(Clone)]
pub struct Promise {
    pub(crate) mv8: MiniV8,
    pub(crate) handle: v8::Global<v8::Promise>,
}

impl Promise {
  pub fn result(&self) -> Value {
    self.mv8.scope(|scope| {
      let object = v8::Local::new(scope, self.handle.clone());
      let result = object.result(scope);
      Value::from_v8_value(&self.mv8, scope, result)
    })
  }

  // pub fn then(&self, callback: v8::Function) -> Option<v8::Global<v8::Promise>> {
  //   self.mv8.scope(|scope| {
  //     let object: v8::Local<v8::Promise> = v8::Local::new(scope, &self.handle);
  //     let cb: v8::Local<v8::Function> = v8::Local::new(scope, callback);
  //     let result: Option<v8::Local<v8::Promise>> = object.then(scope, cb);

  //     match result {
  //       Some(val) => Some(v8::Global::new(scope, val)),
  //       None => None
  //     }
  //   })
  // }

  pub fn then(&self, callback: Function) -> Option<v8::Global<v8::Promise>> {
    self.mv8.scope(|scope| {
        let object: v8::Local<v8::Promise> = v8::Local::new(scope, &self.handle);
        let callback = v8::Local::new(scope, callback.handle);
        let result: Option<v8::Local<v8::Promise>> = object.then(scope, callback);

        result.map(|val| v8::Global::new(scope, &val))
    })
}
}
