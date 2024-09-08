use arp_types::sources::Source;
use crate::build_multiple_sources;

#[test]
fn external_import() {
    let sources = [
        Source::new_inline("Main.arp", "
        from extern System.Console.dll import System.Console 
        
        fn main() { 
            let x = System.Console.ReadLine(); 
            System.Console.WriteLine(x); 
        }"),
    ];

    let ast = build_multiple_sources(&sources).unwrap();

    dbg!(ast);
}