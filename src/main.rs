use arboard::ImageData;
use qrcode::render::svg;
use qrcode::QrCode;
use slint::Image;
use std::borrow::Cow;

slint::slint! {
    import { VerticalBox } from "std-widgets.slint";
    import { TextEdit } from "std-widgets.slint";
    export component QRCodeGeneratorUI inherits Window {
        // Set the title of the window
        title: "QR Code Generator";
        width: 400px;
        height: 400px;

        in property <image> svg_img;
        out property <string> url;

        callback url-changed(string);

        VerticalBox {
            width: 100%;
            height: 100%;
            spacing: 2rem;

            Text {
                    text: "Input URL:";
                    height: 30px;
                }
            TextEdit {
                height: 4rem;
                text: url;
                edited(val) => {
                    root.url-changed(val);
                }
            }

            Image {
            width: 80%;
            vertical-alignment: bottom;
            horizontal-alignment: center;
            source: svg_img;
            }
        }
    }
}

fn img_from_url_callback(url: String) -> Image {
    let code = QrCode::new(url).unwrap();
    let svg_string_data = code.render::<svg::Color>().build();
    let buffer = svg_string_data.as_bytes();
    Image::load_from_svg_data(buffer).unwrap()
}

fn img_to_clipboard_callback(img: Image) {
    let img = img.to_rgba8().unwrap();
    let width = img.width();
    let height = img.height();

    let r = img
        .as_slice()
        .iter()
        .map(|pixel| pixel.r)
        .collect::<Vec<u8>>();
    let g = img
        .as_slice()
        .iter()
        .map(|pixel| pixel.g)
        .collect::<Vec<u8>>();
    let b = img
        .as_slice()
        .iter()
        .map(|pixel| pixel.b)
        .collect::<Vec<u8>>();
    let a = img
        .as_slice()
        .iter()
        .map(|pixel| pixel.a)
        .collect::<Vec<u8>>();

    let rgba_arr = r
        .iter()
        .zip(g.iter())
        .zip(b.iter())
        .zip(a.iter())
        .map(|(((r, g), b), a)| vec![*r, *g, *b, *a])
        .flatten()
        .collect::<Vec<u8>>();

    let img = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::from(rgba_arr),
    };

    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_image(img).unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = QRCodeGeneratorUI::new()?;

    ui.on_url_changed({
        let ui_handle = ui.as_weak();

        move |url| {
            let ui = ui_handle.unwrap();
            let img = img_from_url_callback(url.to_string());
            ui.set_svg_img(img.clone());
            img_to_clipboard_callback(img);
        }
    });

    ui.run()
}
