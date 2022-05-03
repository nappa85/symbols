use sea_orm::{
    sea_query::{Expr, SimpleExpr},
    EntityTrait,
};

pub trait EntityFilter: EntityTrait + Default {
    fn filter() -> SimpleExpr {
        Expr::val(1).eq(1)
    }
}
