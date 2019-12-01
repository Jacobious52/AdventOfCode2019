pub mod problem;
pub mod util;

#[macro_export]
macro_rules! part {
    ($p:expr) => (
        println!("{}", 
            format!("🥇 Part {}", $p)
                .blue()
                .bold()
                .underline()
        );
    ); 
}

#[macro_export]
macro_rules! run {
    ($($t:expr => $e:expr => $p:ident), *) => (
        $( 
            println!("{}", $p.test($t, $e));
        )*
    );
    ($i:expr => $p:ident) => (
        println!("{}", "🎁 Result".yellow().bold());
        println!("{}", $p.run($i));
    );
}
