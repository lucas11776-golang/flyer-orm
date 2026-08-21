use crate::types::Bindable;



pub fn to_args<'a, DB>(args: Vec<&'a Box<dyn Bindable<DB>>>) -> <DB as sqlx::Database>::Arguments<'a>
where
    DB: sqlx::Database
{
    let mut arguments= Default::default();

    for arg in args {
        arg.bind_to(&mut arguments).unwrap();
    }

    return arguments;
}