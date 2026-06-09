pub static mut FRAMEWORK:Option<Framework> = None;

pub mod collections
{
    pub use std::collections::{ * };
} pub use self::collections::{ HashMap };

pub mod marker
{
    pub use std::marker::{ * };
} pub use self::marker::{ Send };

pub mod sync
{
    pub use std::sync::{ * };
} pub use self::sync::mpsc::{ channel, Receiver, Sender, TryRecvError };

pub mod thread
{
    pub use std::thread::{ * };
} 

pub enum Entity
{
    Dictionary( HashMap<String, String> ),
    Promise( Promise<String, String> )
}

pub struct Promise<T: Send, E: Send>
{
    receiver: Receiver<Result<T, E>>
}

impl<T: Send + 'static, E: Send + 'static> Promise<T, E>
{
    pub fn then<T2, E2, F1, F2>(self, callback: F1, errback: F2) -> Promise<T2, E2> where
    T2: Send + 'static, 
    E2: Send + 'static,
    F1: FnOnce(T) -> Result<T2, E2>, 
    F2: FnOnce(E) -> Result<T2, E2>,
    F1: Send + 'static, F2: Send + 'static
    {
        let recv = self.receiver;
        let (tx, rx) = channel();

        thread::spawn(move ||
        {
            Promise::impl_then(tx, recv, callback, errback);
        });

        Promise { receiver: rx }
    }
    
    pub fn then_result<T2, E2, F>(self, callback: F) -> Promise<T2, E2> where
    T2: Send + 'static,
    E2: Send + 'static,
    F: FnOnce(Result<T, E>) -> Result<T2, E2>,
    F: Send + 'static
    {
        let recv = self.receiver;
        let (tx, rx) = channel();

        thread::spawn(move || 
        {
            Promise::impl_then_result(tx, recv, callback);
        });

        Promise { receiver: rx }
    }
    
    pub fn ok_then<T2, F>(self, callback: F) -> Promise<T2, E> where
    T2: Send + 'static,
    F: Send + 'static,
    F: FnOnce(T) -> Result<T2, E>
    {
        let recv = self.receiver;
        let (tx, rx) = channel();

        thread::spawn(move ||
        {
            Promise::impl_ok_then(tx, recv, callback);
        });

        Promise { receiver: rx }
    }
    
    pub fn err_then<E2, F>(self, errback: F) -> Promise<T, E2> where
    F: FnOnce(E) -> Result<T, E2>,
    F: Send + 'static,
    E2: Send + 'static
    {
        let recv = self.receiver;
        let (tx, rx) = channel();

        thread::spawn(move ||
        {
            Promise::impl_err_then(tx, recv, errback);
        });

        Promise { receiver: rx }
    }
    
    pub fn new<F>(func: F) -> Promise<T, E> where
    F: FnOnce() -> Result<T, E>,
    F: Send + 'static
    {
        let (tx, rx) = channel();

        thread::spawn(move ||
        {
            Promise::impl_new(tx, func);
        });

        Promise { receiver: rx }
    }
    
    pub fn race(promises: Vec<Promise<T, E>>) -> Promise<T, E>
    {
        let recs = promises.into_iter().map(|p| p.receiver).collect();
        let (tx, rx) = channel::<Result<T, E>>();
        thread::spawn(move ||
        {
            Promise::impl_race(tx, recs);
        });

        Promise { receiver: rx }
    }
    
    pub fn all(promises: Vec<Promise<T, E>>) -> Promise<Vec<T>, E>
    {
        let receivers: Vec<Receiver<Result<T, E>>> = promises.into_iter().map(|p| p.receiver).collect();
        let (tx, rx) = channel();

        thread::spawn(move || 
        {
            Promise::impl_all(tx, receivers);
        });

        return Promise { receiver: rx };
    }
    
    pub fn resolve(val: T) -> Promise<T, E>
    {
        Promise::from_result(Ok(val))
    }
    
    pub fn reject(val: E) -> Promise<T, E>
    {
        Promise::from_result(Err(val))
    }
    
    pub fn from_result(result: Result<T, E>) -> Promise<T, E>
    {
        let (tx, rx) = channel();
        tx.send(result).unwrap();
        Promise { receiver: rx }
    }
    
    fn impl_new<F>(tx: Sender<Result<T, E>>, func: F) where
    F: FnOnce() -> Result<T, E>, 
    F: Send + 'static
    {
        let result = func();
        tx.send(result).unwrap_or(());
    }

    fn impl_then<T2, E2, F1, F2>(tx: Sender<Result<T2, E2>>, rx: Receiver<Result<T, E>>, callback: F1, errback: F2) where
    T2: Send + 'static, E2: Send + 'static,
    F1: FnOnce(T) -> Result<T2, E2>, F2: FnOnce(E) -> Result<T2, E2>,
    F1: Send + 'static, F2: Send + 'static
    {
        if let Ok(message) = rx.recv()
        {
            match message
            {
                Ok(val) => tx.send(callback(val)).unwrap_or(()),
                Err(err) => tx.send(errback(err)).unwrap_or(())
            };
        }
    }

    fn impl_then_result<T2, E2, F>(tx: Sender<Result<T2, E2>>, rx: Receiver<Result<T, E>>, callback: F) where
    T2: Send + 'static, E2: Send + 'static,
    F: FnOnce(Result<T, E>) -> Result<T2, E2>, F: Send + 'static
    {
        if let Ok(result) = rx.recv()
        {
            tx.send(callback(result)).unwrap_or(());
        }
    }

    fn impl_ok_then<T2, F>(tx: Sender<Result<T2, E>>, rx: Receiver<Result<T, E>>, callback: F) where
    F: FnOnce(T) -> Result<T2, E>, F: Send + 'static,
    T2: Send + 'static
    {
        if let Ok(message) = rx.recv()
        {
            match message
            {
                Ok(val) => tx.send(callback(val)).unwrap_or(()),
                Err(err) => tx.send(Err(err)).unwrap_or(())
            }
        }
    }

    fn impl_err_then<E2, F>(tx: Sender<Result<T, E2>>, rx: Receiver<Result<T, E>>, errback: F) where
    F: FnOnce(E) -> Result<T, E2>, F: Send + 'static,
    E2: Send + 'static
    {
        if let Ok(message) = rx.recv()
        {
            match message
            {
                Ok(val) => tx.send(Ok(val)).unwrap_or(()),
                Err(err) => tx.send(errback(err)).unwrap_or(())
            }
        }
    }
    
    fn impl_race(tx: Sender<Result<T, E>>, mut recs: Vec<Receiver<Result<T, E>>>)
    {
        'outer: loop
        {
            if recs.len() == 0 { return; }

            for i in 0..recs.len()
            {
                match recs[i].try_recv()
                {
                    Ok(val) =>
                    {
                        tx.send(val).unwrap_or(());
                        return;
                    }

                    Err(err) =>
                    {
                        if err == TryRecvError::Disconnected { recs.remove(i); }
                    }
                }
            }
        }
    }

    fn impl_all(tx: Sender<Result<Vec<T>, E>>, recs: Vec<Receiver<Result<T, E>>>)
    {
        let mut values: Vec<T> = Vec::with_capacity(recs.len());
        let mut mut_receivers = recs;
        'outer: loop
        {
            for i in 0..mut_receivers.len()
            {
                match mut_receivers[i].try_recv()
                {
                    Ok(val) =>
                    {
                        match val
                        {
                            Ok(t) => values.push(t),
                            Err(e) =>
                            {
                                tx.send(Err(e)).unwrap_or(());
                                return;
                            }
                        }

                        mut_receivers.remove(i);
                    }

                    Err(err) =>
                    {
                        if err == TryRecvError::Disconnected { mut_receivers.remove(i); }
                    }
                }
            }
            
            if mut_receivers.len() == 0
            {
                let result = Ok(values);
                tx.send(result).unwrap_or(());
                return;
            }
        }
    }
}

pub struct Framework
{
    pub module:HashMap<String, Entity>,
}

impl Framework
{
    pub fn new( module:Option<HashMap<String, Entity>> ) -> Self
    {
        match module
        {
            None=>
            {
                Self
                {
                    module:HashMap::new()
                }
            }

            Some( module ) =>
            {
                Self
                {
                    module
                }
            }
        }
    }
}

pub unsafe fn framework() -> ()
{
    unsafe
    {
        return ();
    }
}

pub unsafe fn domain() -> ()
{
    unsafe
    {
        FRAMEWORK = Some( Framework::new( None ) );
        return ();
    }
}

pub fn main()
{
    unsafe
    {
        let _ = domain();
    }
}
