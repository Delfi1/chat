// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct RemoveMessageArgs {
    pub id: u32,
}

impl From<RemoveMessageArgs> for super::Reducer {
    fn from(args: RemoveMessageArgs) -> Self {
        Self::RemoveMessage { id: args.id }
    }
}

impl __sdk::InModule for RemoveMessageArgs {
    type Module = super::RemoteModule;
}

pub struct RemoveMessageCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `remove_message`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait remove_message {
    /// Request that the remote module invoke the reducer `remove_message` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_remove_message`] callbacks.
    fn remove_message(&self, id: u32) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `remove_message`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`RemoveMessageCallbackId`] can be passed to [`Self::remove_on_remove_message`]
    /// to cancel the callback.
    fn on_remove_message(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &u32) + Send + 'static,
    ) -> RemoveMessageCallbackId;
    /// Cancel a callback previously registered by [`Self::on_remove_message`],
    /// causing it not to run in the future.
    fn remove_on_remove_message(&self, callback: RemoveMessageCallbackId);
}

impl remove_message for super::RemoteReducers {
    fn remove_message(&self, id: u32) -> __sdk::Result<()> {
        self.imp
            .call_reducer("remove_message", RemoveMessageArgs { id })
    }
    fn on_remove_message(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &u32) + Send + 'static,
    ) -> RemoveMessageCallbackId {
        RemoveMessageCallbackId(self.imp.on_reducer(
            "remove_message",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::RemoveMessage { id },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, id)
            }),
        ))
    }
    fn remove_on_remove_message(&self, callback: RemoveMessageCallbackId) {
        self.imp.remove_on_reducer("remove_message", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `remove_message`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_remove_message {
    /// Set the call-reducer flags for the reducer `remove_message` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn remove_message(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_remove_message for super::SetReducerFlags {
    fn remove_message(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("remove_message", flags);
    }
}
