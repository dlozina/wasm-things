use std::error::Error;
use wasmtime::*;

struct Log {
    integers_logged: Vec<u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    // It can be only one across all the threads
    let engine = Engine::default();
    let module = Module::from_file(&engine, "run.wat")?;

    // For host-provided functions it's recommended to use a `Linker` which does
    // name-based resolution of functions.
    let mut linker = Linker::new(&engine);

    //***********************
    // host-defined functions
    // First we create our simple "double" function which will only multiply its
    // input by two and return it.
    linker.func_wrap("", "double", |param: i32| param * 2)?;

    // Next we define a `log` function. Note that we're using a
    // Wasmtime-provided `Caller` argument to access the state on the `Store`,
    // which allows us to record the logged information.
    linker.func_wrap("", "log", |mut caller: Caller<'_, Log>, param: u32| {
        println!("log: {}", param);
        caller.data_mut().integers_logged.push(param);
    })?;
    //***********************

    // As above, instantiation always happens within a `Store`. This means to
    // actually instantiate with our `Linker` we'll need to create a store. Note
    // that we're also initializing the store with our custom data here too.
    //
    // Afterwards we use the `linker` to create the instance.
    let data = Log { integers_logged: Vec::new() };
    let mut store = Store::new(&engine, data);
    let instance = linker.instantiate(&mut store, &module)?;

    // Like before, we can get the run function and execute it.
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    run.call(&mut store, ())?;

    // We can also inspect what integers were logged:
    println!("logged integers: {:?}", store.data().integers_logged);

    Ok(())
}