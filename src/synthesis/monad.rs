#[derive(Debug, Clone, PartialEq, Eq)]
struct StringMonad<T> {
    value: Vec<String>,
    result: T,
}

impl<T> StringMonad<T> {
    fn ret(result: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self {
            value: vec![format!("f called with: {}", result)],
            result,
        }
    }

    fn lift(self) -> T {
        self.result
    }

    fn bind<U, F>(self, f: F) -> StringMonad<U>
    where
        F: FnOnce(T) -> StringMonad<U> + 'static,
    {
        let mut new_monad = f(self.result);
        new_monad.value = [self.value, new_monad.value].concat();
        new_monad
    }

    fn empty() -> Self
    where
        T: Default,
    {
        Self {
            value: vec![],
            result: T::default(),
        }
    }
}

fn compose<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> StringMonad<C>
where
    F: FnOnce(A) -> StringMonad<B> + 'static,
    G: FnOnce(B) -> StringMonad<C> + 'static,
{
    move |x: A| f(x).bind(g)
}

macro_rules! bind_chain {
    ($monad:expr, $func:expr) => {
        $monad.bind($func)
    };
    ($monad:expr, $func:expr, $($rest:expr),+) => {
        bind_chain!($monad.bind($func), $($rest),+)
    };
}

macro_rules! compose_chain {
    ($first:expr) => {
        $first
    };
    ($first:expr, $($rest:expr),+) => {
        compose($first, compose_chain!($($rest),+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monad_composition() {
        let f = |x: i32| StringMonad::ret(x * 2);
        let g = |x: i32| StringMonad::ret(x + 10);
        let composed_fn = compose(f, g);
        let compose_result = composed_fn(5);
        println!("{:?}", compose_result); // Output the final monad
        let monad_a = StringMonad::ret(10);
        let bind_result = monad_a.bind(|x| StringMonad::ret(x * 2));
        println!("{:?}", bind_result);
        assert_eq!(
            compose_result, bind_result,
            "Compose and bind should produce the same result."
        );
    }

    #[test]
    fn monad_composition_macro() {
        let f = |x: i32| StringMonad::ret(x * 2);
        let g = |x: i32| StringMonad::ret(x + 10);
        let h = |x: i32| StringMonad::ret(x - 1);

        let monad = StringMonad::ret(5);
        let bind_result = bind_chain!(monad, f, g, h);
        println!("Bind chain result: {:?}", bind_result);

        let composed_fn = compose_chain!(f, g, h);
        let compose_result = composed_fn(5);
        println!("Compose chain result: {:?}", compose_result);
    }
}
