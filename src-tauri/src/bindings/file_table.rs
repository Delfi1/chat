// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::file_type::File;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `file`.
///
/// Obtain a handle from the [`FileTableAccess::file`] method on [`super::RemoteTables`],
/// like `ctx.db.file()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.file().on_insert(...)`.
pub struct FileTableHandle<'ctx> {
    imp: __sdk::TableHandle<File>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `file`.
///
/// Implemented for [`super::RemoteTables`].
pub trait FileTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`FileTableHandle`], which mediates access to the table `file`.
    fn file(&self) -> FileTableHandle<'_>;
}

impl FileTableAccess for super::RemoteTables {
    fn file(&self) -> FileTableHandle<'_> {
        FileTableHandle {
            imp: self.imp.get_table::<File>("file"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct FileInsertCallbackId(__sdk::CallbackId);
pub struct FileDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for FileTableHandle<'ctx> {
    type Row = File;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = File> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = FileInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> FileInsertCallbackId {
        FileInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: FileInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = FileDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> FileDeleteCallbackId {
        FileDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: FileDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<File>("file");
    _table.add_unique_constraint::<u32>("id", |row| &row.id);
}
pub struct FileUpdateCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::TableWithPrimaryKey for FileTableHandle<'ctx> {
    type UpdateCallbackId = FileUpdateCallbackId;

    fn on_update(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row, &Self::Row) + Send + 'static,
    ) -> FileUpdateCallbackId {
        FileUpdateCallbackId(self.imp.on_update(Box::new(callback)))
    }

    fn remove_on_update(&self, callback: FileUpdateCallbackId) {
        self.imp.remove_on_update(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<File>> {
    __sdk::TableUpdate::parse_table_update(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<File>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}

/// Access to the `id` unique index on the table `file`,
/// which allows point queries on the field of the same name
/// via the [`FileIdUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.file().id().find(...)`.
pub struct FileIdUnique<'ctx> {
    imp: __sdk::UniqueConstraintHandle<File, u32>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> FileTableHandle<'ctx> {
    /// Get a handle on the `id` unique index on the table `file`.
    pub fn id(&self) -> FileIdUnique<'ctx> {
        FileIdUnique {
            imp: self.imp.get_unique_constraint::<u32>("id"),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> FileIdUnique<'ctx> {
    /// Find the subscribed row whose `id` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u32) -> Option<File> {
        self.imp.find(col_val)
    }
}
