pub struct WriteOptions {
    pub parent: ParentPolicy,
    pub collision: CollisionStrategy,
}

pub enum ParentPolicy {
    RequireExists,
    CreateIfMissing,
}

pub enum CollisionStrategy {
    Error,
    Overwrite,
    Rename(RenameOptions),
    Skip,
}

pub enum RenameOptions {
    Timestamp,
    Uuid,
    Counter,
}
