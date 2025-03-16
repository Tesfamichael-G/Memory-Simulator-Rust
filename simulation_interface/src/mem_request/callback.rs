pub trait Callback {
    fn invoke(&self, block_address: usize);
}

// pub struct TransactionCallback <C: Callback> {
//
//     pub block_address: usize,
//     pub completion_time: u32,
//     pub callback: C
// }

pub struct ReadCallback {

    pub block_address: usize,
    pub completion_time: u32,
    pub data: u32
}

// pub struct ReadCompleteCallback<C> {
//     callback: C,
// }
//
// impl<C> ReadCompleteCallback<C> {//: FnOnce(usize) C:Callback
//     pub fn new(callback: C) -> Self where C: FnOnce(usize) {
//         ReadCompleteCallback { callback }
//     }
// }
//
// impl<C: Callback>  Callback for ReadCompleteCallback<C> {
//     fn invoke(&self, block_address: usize) {
//         (self.callback).invoke(block_address);
//     }
// }

//
// pub struct ReadCompleteCallback<F: Fn(usize) + 'static> {
//     callback: F,
// }
//
// impl<F: Fn(usize) + 'static> ReadCompleteCallback<F> {
//     pub fn new(callback: F) -> Self {
//         ReadCompleteCallback { callback }
//     }
//
//     pub fn invoke(&self, arg: usize) {
//         (self.callback)(arg);
//     }
// }
//
// impl<F: Fn(usize) + 'static> FnOnce<(usize,)> for ReadCompleteCallback<F> {
//     type Output = ();
//
//     extern "rust-call" fn call_once(self, args: (usize,)) {
//         (self.callback)(args.0)
//     }
// }
//
// impl<F: Fn(usize) + 'static> FnMut<(usize,)> for ReadCompleteCallback<F> {
//     extern "rust-call" fn call_mut(&mut self, args: (usize,)) {
//         (self.callback)(args.0)
//     }
// }
//
// impl<F: Fn(usize) + 'static> Fn<(usize,)> for ReadCompleteCallback<F> {
//     extern "rust-call" fn call(&self, args: (usize,)) {
//         (self.callback)(args.0)
//     }
// }

// pub struct ReadCompleteCallback<C> where C: Fn(usize),
// {
//     callback: C,
// }
//


// impl<C> ReadCompleteCallback<C>
//     where
//         C: Fn(usize),
// {
//     pub fn new(callback: C) -> Self {
//         Self { callback }
//     }
//
//     pub fn invoke(&self, arg: usize) {
//         (self.callback)(arg);
//     }
// }

