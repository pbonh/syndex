mod datatype;
mod egglog_names;
pub mod facts;
mod inst;
pub mod rules;
pub mod sorts;
mod unit;
pub use unit::LLHDEgglogFacts;
pub mod llhd;

use egglog::ast::Command;

type EgglogCommandList = Vec<Command>;

#[cfg(test)]
mod tests {

    use bon::builder;

    #[builder]
    fn greet(name: &str, level: Option<u32>) -> String {
        let level = level.unwrap_or(0);

        format!("Hello {name}! Your level is {level}")
    }

    #[test]
    fn bon_ordered_builder_free_function() {
        let greeting = greet()
            .level(24) // <- setting `level` is optional, we could omit it
            .name("Bon")
            .call();

        assert_eq!(greeting, "Hello Bon! Your level is 24");
    }
}
