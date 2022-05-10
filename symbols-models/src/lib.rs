#![deny(warnings)]
#![deny(missing_docs)]

//! # Symbols-models
//! 
//! Shared traits from Symbols proc-macro-utility, to be able to share models between macros and real applications.

use sea_orm::{
    sea_query::{Expr, SimpleExpr},
    EntityTrait,
};

/// This trait allows data filtering on macro execution
/// It's default implementation simply adds WHERE 1 = 1 to data retrieve query
/// 
/// Since only basic types are supported, it's importato to use only basic types in models.
pub trait EntityFilter: EntityTrait + Default {
    /// Returned expression in injected in data retrieve query to allow data filtering
    fn filter() -> SimpleExpr {
        Expr::val(1).eq(1)
    }
}
