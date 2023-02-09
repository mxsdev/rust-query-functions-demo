use std::{fmt::Display, any::{Any}};

fn print_name(
    name: Name,
) {
    println!("name is {name}")
}

fn print_age(
    age: Age,
) {
    println!("age is {age}")
}

fn print_name_and_age(
    name: Name,
    age: Age,
) {
    println!("{name} is {age} years old!")
}

fn main() {
    let app = App {
        age: 20,
        name: "Max".into(),
    };
    
    app.run(print_name);
    app.run(print_age);
    app.run(print_name_and_age);
}

// implementation

trait Param: Sized + Clone + 'static { }

#[derive(Debug, Clone)]
struct Name(String);

impl Param for Name { }

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone)]
struct Age(u32);

impl Param for Age { }

impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

trait AvailableParams {
    fn get_param<T: Param + 'static>(&self) -> Option<T>;
}

struct App {
    name: String,
    age: u32,
}

impl AvailableParams for App {
    fn get_param<T: Param + 'static>(&self) -> Option<T> {
        if let Some(res) = (Box::new(Age(self.age)) as Box<dyn Any + 'static>).downcast::<T>().ok() {
            return Some(*res)
        }

        if let Some(res) = (Box::new(Name(self.name.clone())) as Box<dyn Any + 'static>).downcast::<T>().ok() {
            return Some(*res)
        }

        return None
    }
}

trait ParamFunction<Out, Params> {
    fn run(&mut self, params: &impl AvailableParams) -> Option<Out>;
}

impl<Out, Func, F1: Param> ParamFunction<Out, F1> for Func
where Func:
    FnMut(F1) -> Out
{
    fn run(&mut self, params: &impl AvailableParams) -> Option<Out> {
        self(
            params.get_param::<F1>()?,
        ).into()
    }
}

impl<Out, Func, F1: Param, F2: Param> ParamFunction<Out, (F1, F2)> for Func
where Func:
    FnMut(F1, F2) -> Out
{
    fn run(&mut self, params: &impl AvailableParams) -> Option<Out> {
        self(
            params.get_param::<F1>()?,
            params.get_param::<F2>()?,
        ).into()
    }
}

impl App {
    pub fn run<Out, Params>(
        &self,
        mut f: impl ParamFunction<Out, Params>
    ) -> Out {
        return f.run(self).unwrap()
    }
}
