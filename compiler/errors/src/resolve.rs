pub enum ResolverError {
    UndefinedVariable,
    UseBeforeAssign,
    UndefinedType,
    UsingTypeAsVariable,
    UsingVariableAsType,
    CannotFindFile,
    NoSuchExport,
    PrivateExport,
    CircularImport,
    RecursiveImport,
    PrependError,
}

pub enum ResolverWarnings {
    UnusedVariable,
    DeadCode,
    InvalidCase,
}
