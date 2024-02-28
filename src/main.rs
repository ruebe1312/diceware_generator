#![allow(non_snake_case)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rand::Rng;

fn main() {
    let app = MainWindow::new().unwrap();
    let app_weak = app.as_weak();
    app.on_generate_passphrase(move || {
        let app_u = app_weak.upgrade().unwrap();
        let pass_len: u8 = app_u.get_slv().try_into().unwrap();
        let phrase = generate_passphrase(pass_len);
        app_u.set_output_phrase(phrase.into());
    });
    
    app.run().unwrap();
}

fn generate_passphrase(phrase_length: u8) -> String {
    let filepath = "diceware.txt";
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {:?}", filepath),
    };

    let reader = BufReader::new(file);
    let mut wordlist: Vec<String> = Vec::new();
    for line in reader.lines() {
        wordlist.push(match line {
            Ok(line) => line,
            Err(_) => panic!("Fucked up at line"),
        });
    }

    let mut output: String = String::new();
    let mut rng = rand::thread_rng();
    let wordlist_length: usize = wordlist.len();
    for i in 0..phrase_length {
        let random_value: usize = rng.gen_range(0..wordlist_length);
        if i > 0 {
        output.push_str(" ");
        }
        let word:String = wordlist.get(random_value).expect("Fucked the Conversion").to_lowercase();
        let splits = word.split_whitespace();
        output.push_str(&splits.last().expect("Fucked the Conversion"));
    }

    return output;
}

slint::slint! {
    import { Slider, VerticalBox, HorizontalBox, TextEdit, GroupBox } from "std-widgets.slint";

    export global Palette {
        out property<color> c0: #0D1821;
        out property<color> c1: #344966;
        out property<color> c2: #E6AACE;
        out property<color> c3: #F0F4EF;
        out property<color> c4: #BFCC94;
    }

    export global Values {
        in-out property<int> slider_value: 10;
    }

    component length_slider inherits Rectangle {
        width: 300px;
        height: 30px;
        padding: 10px;
        border-radius: 4px;
        background: Palette.c1;

        HorizontalLayout{
            alignment: center;
            padding: 5px;
            spacing: 10px;
        Slider {
            width: parent.width - 40px;
            height: parent.height;
            minimum: 6;
            maximum: 26;
            value: Values.slider_value;
            changed => {Values.slider_value = self.value}
                }
        Text {
            width: 60px;
            height: parent.height;
            font-size: 15px;
            text: Values.slider_value;
            color: Palette.c3;
        }
    }
    }

    component genrate_button inherits TouchArea {
        width: 180px;
        height: 30px;

        in property <string> text <=> i-text.text;
    
        i-container := Rectangle {
            border-radius: 4px;
            background: Palette.c1;
    
            HorizontalLayout {  
                padding: 4px;
    
                i-text := Text {
                    color: #ffffff;
                    horizontal-alignment: center;
                    font-size: 14px;
                }
            }
    
            animate background { duration: 500ms;  }
        }
    
        states [  
            pressed when root.pressed: {
                i-container.background: Palette.c2;
                i-text.color: #000000;
            }
        ]
    }
    

    component MainBox inherits Rectangle {
        width: 800px;
        height: 600px;
        background: Palette.c0;
    }

    export component MainWindow inherits Window {
        width: 800px;
        height: 600px;

        in-out property<int> slv: Values.slider_value;
        in-out property<string> output_phrase: "All Credentials Are Beschützt";

        callback generate_passphrase;

        MainBox {
            HorizontalLayout {
                alignment: center;
            VerticalLayout {
                alignment: center;
                spacing: 10px;
                // Passphrase
                TextEdit {
                    font-size: 20px;
                    width: 600px;
                    height: 200px;
                    text: output_phrase;
                    read-only: true;
                    horizontal-alignment: center;
                }
                HorizontalLayout {
                    alignment: center;
                    spacing: 30px;
                    // Slider Für länge der Passphrase
                    length_slider {}
                    // Button um neue phrase zu generieren
                    genrate_button {  
                        text: "Neue Passphrase";
                        clicked => {root.generate_passphrase()}
                        }
                    }
                }
            }
        }
    }
}