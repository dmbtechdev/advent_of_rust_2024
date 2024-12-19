// 1. We have 3 states:
// - Empty
// - Ready
// - Flying


use std::marker::PhantomData;
 
pub struct Empty;
pub struct Ready;
pub struct Flying;

// pub trait State {}
// impl State for Empty {}
// impl State for Ready {}
// impl State for Flying {}

// 2. Finish the Seligh struct definition,

// Empty State
// new() an associated function that creates a new Sleigh in the Empty state.
// load() a method that transitions the Sleigh from Empty to Ready.
// Ready State
// take_off() a method that transitions the Sleigh from Ready to Flying.
// unload() a method that transitions the Sleigh from Ready to Empty.
// Flying State
// land() a method that transitions the Sleigh from Flying to Ready.

pub struct Sleigh<T> {
    _state: PhantomData<T>,
}

// pub struct Sleigh<State> {
//     _state: PhantomData<State>,
// }


// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { _state: PhantomData }
    }
 
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { _state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { _state: PhantomData }
    }
 
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { _state: PhantomData }
    }
}
 
impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { _state: PhantomData }
    }
}

