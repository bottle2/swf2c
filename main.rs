// swf2c -- Generate C code that plays Flash animations
//
// Copyright (C) 2025, 2026 Bento Borges Schirmer
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::io::Write;

#[derive(Clone)]
enum Object {
    Shape,
    Sprite,
}

fn main() {
    macro_rules! fuckprint {
        ($($x:tt)*) => {
            write!(std::io::stdout().lock(), $($x)*).unwrap_or_else(|_| { std::process::exit(0)})
        }
    }
    macro_rules! efuckprint {
        ($($x:tt)*) => {
            write!(std::io::stderr().lock(), $($x)*).unwrap_or_else(|_| { std::process::exit(0)})
        }
    }

    //let original:swf::Matrix = swf::Matrix{
    //    a: swf::Fixed16::from_f64(0.25),
    //    b: swf::Fixed16::from_f64(0.0),
    //    c: swf::Fixed16::from_f64(0.0),
    //    d: swf::Fixed16::from_f64(0.5),
    //    tx: swf::Twips::from_pixels(20.0),
    //    ty: swf::Twips::from_pixels(10.0)
    //};
    //let mut inv = original;
    //inv.invert();
    //let res = inv * original;
    //fuckprint!("{:#?}",original);
    //fuckprint!("{:#?}",inv);
    //std::process::exit(0);

    let die = || -> ! {
        efuckprint!("Usage: swf2c -c|-h|-s<n> <file>\n");
        std::process::exit(1);
    };

    let Ok(g) = getopt3::new(env::args().skip(1), "chs:") else {
        die();
    };
    let Ok(g) = getopt3::validate(g) else {
        die();
    };

    if g.arguments.len() != 1 {
        die();
    }

    //let is_trace = g.options.contains_key(&'t');
    let is_header = g.options.contains_key(&'h');
    let is_source = g.options.contains_key(&'c');
    let wants_stats = g.options.get(&'s');

    /*
    if let Some(str) = env::args().skip(1).next()
    {
        for c in str.chars() {
            match c {
                'c' => is_source = true,
                't' => is_trace = true,
                'h' => is_header = true,
                _ => die(),
            }
        }
    } */

    if wants_stats.is_none() && is_header == is_source {
        die();
    }

    //fuckprint!("hi baby\n");

    //let filename = if let Some(a) = g.arguments.first() {
    //    a
    //} else {
    //    die();
    //};

    //let filename = g.arguments.first().unwrap_or_else(||{die(); ""});
    let filename = if let Some(f) = g.arguments.first() {
        f
    } else {
        die();
    };

    macro_rules! soft_todo {
        () => {
            unsafe {
                static mut ONCE:bool = false;
                if !ONCE { efuckprint!("{}:{}:not implemented ({})\n", file!(), line!(), filename); }
                ONCE = true;
            }
        };
        ($($arg:tt)+) => {
            unsafe {
                static mut ONCE:bool = false;
                if !ONCE { efuckprint!("{}:{}:not implemented ({}): {}\n", file!(), line!(), filename, format!($($arg)+)); }
                ONCE = true;
            }
        };
    }

    let mut lowername = String::with_capacity(filename.len());
    let mut uppername = String::with_capacity(filename.len());

    for (i, c) in
        filename
            .as_bytes()
            .iter()
            .enumerate()
            .skip(if let Some(start) = filename.rfind('/') {
                start + 1
            } else {
                0
            })
    {
        match c {
            b'0'..=b'9' if i != 0 => {
                lowername.push(*c as char);
                uppername.push(*c as char);
            }
            b'a'..=b'z' => {
                lowername.push(*c as char);
                uppername.push((*c - b'a' + b'A') as char);
            }
            b'A'..=b'Z' => {
                lowername.push((*c - b'A' + b'a') as char);
                uppername.push(*c as char);
            }
            b'_' | b'.' => {
                lowername.push('_');
                uppername.push('_');
            }
            _ => {
                efuckprint!("filename wieldn't a C identifier\n");
                std::process::exit(1);
            }
        }
    }

    if filename.ends_with(".swf") {
        lowername.truncate(lowername.len() - 4);
        uppername.truncate(uppername.len() - 4);
    }

    let file = if let Ok(f) = File::open(filename) {
        f
    } else {
        efuckprint!("couldn't open file {}\n", filename);
        std::process::exit(1);
    };

    let reader = BufReader::new(file);
    let swf_buf = swf::decompress_swf(reader).unwrap();
    let swf = swf::parse_swf(&swf_buf).unwrap();
    //trace!("The SWF is version {}.\n", swf.header.version());
    //trace!("The SWF has {} tags.\n", swf.tags.len());

    let le_framerate = swf.header.frame_rate();
    let le_n_frame = swf.header.num_frames();
    let pixel_width = (swf.header.stage_size().x_max - swf.header.stage_size().x_min).to_pixels();
    let pixel_height = (swf.header.stage_size().y_max - swf.header.stage_size().y_min).to_pixels();

    if let Some(which) = wants_stats {
        match which.as_str() {
            "1" => {
                let mut correct_framerate = format!("{}", le_framerate);
                if correct_framerate.contains(".") {
                    correct_framerate = correct_framerate
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .replacen(".", "\\&,", 1);
                }
                fuckprint!(
                    "{}\t{}x{}\t{}",
                    correct_framerate,
                    pixel_width,
                    pixel_height,
                    swf.header.num_frames()
                );
                std::process::exit(0);
            }
            "2" => {
                fuckprint!("{}\t{}", swf.header.version(), swf.tags.len());
                std::process::exit(0);
            }
            "3" => {
                fuckprint!("{}p {}p", pixel_width, pixel_height);
                std::process::exit(0);
            }
            _ => die(),
        }
    }

    if is_header {
        fuckprint!(
            r#"#ifndef {}_H
#define {}_H

#ifdef __cplusplus
extern "C" {{
#endif

#define {}_framerate {}
enum {{
    {}_n_frame   = {},
    {}_width     = {},
    {}_height    = {}
}};

#ifdef FEAT_PLUTOVG
void {}_init_plutovg(void);
void {}_free_plutovg(void);
void {}_render_sdl_plutovg(void *pixels, int pitch, int frame);
#endif

#ifdef FEAT_HTML5
void {}_render_html5(__externref_t CanvasRenderingContext2D, int frame);
void {}_render_sdl_html5(__externref_t CanvasRenderingContext2D, int frame, void *pixels, int pitch);
#endif

#ifdef FEAT_CAIRO
#include <cairo.h>

void {}_init_cairo(void);
void {}_free_cairo(void);
void {}_render_cairo(cairo_t *cr, int frame);
#endif

#ifndef FEAT_NO_DATA
void {}_data(float *framerate, int *n_frame, int *width, int *height);
bool {}_background_color(unsigned char *r, unsigned char *g, unsigned char *b);
void {}_transform(double,double,double,double,double,double);
#endif

// TODO Discuss rendering cutscenes, pre-rendering sprites, ImageBitmap etc.
// TODO See Web Worker + OffscreenCanvasRenderingContext2D.
// TODO Document pixel format expected.
// TODO Explain how to copy HTML canvas data to e.g. texture.
// TODO Figure out parameter order
// TODO Figure out some naming convention <swf>_{{init,free,render}}[_<framework>]_<engine>[_<variant>]
// TODO Discuss thread safety and reentrancy

#ifdef __cplusplus
}}
#endif

#endif
"#,
            uppername,
            uppername,
            lowername,
            le_framerate,
            lowername,
            le_n_frame,
            lowername,
            pixel_width,
            lowername,
            pixel_height,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername,
            lowername
        );

        std::process::exit(0);
    }

    let mut shapes: Vec<swf::Shape> = Vec::new();
    let mut sprite_ids: Vec<u16> = Vec::new();
    let mut sprites: Vec<swf::Sprite> = Vec::new();
    let mut allowed: Vec<u16> = Vec::new();
    let encoding = swf::SwfStr::encoding_for_version(swf.header.version());
    let mut display_list: Vec<Option<(u16, Object, Option<swf::Matrix>)>> = Vec::new();

    enum Frame {
        DisplayList(Vec<Option<(u16, Object, Option<swf::Matrix>)>>),
    }

    let mut display_lists: Vec<Frame> = Vec::new();

    let mut n_clipping = 0;
    let mut n_blending = 0;

    let mut background: Option<swf::Color> = None;

    for tag in swf.tags {
        match tag {
            swf::Tag::ExportAssets(exported_assets) => soft_todo!(),
            swf::Tag::ScriptLimits {
                max_recursion_depth,
                timeout_in_seconds,
            } => soft_todo!(),
            swf::Tag::ShowFrame => {
                display_lists.push(Frame::DisplayList(display_list.clone()));
            }
            swf::Tag::Protect(None) =>
            /*trace!("no protect\n")*/
            {
                ()
            }
            swf::Tag::Protect(Some(swf_str)) =>
            /*trace!("protect is {}\n", swf_str.to_string_lossy(encoding))*/
            {
                ()
            }
            swf::Tag::CsmTextSettings(csm_text_settings) => soft_todo!(),
            swf::Tag::DebugId(_) => soft_todo!(),
            swf::Tag::DefineBinaryData(define_binary_data) => soft_todo!(),
            swf::Tag::DefineBits { id, jpeg_data } => soft_todo!(),
            swf::Tag::DefineBitsJpeg2 { id, jpeg_data } => soft_todo!(),
            swf::Tag::DefineBitsJpeg3(define_bits_jpeg3) => soft_todo!(),
            swf::Tag::DefineBitsLossless(define_bits_lossless) => soft_todo!(),
            swf::Tag::DefineButton(button) => soft_todo!(),
            swf::Tag::DefineButton2(button) => soft_todo!(),
            swf::Tag::DefineButtonColorTransform(button_color_transform) => soft_todo!(),
            swf::Tag::DefineButtonSound(button_sounds) => soft_todo!(),
            swf::Tag::DefineEditText(edit_text) => soft_todo!(),
            swf::Tag::DefineFont(font_v1) => soft_todo!(),
            swf::Tag::DefineFont2(font) => soft_todo!(),
            swf::Tag::DefineFont4(font4) => soft_todo!(),
            swf::Tag::DefineFontAlignZones {
                id,
                thickness,
                zones,
            } => soft_todo!(),
            swf::Tag::DefineFontInfo(font_info) => {
                /*trace!("define font info. no {}\n", font_info.code_table.len());*/
                ()
            }
            swf::Tag::DefineFontName {
                id,
                name,
                copyright_info,
            } => soft_todo!(),
            swf::Tag::DefineMorphShape(ms) => {
                allowed.push(ms.id);
                shapes.push(swf::Shape {
                    version: 99,
                    id: ms.id,
                    shape_bounds: swf::Rectangle::INVALID,
                    edge_bounds: swf::Rectangle::INVALID,
                    flags: swf::ShapeFlag::empty(),
                    styles: swf::ShapeStyles {
                        fill_styles: ms.end.fill_styles,
                        line_styles: ms.end.line_styles,
                    },
                    shape: ms.end.shape,
                });
            }
            swf::Tag::DefineScalingGrid { id, splitter_rect } => soft_todo!(),
            swf::Tag::DefineShape(shape) => {
                /*trace!("defshape {}\n", shape.id);*/
                allowed.push(shape.id);
                shapes.push(shape)
            }
            swf::Tag::DefineSound(sound) => soft_todo!(),
            swf::Tag::DefineSprite(sprite) => {
                allowed.push(sprite.id);
                /*trace!("sprite {} with {} tags and {} frames\n", sprite.id, sprite.tags.len(), sprite.num_frames);*/
                sprite_ids.push(sprite.id);
                sprites.push(sprite);
            }
            swf::Tag::DefineText(text) => soft_todo!(),
            swf::Tag::DefineText2(text) => soft_todo!(),
            swf::Tag::DefineVideoStream(define_video_stream) => soft_todo!(),
            swf::Tag::DoAbc(items) => soft_todo!(),
            swf::Tag::DoAbc2(do_abc2) => soft_todo!(),
            swf::Tag::DoAction(items) => { /*trace!("some {} actions\n", items.len());*/ }
            swf::Tag::DoInitAction { id, action_data } => soft_todo!(),
            swf::Tag::EnableDebugger(swf_str) => soft_todo!(),
            swf::Tag::EnableTelemetry { password_hash } => soft_todo!(),
            swf::Tag::End => soft_todo!(),
            swf::Tag::Metadata(swf_str) => soft_todo!(),
            swf::Tag::ImportAssets { url, imports } => soft_todo!(),
            swf::Tag::JpegTables(items) => soft_todo!(),
            swf::Tag::NameCharacter(name_character) => soft_todo!(),
            swf::Tag::SetBackgroundColor(color) => {
                if background.is_none() {
                    background = Some(color);
                }
            }
            swf::Tag::SetTabIndex { depth, tab_index } => soft_todo!(),
            swf::Tag::SoundStreamBlock(items) => soft_todo!(),
            swf::Tag::SoundStreamHead(sound_stream_head) => soft_todo!(),
            swf::Tag::SoundStreamHead2(sound_stream_head) => soft_todo!(),
            swf::Tag::StartSound(start_sound) => soft_todo!(),
            swf::Tag::StartSound2 {
                class_name,
                sound_info,
            } => soft_todo!(),
            swf::Tag::SymbolClass(symbol_class_links) => soft_todo!(),
            swf::Tag::PlaceObject(place_object) => {
                sprite_ids.sort();
                n_clipping += place_object.clip_depth.is_some() as i32;
                n_blending += place_object.blend_mode.is_some() as i32;
                match place_object.action {
                    swf::PlaceObjectAction::Place(id) => {
                        /*trace!("placeobj {}\n", id);*/
                        if display_list.len() <= place_object.depth as usize {
                            display_list.resize(place_object.depth as usize + 1, None);
                        }
                        let orig_matrix = if display_list[place_object.depth as usize].is_some() {
                            display_list[place_object.depth as usize].clone().unwrap().2
                        } else {
                            None
                        };
                        display_list[place_object.depth as usize] = Some((
                            id,
                            if sprite_ids.binary_search(&id).is_ok() {
                                Object::Sprite
                            } else {
                                Object::Shape
                            },
                            if place_object.matrix.is_some() {
                                place_object.matrix
                            } else {
                                orig_matrix
                            },
                        ));
                    }
                    swf::PlaceObjectAction::Modify => {
                        /*trace!("modifyobj at depth {}\n", place_object.depth);*/
                        let orig_matrix =
                            display_list[place_object.depth as usize].clone().unwrap().2;
                        display_list[place_object.depth as usize] = Some((
                            display_list[place_object.depth as usize].clone().unwrap().0,
                            display_list[place_object.depth as usize].clone().unwrap().1,
                            if place_object.matrix.is_some() {
                                place_object.matrix
                            } else {
                                orig_matrix
                            },
                        ))
                    }
                    swf::PlaceObjectAction::Replace(id) => {
                        /*trace!("replace id {}\n", id);*/
                        let orig_matrix =
                            display_list[place_object.depth as usize].clone().unwrap().2;
                        display_list[place_object.depth as usize] = Some((
                            id,
                            if sprite_ids.binary_search(&id).is_ok() {
                                Object::Sprite
                            } else {
                                Object::Shape
                            },
                            if place_object.matrix.is_some() {
                                place_object.matrix
                            } else {
                                orig_matrix
                            },
                        ));
                    }
                };
                if let Some(name) = place_object.name {
                    /*trace!("Got name {} when placing\n", name.to_string_lossy(encoding))*/
                    ()
                };
                if let Some(ratio) = place_object.ratio {
                    /*trace!("Got ratio {}\n", ratio)*/
                    ()
                }
                if let Some(actions) = place_object.clip_actions {
                    for ca in actions.iter() {
                        // soft_todo The action data is in swf::OpCode
                        /*trace!("clip action {} with {} data\n", ca.events.bits(), ca.action_data.len())*/
                    }
                }
            }
            swf::Tag::RemoveObject(remove_object) => {
                /*trace!("remove\n");*/
                display_list[remove_object.depth as usize] = None;
            }
            swf::Tag::VideoFrame(video_frame) => soft_todo!(),
            swf::Tag::FileAttributes(file_attributes) => soft_todo!(),
            swf::Tag::FrameLabel(frame_label) => {
                /*trace!("frame label: {} (is anchor: {})\n", frame_label.label.to_str_lossy(encoding), frame_label.is_anchor);*/
                // TODO What is the current frame? the previous rendered or the to-be-rendered?
                ()
            }
            swf::Tag::DefineSceneAndFrameLabelData(define_scene_and_frame_label_data) => {
                soft_todo!()
            }
            swf::Tag::ProductInfo(product_info) => soft_todo!(),
            swf::Tag::Unknown { tag_code, data } => soft_todo!(),
        }
    }
    fuckprint!("\n");

    #[derive(PartialEq, Eq)]
    struct Godamn(swf::FillStyle);

    // TODO Should make into pull request for Ruffle
    impl Hash for Godamn {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            match &self.0 {
                swf::FillStyle::Color(color) => color.hash(state),
                swf::FillStyle::LinearGradient(gradient) => gradient.hash(state),
                swf::FillStyle::RadialGradient(gradient) => gradient.hash(state),
                swf::FillStyle::FocalGradient {
                    gradient,
                    focal_point,
                } => {
                    gradient.hash(state);
                    focal_point.hash(state)
                }
                swf::FillStyle::Bitmap {
                    id,
                    matrix,
                    is_smoothed,
                    is_repeating,
                } => {
                    id.hash(state);
                    matrix.hash(state);
                    is_smoothed.hash(state);
                    is_repeating.hash(state);
                }
            }
        }
    }
    impl From<swf::FillStyle> for Godamn {
        fn from(value: swf::FillStyle) -> Self {
            Self(value)
        }
    }

    let mut fs2id: HashMap<Godamn, usize> = HashMap::new();

    fn maybe_print_fillstyle(dict: &mut HashMap<Godamn, usize>, fs: swf::FillStyle) {
        let key = Godamn::from(fs.clone());
        if !dict.contains_key(&key) {
            fuckprint!(" \\\nX({},", dict.len());
            enum Death {
                Linear,
                Radial,
                Focal(swf::Fixed8),
            }
            struct Suicide {
                typ: Death,
                fuck: swf::Gradient,
            }
            let mut suicide: Option<Suicide> = None;

            match key.0.clone() {
                swf::FillStyle::Color(color) => {
                    fuckprint!("S({},{},{},{})", color.r, color.g, color.b, color.a)
                }
                swf::FillStyle::LinearGradient(lg) => {
                    suicide = Some(Suicide {
                        typ: Death::Linear,
                        fuck: lg,
                    })
                }
                swf::FillStyle::RadialGradient(rg) => {
                    suicide = Some(Suicide {
                        typ: Death::Radial,
                        fuck: rg,
                    })
                }
                swf::FillStyle::FocalGradient {
                    gradient,
                    focal_point,
                } => {
                    suicide = Some(Suicide {
                        typ: Death::Focal(focal_point),
                        fuck: gradient,
                    })
                }
                swf::FillStyle::Bitmap {
                    id,
                    matrix,
                    is_smoothed,
                    is_repeating,
                } => {
                    fuckprint!(
                        "B({},SMOOTH_{},REPEAT_{},M({},{},{},{},{},{}))",
                        id,
                        if is_smoothed { "YES" } else { "NO" },
                        if is_repeating { "YES" } else { "NO" },
                        matrix.a.to_f64(),
                        matrix.b.to_f64(),
                        matrix.c.to_f64(),
                        matrix.d.to_f64(),
                        matrix.tx.to_pixels(),
                        matrix.ty.to_pixels()
                    )
                }
            }

            if let Some(Suicide {
                typ,
                fuck:
                    swf::Gradient {
                        matrix,
                        spread,
                        interpolation,
                        records,
                    },
            }) = suicide
            {
                fuckprint!(
                    "G({},{},{},",
                    match typ {
                        Death::Linear => "L".to_string(),
                        Death::Radial => "R".to_string(),
                        Death::Focal(f) => format!("F({}", f),
                    },
                    match spread {
                        swf::GradientSpread::Pad => "PAD",
                        swf::GradientSpread::Reflect => "REFLECT",
                        swf::GradientSpread::Repeat => "REPEAT",
                    },
                    match interpolation {
                        swf::GradientInterpolation::Rgb => "NRGB",
                        swf::GradientInterpolation::LinearRgb => "LRGB",
                    }
                );

                if records.is_empty() {
                    efuckprint!("no gradient records\n");
                    std::process::exit(1);
                } else {
                    for gr in records {
                        fuckprint!(
                            "GR({},{},{},{},{})",
                            gr.ratio,
                            gr.color.r,
                            gr.color.g,
                            gr.color.b,
                            gr.color.a
                        )
                    }
                }

                //let scale = swf::Matrix::scale(swf::Fixed16::from_f64(1.0/20.0), swf::Fixed16::from_f64(1.0/20.0));

                //let mut matrix = matrix;
                //matrix.invert();
                //matrix = matrix * scale;

                //let border = swf::Twips::new(16384);
                //let p0: swf::Point<swf::Twips> = swf::Point::new(-border, -border);
                //let p1: swf::Point<swf::Twips> = swf::Point::new(border, border);
                //let p0 = matrix * p0;
                //let p1 = matrix * p1;
                //let center_x = (p1.x + p0.x) / 2;
                //let center_y = (p1.y + p0.y) / 2;
                //let go_origin = swf::Matrix::translate(-center_x, -center_y);
                //let factor = swf::Fixed16::from_f64(1.0/20.0);
                //let scale = swf::Matrix::scale(factor, factor);
                //let go_back = swf::Matrix::translate(center_x/20, center_y/20);
                ////let mut matrix = matrix * (go_back * (scale * go_origin));
                ////let mut matrix = ((matrix * go_back) * scale) * go_origin;
                //let mut matrix = go_origin * scale * go_back * matrix;

                //matrix.invert();

                // TODO BROKEN!!!
                fuckprint!(
                    ",M({},{},{},{},{},{}))",
                    matrix.a.to_f64(),
                    matrix.b.to_f64(),
                    matrix.c.to_f64(),
                    matrix.d.to_f64(),
                    matrix.tx.get(),
                    matrix.ty.get()
                )
            }

            fuckprint!(")");

            dict.insert(key, dict.len());
        }
    }

    fuckprint!("\n#define FILL_XS");
    for s in shapes.iter() {
        for fs in s.styles.fill_styles.iter() {
            maybe_print_fillstyle(&mut fs2id, fs.clone());
        }
        for ls in s.styles.line_styles.iter() {
            maybe_print_fillstyle(&mut fs2id, ls.fill_style().clone());
        }
        for r in s.shape.iter() {
            if let swf::ShapeRecord::StyleChange(c) = r {
                if let Some(n) = &c.new_styles {
                    for fs in n.fill_styles.iter() {
                        maybe_print_fillstyle(&mut fs2id, fs.clone());
                    }
                    for ls in s.styles.line_styles.iter() {
                        maybe_print_fillstyle(&mut fs2id, ls.fill_style().clone());
                    }
                }
            }
        }
    }

    fuckprint!("\n#define SHAPE_XS");

    // Filled areas
    #[derive(Clone)]
    struct Aresta {
        inicio: swf::Point<swf::Twips>,
        fim: swf::Point<swf::Twips>,
        controle: Option<swf::Point<swf::Twips>>,
    }

    impl Aresta {
        fn new(
            inicio: swf::Point<swf::Twips>,
            fim: swf::Point<swf::Twips>,
            controle: Option<swf::Point<swf::Twips>>,
        ) -> Self {
            Self {
                inicio,
                fim,
                controle,
            }
        }
    }

    struct Shit {
        n1: swf::Twips,
        n2: swf::Twips,
    }
    impl From<swf::Point<swf::Twips>> for Shit {
        fn from(value: swf::Point<swf::Twips>) -> Self {
            Self {
                n1: value.x,
                n2: value.y,
            }
        }
    }
    impl From<swf::PointDelta<swf::Twips>> for Shit {
        fn from(value: swf::PointDelta<swf::Twips>) -> Self {
            Self {
                n1: value.dx,
                n2: value.dy,
            }
        }
    }

    impl fmt::Display for Shit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{},{}", self.n1.to_pixels(), self.n2.to_pixels())
        }
    }

    fn poop(
        from: swf::Point<swf::Twips>,
        controle: Option<swf::Point<swf::Twips>>,
        dest: swf::Point<swf::Twips>,
    ) {
        if let Some(ctrl) = controle {
            fuckprint!(
                " \\\n  B({},{},{})",
                Shit::from(from),
                Shit::from(ctrl),
                Shit::from(dest)
            )
        } else {
            fuckprint!(" \\\n  L({})", Shit::from(dest))
        }
    }

    fn dump2(a: &mut Vec<Aresta>, fill_id: usize, id: usize, width: Option<f64>) {
        if a.is_empty() {
            return;
        }
        if let Some(w) = width {
            fuckprint!(" \\\n S({},{},{},", id, fill_id, w);
        } else {
            fuckprint!(" \\\n A({},{},", id, fill_id);
        }

        // Idiotic brute force, who cares, fuck

        while !a.is_empty() {
            let mut curr = a[0].fim;

            fuckprint!(" \\\n  M({})", Shit::from(a[0].inicio));

            poop(a[0].inicio, a[0].controle, curr);
            a.remove(0);

            loop {
                let mut algum = false;
                a.retain(|aresta| {
                    if aresta.inicio == curr {
                        curr = aresta.fim;
                        poop(aresta.inicio, aresta.controle, curr);
                        algum = true;
                        false
                    } else if aresta.fim == curr {
                        curr = aresta.inicio;
                        poop(aresta.fim, aresta.controle, curr);
                        algum = true;
                        false
                    } else {
                        true
                    }
                });
                if !algum {
                    if width.is_none() {
                        fuckprint!(" \\\n  C");
                    }
                    break;
                }
            }
        }

        fuckprint!(")");
    }

    fn dump(
        areas: &mut [Vec<Aresta>],
        strokes: &mut [Vec<Aresta>],
        styles: &swf::ShapeStyles,
        fs2id: &HashMap<Godamn, usize>,
        id: usize,
    ) {
        for (i, a) in areas.iter_mut().enumerate() {
            dump2(
                a,
                *fs2id
                    .get(&Godamn::from(styles.fill_styles.get(i).unwrap().clone()))
                    .unwrap(),
                id + i,
                None,
            );
        }
        for (i, s) in strokes.iter_mut().enumerate() {
            dump2(
                s,
                *fs2id
                    .get(&Godamn::from(
                        styles.line_styles.get(i).unwrap().fill_style().clone(),
                    ))
                    .unwrap(),
                id + i + areas.len(),
                Some(styles.line_styles.get(i).unwrap().width().to_pixels()),
            );
        }
    }

    let mut n_path: usize = 0;
    for s in shapes.iter() {
        fuckprint!(" \\\nX({},", s.id);
        let mut areas: Vec<Vec<Aresta>> = Vec::new();
        let mut strokes: Vec<Vec<Aresta>> = Vec::new();
        areas.resize_with(s.styles.fill_styles.len(), || Vec::new());
        strokes.resize_with(s.styles.line_styles.len(), || Vec::new());

        let mut styles = &s.styles;

        let mut cursor: swf::Point<swf::Twips>;
        let mut fill_style0: u32 = 0;
        let mut fill_style1: u32 = 0;
        let mut line_style: u32 = 0;

        if let Some(swf::ShapeRecord::StyleChange(sc)) = &s.shape.first() {
            if let Some(to) = sc.move_to {
                cursor = to;
            } else {
                cursor = swf::Point::new(swf::Twips::new(0), swf::Twips::new(0));
                efuckprint!("chid {} no first move\n", s.id);
            }
            if let Some(fs0) = sc.fill_style_0 {
                fill_style0 = fs0;
            }
            if let Some(fs1) = sc.fill_style_1 {
                fill_style1 = fs1;
            }
            if let Some(ls) = sc.line_style {
                line_style = ls;
            }
            if let Some(ns) = &sc.new_styles {
                areas.resize_with(ns.fill_styles.len(), || Vec::new());
                strokes.resize_with(ns.line_styles.len(), || Vec::new());
                styles = ns;
            }
        } else {
            efuckprint!("chid {} has wrong start tag\n", s.id);
            std::process::exit(1);
        }

        for sr in s.shape.iter().skip(1) {
            match sr {
                swf::ShapeRecord::StyleChange(style_change_data) => {
                    if let Some(to) = style_change_data.move_to {
                        cursor = to;
                    }
                    if let Some(fs0) = style_change_data.fill_style_0 {
                        fill_style0 = fs0;
                    }
                    if let Some(fs1) = style_change_data.fill_style_1 {
                        fill_style1 = fs1;
                    }
                    if let Some(ls) = style_change_data.line_style {
                        line_style = ls;
                    }
                    if let Some(ns) = &style_change_data.new_styles {
                        dump(&mut areas, &mut strokes, styles, &fs2id, n_path);
                        n_path += areas.len() + strokes.len();
                        areas.clear();
                        areas.resize_with(ns.fill_styles.len(), || Vec::new());
                        strokes.clear();
                        strokes.resize_with(ns.line_styles.len(), || Vec::new());
                        styles = ns;
                    }
                }
                swf::ShapeRecord::StraightEdge { delta } => {
                    let aresta = Aresta::new(cursor, cursor + *delta, None);
                    if fill_style0 != fill_style1 {
                        if fill_style0 > 0 {
                            areas[fill_style0 as usize - 1].push(aresta.clone());
                        }
                        if fill_style1 > 0 {
                            areas[fill_style1 as usize - 1].push(aresta.clone());
                        }
                    }
                    if line_style > 0 {
                        strokes[line_style as usize - 1].push(aresta);
                    }
                    cursor += *delta;
                }
                swf::ShapeRecord::CurvedEdge {
                    control_delta,
                    anchor_delta,
                } => {
                    let aresta = Aresta::new(
                        cursor,
                        cursor + *control_delta + *anchor_delta,
                        Some(cursor + *control_delta),
                    );
                    if fill_style0 != fill_style1 {
                        if fill_style0 > 0 {
                            areas[fill_style0 as usize - 1].push(aresta.clone());
                        }
                        if fill_style1 > 0 {
                            areas[fill_style1 as usize - 1].push(aresta.clone());
                        }
                    }
                    if line_style > 0 {
                        strokes[line_style as usize - 1].push(aresta);
                    }
                    cursor += *control_delta;
                    cursor += *anchor_delta;
                }
            }
        }
        dump(&mut areas, &mut strokes, styles, &fs2id, n_path);
        n_path += areas.len() + strokes.len();
        fuckprint!(")")
    }

    sprite_ids.sort();

    allowed.sort();
    fuckprint!("\n#define SPRITE_XS \\\n");
    let mut here_we_go_couting = 0;
    for s in sprites.iter() {
        fuckprint!(" SDEF({}, {}, \\\n", s.id, s.num_frames);
        display_list.clear();
        let mut frame_i = 0;
        for t in s.tags.iter() {
            match t {
                swf::Tag::ShowFrame => {
                    if 0 == display_list
                        .iter()
                        .filter(|&n| {
                            if let Some((id, _, _)) = n {
                                allowed.binary_search(id).is_ok()
                            } else {
                                false
                            }
                        })
                        .count()
                    {
                        fuckprint!("  NOSFRAME({}) \\\n", frame_i);
                        frame_i += 1;
                    } else {
                        fuckprint!("  SFRAME({}, \\\n", frame_i);
                        frame_i += 1;
                        for (id, t, m) in display_list.iter().flatten() {
                            if allowed.binary_search(id).is_ok() {
                                let m = m.unwrap_or(swf::Matrix::IDENTITY);
                                fuckprint!(
                                    "   SP{}({}, {}, {}, {}, {}, {}, {}) \\\n",
                                    match t {
                                        Object::Shape => "SH",
                                        Object::Sprite => "SP",
                                    },
                                    id,
                                    m.a.to_f64(),
                                    m.b.to_f64(),
                                    m.c.to_f64(),
                                    m.d.to_f64(),
                                    m.tx.to_pixels(),
                                    m.ty.to_pixels(),
                                );
                            } else {
                                fuckprint!("   /* would be id {}*/ \\\n", id);
                            }
                        }
                        fuckprint!("  ) \\\n");
                    }
                }
                swf::Tag::DoAbc(items) => soft_todo!(),
                swf::Tag::DoAbc2(do_abc2) => soft_todo!(),
                swf::Tag::DoAction(items) => soft_todo!(),
                swf::Tag::DoInitAction { id, action_data } => soft_todo!(),
                swf::Tag::End => soft_todo!(),
                swf::Tag::SoundStreamBlock(items) => soft_todo!(),
                swf::Tag::SoundStreamHead(sound_stream_head) => soft_todo!(),
                swf::Tag::SoundStreamHead2(sound_stream_head) => soft_todo!(),
                swf::Tag::StartSound(start_sound) => soft_todo!(),
                swf::Tag::StartSound2 {
                    class_name,
                    sound_info,
                } => soft_todo!(),
                swf::Tag::PlaceObject(place_object) => {
                    // soft_todo This code has been copy&pasted!! Refactor!
                    match place_object.action {
                        swf::PlaceObjectAction::Place(id) => {
                            /*trace!("placeobj {}\n", id);*/
                            if display_list.len() <= place_object.depth as usize {
                                display_list.resize(place_object.depth as usize + 1, None);
                            }
                            let orig_matrix = if display_list[place_object.depth as usize].is_some()
                            {
                                display_list[place_object.depth as usize].clone().unwrap().2
                            } else {
                                None
                            };
                            display_list[place_object.depth as usize] = Some((
                                id,
                                if sprite_ids.binary_search(&id).is_ok() {
                                    Object::Sprite
                                } else {
                                    Object::Shape
                                },
                                if place_object.matrix.is_some() {
                                    place_object.matrix
                                } else {
                                    orig_matrix
                                },
                            ));
                        }
                        swf::PlaceObjectAction::Modify => {
                            /*trace!("modifyobj at depth {}\n", place_object.depth);*/
                            let orig_matrix =
                                display_list[place_object.depth as usize].clone().unwrap().2;
                            display_list[place_object.depth as usize] = Some((
                                display_list[place_object.depth as usize].clone().unwrap().0,
                                display_list[place_object.depth as usize].clone().unwrap().1,
                                if place_object.matrix.is_some() {
                                    place_object.matrix
                                } else {
                                    orig_matrix
                                },
                            ))
                        }
                        swf::PlaceObjectAction::Replace(id) => {
                            /*trace!("replace id {}\n", id);*/
                            let orig_matrix =
                                display_list[place_object.depth as usize].clone().unwrap().2;
                            display_list[place_object.depth as usize] = Some((
                                id,
                                if sprite_ids.binary_search(&id).is_ok() {
                                    Object::Sprite
                                } else {
                                    Object::Shape
                                },
                                if place_object.matrix.is_some() {
                                    place_object.matrix
                                } else {
                                    orig_matrix
                                },
                            ));
                        }
                    };
                }
                swf::Tag::RemoveObject(remove_object) => {
                    /*trace!("remove obj in sprite\n");*/
                    display_list[remove_object.depth as usize] = None;
                }
                _ => soft_todo!("Unexpected tag {:?}", t),
            }
        }
        here_we_go_couting += 1;
        fuckprint!(
            " ){}\n",
            if here_we_go_couting == sprites.len() {
                ""
            } else {
                " \\"
            }
        );
    }

    fuckprint!("\n#define FRAME_XS");
    allowed.sort();
    let mut fuck = 1;
    for bruh in display_lists.iter() {
        match bruh {
            Frame::DisplayList(dl) => {
                fuckprint!(" \\\nF({},", fuck);
                fuck += 1;
                for (id, t, m) in dl.iter().flatten() {
                    if allowed.binary_search(id).is_ok() {
                        // TODO Optimize out: if previous matrix identical to
                        // current, don't change matrix. but needs to check
                        // recursively for every sprite
                        // TODO Deduplicate code from sprite and main loop
                        let m = m.unwrap_or(swf::Matrix::IDENTITY);
                        fuckprint!(
                            " \\\n P{}({}, {}, {}, {}, {}, {}, {})",
                            match t {
                                Object::Shape => "SH",
                                Object::Sprite => "SP",
                            },
                            id,
                            m.a.to_f64(),
                            m.b.to_f64(),
                            m.c.to_f64(),
                            m.d.to_f64(),
                            m.tx.to_pixels(),
                            m.ty.to_pixels(),
                        );
                    }
                }
                fuckprint!(")");
            }
        }
    }

    //assert!(swf.header.num_frames() as usize == display_lists.len());
    /*
        fuckprint!("\n");
        fuckprint!("n\n");
    */

    fuckprint!(
        r#"
#ifndef FEAT_JAVASCRIPT

#define SDEF(...) +1
#if 0 SPRITE_XS > 0
# undef SDEF
# define SDEF(ID, COUNT, CTOR) s##ID##d = COUNT,
  enum {{ SPRITE_XS }};
# undef SDEF
#endif
#undef SDEF

#ifndef FEAT_NO_DATA
void {0}_data(float *framerate, int *n_frame, int *width, int *height)
{{
    if (framerate) *framerate = {3};
    if (n_frame)   *n_frame   = {4};
    if (width)     *width     = {1};
    if (height)    *height    = {2};
}}

bool {0}_background_color(unsigned char *r, unsigned char *g, unsigned char *b)
{{
    {5}
}}

#define MATRIX_XS(S) X(a,1)S X(b,0)S X(c,0)S X(d,1)S X(tx,0)S X(ty,0)
#define COMMA ,

#define X(I,V) static double t_##I = V
MATRIX_XS(;);
#undef X
#define X(I,V) double I
void {0}_transform(MATRIX_XS(COMMA))
#undef X
{{
#define X(I,V) t_##I = I
MATRIX_XS(;);
#undef X
}}

#endif
#endif

#ifdef FEAT_PLUTOVG
#include <assert.h>
#include <string.h>
#include <plutovg.h>

static void segment(plutovg_path_t *p, float x[static 1], float y[static 1], float dx, float dy) {{
 plutovg_path_line_to(p, *x += dx, *y += dy);
}}

static void curve(plutovg_path_t *p, float x[static 1], float y[static 1], float adx, float ady, float cdx, float cdy) {{
 float new_x = *x+adx+cdx;
 float new_y = *y+ady+cdy;
 plutovg_path_quad_to(p, *x+cdx, *y+cdy, new_x, new_y);
 *x = new_x;
 *y = new_y;
}}

#define M(X, Y) plutovg_path_move_to(p, x=(X), y=(Y));
#define L(DX, DY) segment(p, &x, &y, (DX), (DY));
#define B(X0, Y0, ...) curve(p, &x, &y, __VA_ARGS__);
#define X(ID, COUNT, CTOR) \
 static plutovg_path_t *o##ID; \
 static void o##ID##i(void) {{ plutovg_path_t *p = o##ID; float x, y; CTOR }}
SHAPE_XS
#undef X
#undef B
#undef L
#undef M

static plutovg_surface_t *s;
static plutovg_canvas_t *c;

void {0}_init_plutovg(void) {{
 s = plutovg_surface_create({1}, {2});
 c = plutovg_canvas_create(s);
 plutovg_canvas_set_line_width(c, 1);
 plutovg_canvas_set_rgb(c, 0,0,0);

 #define X(ID, COUNT, CTOR) \
  o##ID = plutovg_path_create(); \
  plutovg_path_reserve(o##ID, (COUNT));
 SHAPE_XS
 #undef X

 #define X(ID, COUNT, CTOR) o##ID##i,
 static void(*inits[])(void) = {{
  SHAPE_XS
 }};
 #undef X

 #pragma omp parallel for schedule(dynamic)
 for (int i = 0; i < (int)(sizeof inits / sizeof *inits); i++)
  inits[i]();
}}

void {0}_free_plutovg(void) {{
 #define X(ID, COUNT, CTOR) plutovg_path_destroy(o##ID);
 SHAPE_XS
 #undef X
 plutovg_canvas_destroy(c);
 plutovg_surface_destroy(s);
}}

#define SDEF(ID, COUNT, CTOR) \
 static void s##ID##r(int frame_i, plutovg_matrix_t const base[static 1]) {{ \
  plutovg_matrix_t matrix; \
  (void)matrix; (void)base; \
  switch (frame_i) {{ \
   CTOR \
   default: assert(!"Invalid frame!"); break; \
  }} \
 }}
#define SFRAME(FRAME, CTOR) case FRAME: CTOR break;
#define NOSFRAME(FRAME) case FRAME: break;
#define SPSH(ID, ...) \
 assert(o##ID != NULL); \
 plutovg_matrix_multiply(&matrix, &PLUTOVG_MAKE_MATRIX(__VA_ARGS__), base); \
 plutovg_canvas_set_matrix(c, &matrix); \
 plutovg_canvas_stroke_path(c, o##ID);
#define SPSP(ID, ...) \
 plutovg_matrix_multiply(&matrix, &PLUTOVG_MAKE_MATRIX(__VA_ARGS__), base); \
 s##ID##r(ID % s##ID##d, &matrix);
SPRITE_XS
#undef SPSP
#undef SPSH
#undef SFRAME
#undef SDEF

void {0}_render_sdl_plutovg(void *pixels, int pitch, int frame) {{
 assert(c != NULL);
 switch (frame) {{
  #define F(ID, CTOR) case ID: CTOR break;
  #define PSH(ID, A, B, C, D, TX, TY) \
   assert(o##ID != NULL); \
   plutovg_canvas_set_matrix(c, &PLUTOVG_MAKE_MATRIX(A, B, C, D, TX, TY)); \
   plutovg_canvas_stroke_path(c, o##ID);
  #define PSP(ID, ...) s##ID##r(ID % s##ID##d, &PLUTOVG_MAKE_MATRIX(__VA_ARGS__));
  FRAME_XS
  #undef PSP
  #undef PSH
  #undef F
  default: assert(!"No such frame"); break;
 }}

 int stride = plutovg_surface_get_stride(s);
 unsigned char *data = plutovg_surface_get_data(s);
 if (stride == pitch)
  memcpy(pixels, data, 4 * {1} * {2});
 else for (int i = 0; i < {2}; i++)
  memcpy(((unsigned char *)pixels) + pitch * i, data + stride * i, 4 * {1});
 plutovg_surface_clear(s, &PLUTOVG_WHITE_COLOR);
}}
#endif

#if defined(FEAT_HTML5) || defined(FEAT_JAVASCRIPT)
#ifdef FEAT_HTML5
#include <emscripten.h>
#endif

// TODO I got bored, implement sprites here.

#define X(ID, COUNT, CTOR) \
 function o##ID##r(ctx) {{ let x, y; ctx.beginPath(); CTOR ctx.stroke(); }}
#define M(X, Y) ctx.moveTo(x=(X), y=(Y));
#define L(DX, DY) ctx.lineTo(x += (DX), y += (DY));
#define B(X0, Y0, ADX, ADY, CDX, CDY) \
 ctx.quadraticCurveTo(x+(CDX), y+(CDY), x+(CDX)+(ADX), y+(CDY)+(ADY)); x += (ADX)+(CDX); y += (ADY)+(CDY);
#define F(ID, CTOR) case ID: CTOR break;
#define PSH(ID, A, B, C, D, TX, TY) \
 ctx.setTransform((A), (B), (C), (D), (TX), (TY)); \
 o##ID##r(ctx);
#define PSP(...)
#ifdef FEAT_HTML5
#define EM_JS2(...) EM_JS(__VA_ARGS__);
#elif defined(FEAT_JAVASCRIPT)
#define EM_JS2(T, N, P, B) function {0}_render(ctx, frame) B
#endif
EM_JS2(void, {0}_render_html5_2, (__externref_t ctx, int frame), {{
 SHAPE_XS
 ctx.resetTransform();
 ctx.fillStyle = 'white';
 ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
 ctx.strokeStyle = 'black';
 switch (frame) {{
  FRAME_XS
  default: alert('No such frame'); break;
 }}
}})
#undef EM_JS2
#undef P
#undef F
#undef B
#undef L
#undef M
#undef X

#ifdef FEAT_HTML5
void {0}_render_html5(__externref_t CanvasRenderingContext2D, int frame) {{
    {0}_render_html5_2(CanvasRenderingContext2D, frame);
}}
#endif

#ifdef FEAT_JAVASCRIPT
function {0}_info() {{
    return {{
        framerate: {3},
        n_frame: {4},
        width: {1},
        height: {2}
    }};
}}
#endif

#endif

#ifdef FEAT_CAIRO
#include <cairo.h>
#include <assert.h>
#include <stddef.h>

#define X(ID,F) static cairo_pattern_t *cap##ID;
FILL_XS
#undef X

static cairo_matrix_t ca_scale;

#define X(ID,F) static void cap_i##ID(void) {{ cairo_pattern_t *p; F cap##ID = p; }}
#define S(R,G,B,A) p = cairo_pattern_create_rgba(R/255.0, G/255.0, B/255.0, A/255.0);
#define M(...) \
cairo_matrix_t m = {{__VA_ARGS__}}; \
/*cairo_matrix_multiply(&m, &m, &ca_scale);*/ \
cairo_pattern_set_matrix(p, &m);
#define B(ID,...) p = cairo_pattern_create_rgba(1,1,1,1);
#define SMOOTH_YES
#define SMOOTH_NO
#define REPEAT_YES
#define REPEAT_NO
#define G(T,SPR,INTR,RS,M) p = cairo_pattern_create_rgba(1,1,1,0);
//#define G(T,SPR,INTR,RS,M) p = T; cairo_pattern_set_extend(p,SPR); INTR RS M
#define L cairo_pattern_create_linear(-16384,0,16384,0)
#define R cairo_pattern_create_radial(0,0,0,0,0,16384)
#define F(R) cairo_pattern_create_radial(R*16384,0,0,0,0,16384)
#define PAD CAIRO_EXTEND_PAD
#define REFLECT CAIRO_EXTEND_REFLECT
#define REPEAT CAIRO_EXTEND_REPEAT
#define NRGB
#define LRGB
#define GR(RA,R,G,B,A) cairo_pattern_add_color_stop_rgba(p,RA/255.0,R/255.0,G/255.0,B/255.0,A/255.0);
FILL_XS
#undef G
#undef L
#undef R
#undef F
#undef PAD
#undef REFLECT
#undef REPEAT
#undef NRGB
#undef LRGB
#undef GR
#undef B
#undef SMOOTH_YES
#undef SMOOTH_NO
#undef REPEAT_YES
#undef REPEAT_NO
#undef S
#undef M
#undef X

#define X(ID,F) cap_i##ID();
void {0}_init_cairo(void) {{ cairo_matrix_init_scale(&ca_scale, 1/20.0, 1/20.0); FILL_XS }}
#undef X

#define X(ID,F) cairo_pattern_destroy(cap##ID);
void {0}_free_cairo(void) {{ FILL_XS }}
#undef X

#define MKH(L,T) {{.header = {{.length=(L),.type=CAIRO_PATH_##T}}}}
#define MKP(X,Y) {{.point={{(X), (Y)}}}}
#define M(X, Y) MKH(2,MOVE_TO),MKP((X),(Y)),
#define L(X, Y) MKH(2,LINE_TO),MKP((X),(Y)),
#define B(X0, Y0, CX, CY, X, Y) MKH(4,CURVE_TO),\
MKP(2.0 / 3.0 * (CX) + 1.0 / 3.0 * (X0), 2.0 / 3.0 * (CY) + 1.0 / 3.0 * (Y0)),\
MKP(2.0 / 3.0 * (CX) + 1.0 / 3.0 * (X) , 2.0 / 3.0 * (CY) + 1.0 / 3.0 * (Y)),\
MKP((X),(Y)),
#define C MKH(1,CLOSE_PATH),
#define A(ID,FID,CTOR) static cairo_path_data_t ca_d##ID[] = {{ CTOR }};
#define S(ID,FID,WID,CTOR) static cairo_path_data_t ca_d##ID[] = {{ CTOR }};
#define X(ID,CTOR) CTOR
SHAPE_XS
#undef X
#undef S
#undef A
#undef C
#undef B
#undef L
#undef M
#undef MKP
#undef MKH

#define MKD(ID) static cairo_path_t ca_p##ID = \
{{ CAIRO_STATUS_SUCCESS, ca_d##ID, sizeof (ca_d##ID) / sizeof (cairo_path_data_t) }};
#define A(ID,FID,CTOR) MKD(ID)
#define S(ID,FID,WID,CTOR) MKD(ID)
#define X(ID,CTOR) CTOR
SHAPE_XS
#undef X
#undef S
#undef A
#undef MKD

#define A(ID,FID,CTOR) cairo_set_source(cr,cap##FID); cairo_append_path(cr, &ca_p##ID); cairo_fill(cr);
#define S(ID,FID,WID,CTOR) cairo_set_line_width(cr,WID); cairo_append_path(cr, &ca_p##ID); cairo_set_source(cr,cap##FID); cairo_stroke(cr);
#define X(ID, CTOR) static void car##ID(cairo_t *cr) {{ (void)cr; CTOR }}
SHAPE_XS
#undef X
#undef S
#undef A

static void cairo_trans2(cairo_t *cr, void (*r)(cairo_t *cr), cairo_matrix_t const matrix[static 1])
{{
 cairo_set_matrix(cr, matrix);
 r(cr);
}}

static void cairo_trans(cairo_t *cr, void (*r)(cairo_t *cr), double a, double b, double c, double d, double tx, double ty)
{{
 cairo_matrix_t matrix = {{a, b, c, d, tx, ty}};
 cairo_trans2(cr, r, &matrix);
}}

#define SDEF(ID, COUNT, CTOR) \
 static void car_s##ID##r(cairo_t *c, int frame_i, cairo_matrix_t const base[static 1]) {{ \
  cairo_matrix_t matrix; \
  (void)c; (void)matrix; (void)base; \
  switch (frame_i) {{ \
   CTOR \
   default: assert(!"Invalid frame!"); break; \
  }} \
 }}
#define SFRAME(FRAME, CTOR) case FRAME: CTOR break;
#define NOSFRAME(FRAME) case FRAME: break;
#define SPSH(ID, ...) \
 cairo_matrix_multiply(&matrix, &(cairo_matrix_t){{__VA_ARGS__}}, base); \
 cairo_trans2(c, car##ID, &matrix);
#define SPSP(ID, ...) \
 cairo_matrix_multiply(&matrix, &(cairo_matrix_t){{__VA_ARGS__}}, base); \
 car_s##ID##r(c, ID % s##ID##d, &matrix);
SPRITE_XS
#undef SPSP
#undef SPSH
#undef SFRAME
#undef SDEF

void {0}_render_cairo(cairo_t *cr, int frame)
{{
 cairo_set_line_width(cr, 1);
 cairo_set_fill_rule(cr, CAIRO_FILL_RULE_EVEN_ODD);
 #define X(I,V) t_##I
 cairo_matrix_t matrix, base = {{MATRIX_XS(COMMA)}};
 #undef X
 switch (frame)
 {{
  #define F(ID, CTOR) case ID: CTOR break;
  #define PSH(ID, ...) \
  cairo_matrix_multiply(&matrix, &(cairo_matrix_t){{__VA_ARGS__}}, &base); \
  cairo_trans2(cr, car##ID, &matrix);
  #define PSP(ID, ...) \
  cairo_matrix_multiply(&matrix, &(cairo_matrix_t){{__VA_ARGS__}}, &base); \
  car_s##ID##r(cr, ID % s##ID##d, &matrix);
  FRAME_XS
  #undef PSP
  #undef PSH
  #undef F
  default: assert(!"No such frame"); break;
 }}
}}

#endif
"#,
        lowername,
        pixel_width,
        pixel_height,
        le_framerate,
        le_n_frame,
        match background {
            Some(bg) => format!(
                "if (r) *r = {0};
    if (g) *g = {1};
    if (b) *b = {2};
    return true;",
                bg.r, bg.g, bg.b
            ),
            None => "(void)r; (void)g; (void)b; return false;".to_owned(),
        }
    );

    /*trace!("max x = {}\n", swf.header.stage_size().x_max.to_pixels());*/
    /*trace!("max y = {}\n", swf.header.stage_size().y_max.to_pixels());*/
    /*trace!("min x = {}\n", swf.header.stage_size().x_min.to_pixels());*/
    /*trace!("min y = {}\n", swf.header.stage_size().y_min.to_pixels());*/
    /*trace!("decompressed tags: {} bytes\n", swf_buf.data.len());*/
    efuckprint!("n_clipping = {} n_blending = {}\n", n_clipping, n_blending);
}
