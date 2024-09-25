use std::fs;
use std::io::{self, Write};
use walkdir::WalkDir;
use regex::Regex;

fn main() -> io::Result<()> {
    println!(
        r#"
                     ..    d$P              $$      `$b
                    z$"   $$F               4$$      $$L
                    $$   4$$                 $$.     4$$    ,z$P
                    $$   $$'                 $$F      $$   $$$P
                   $$$   $$                  $$f      $$   ""`
                  $'$$; 4$F      .,_         $$'      $$
                .$' ?$L 4$'   .d$" `?    zee $$   ,ec $F  d$F z$$   ,ce,.
              .d$ee. $$ 4$'  d$"   z$  $$"  .$f.d$"  4$  4$$ 4$$P z$P?$$$
             d$" "?$$d$,`$  $$F   z$f,$$    $$.$$    $P  $$% $$$4$"   4$$
.$"%.     ,p$"        $$ $ J$$  z$$$ $$"  .$$ $$"  .$$C 4$P  $$$"     $$f
`$.     ,d$b****q,     $.$ $$$$$P $$.$$b.$P4$ $$L.$P4$F $P  4$P     .$$"
 `?$$g$P"        "     `b' `??"   "?"^?F"   $$`?PF"  $$ "   P'     eF
        "#
    );

    print!("Entrez le chemin du dossier : ");
    io::stdout().flush()?; 

    let mut folder_path = String::new();
    io::stdin().read_line(&mut folder_path)?;
    let folder_path = folder_path.trim();

    print!("Entrez le texte à rechercher : ");
    io::stdout().flush()?;
    
    let mut search_text = String::new();
    io::stdin().read_line(&mut search_text)?;
    let search_text = search_text.trim();

    let regex = Regex::new(search_text).unwrap();

    for entry in WalkDir::new(folder_path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let content = fs::read_to_string(entry.path());
            match content {
                Ok(text) => {
                    if regex.is_match(&text) {
                        println!("Trouvé dans : {:?}", entry.path());
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    Ok(())
}
