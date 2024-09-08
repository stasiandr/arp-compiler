use arp_types::sources::Source;

use crate::build_multiple_sources;



#[test]
fn external_import() {
    let sources = [
        Source::new_inline("Main.arp", "
        from Printer import Printer
        
        fn main() { 
            let line = Printer.read_line();
            Printer.print(line);
        }"),

        Source::new_inline("Printer.arp", "
        from extern System.Console.dll import System.Console
        
        class Printer { }

        impl Printer {
            fn read_line() -> string {
                let line = System.Console.ReadLine();
                return line;
            }

            fn print(input: string) -> string {
                System.Console.WriteLine(input);
            }
        }
        "),
    ];

    let ast = build_multiple_sources(&sources).unwrap();

    dbg!(ast);
}