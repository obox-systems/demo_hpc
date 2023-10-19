use cli::{executor_form, grammar_form};

fn main() {
    let ca = wca::CommandsAggregator::former()
        .grammar(grammar_form())
        .executor(executor_form())
        .build();

    let args = std::env::args().skip(1).collect::<Vec<String>>();

    ca.perform(args.join(" ").as_str()).unwrap();
}
