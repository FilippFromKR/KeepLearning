// macro is_trait!()
// https://stackoverflow.com/questions/71720817/check-if-a-trait-is-implemented-or-not

pub struct NoSynNoSend
{
    _marker: std::marker::PhantomData<std::rc::Rc<()>>,
}

pub struct SyncAndSend {
    id: u64,
}

pub struct NoSend<'a>
{
    _marker: std::marker::PhantomData<&'a ()>,
}

pub struct NoSync
{
    _marker: std::marker::PhantomData<std::cell::RefCell<()>>,
}
#[cfg(test)]
mod test_send {
    use crate::trade_safety::{NoSync, NoSynNoSend};

    fn both<T:Send+Sync>(some:T){}
    fn only_sync<T:Sync>(some:T){}
    fn only_send<T:Send>(some:T){}
    ///
    /// ```compile_fail
    /// only_sync(NoSync{ _marker: Default::default() });
    /// ```
    #[test]
    fn send_no_sync() {
        only_send(NoSync{ _marker: Default::default() })
    }
    #[test]
    fn sync_no_send() {
        only_send(NoSync{ _marker: Default::default() })
    }
}