use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

#[derive(Clone)]
enum Object { Shape, Sprite }

fn main()
{
    let mut out = std::io::stdout().lock();

    macro_rules! fuckprint {
        ($($x:tt)*) => {
            write!(out, $($x)*).unwrap_or_else(|_| { std::process::exit(0)})
        }
    }
    macro_rules! efuckprint {
        ($($x:tt)*) => {
            write!(std::io::stderr().lock(), $($x)*).unwrap_or_else(|_| { std::process::exit(0)})
        }
    }

    let die = || -> ! {efuckprint!("Usage: swf2c -c|-h|-s<n> <file>\n"); std::process::exit(1);};

    let Ok(g) = getopt3::new(env::args().skip(1), "chs:") else { die(); };
    let Ok(g) = getopt3::validate(g) else { die(); };

    if g.arguments.len() != 1 { die(); }

    //let is_trace = g.options.contains_key(&'t');
    let is_header = g.options.contains_key(&'h');
    let is_source = g.options.contains_key(&'c');
    let wants_stats = g.options.get(&'s');

    macro_rules! trace {
        ($($x:tt)*) => { if is_trace { efuckprint!($($x)*) }}
    }

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
    let filename = if let Some(f) = g.arguments.first() {f} else { die(); };

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

    for (i, c) in filename.as_bytes().iter().enumerate().skip(if let Some(start) = filename.rfind('/') { start + 1} else { 0 }) {
        match c {
            b'0'..=b'9' if i != 0 => {lowername.push( *c                as char); uppername.push( *c                as char); },
            b'a'..=b'z'           => {lowername.push( *c                as char); uppername.push((*c - b'a' + b'A') as char); },
            b'A'..=b'Z'           => {lowername.push((*c - b'A' + b'a') as char); uppername.push( *c                as char); },
            b'_' | b'.'           => {lowername.push('_'); uppername.push('_'); },
            _ => {
                efuckprint!("filename wieldn't a C identifier\n");
                std::process::exit(1);
            },
        }
    }

    if filename.ends_with(".swf") {
        lowername.truncate(lowername.len() - 4);
        uppername.truncate(uppername.len() - 4);
    }

    let file = if let Ok(f) = File::open(filename) { f } else {
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
                                        .trim_end_matches('.').replacen(".", "\\&,", 1);
                }
                fuckprint!("{}\t{}x{}\t{}", correct_framerate, pixel_width, pixel_height, swf.header.num_frames());
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
            _ => die()
        }
    }

    if is_header {
        fuckprint!(r"#ifndef {}_H
#define {}_H

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

#ifndef FEAT_NO_DATA
void {}_data(float *framerate, int *n_frame, int *width, int *height);
#endif

// TODO Discuss rendering cutscenes, pre-rendering sprites, ImageBitmap etc.
// TODO See Web Worker + OffscreenCanvasRenderingContext2D.
// TODO Document pixel format expected.
// TODO Explain how to copy HTML canvas data to e.g. texture.
// TODO Figure out parameter order
// TODO Figure out some naming convention <swf>_{{init,free,render}}[_<framework>]_<engine>[_<variant>]
// TODO Discuss thread safety and reentrancy
// TODO Add suggested background color for user clearing

#endif
", uppername, uppername, lowername, le_framerate, lowername, le_n_frame, lowername, pixel_width, lowername, pixel_height, lowername, lowername, lowername, lowername, lowername, lowername);

        std::process::exit(0);
    }

    let mut shapes: Vec<swf::Shape> = Vec::new();
    let mut sprite_ids: Vec<u16> = Vec::new();
    let mut sprites: Vec<swf::Sprite> = Vec::new();
    let mut ignored: Vec<u16> = Vec::new();
    let encoding = swf::SwfStr::encoding_for_version(swf.header.version());
    let mut display_list: Vec<Option<(u16, Object, Option<swf::Matrix>)>> = Vec::new();
    let mut display_lists: Vec<Vec<Option<(u16, Object, Option<swf::Matrix>)>>> = Vec::new();
    let mut frame_i = 0;

    let mut n_solid = 0;
    let mut n_linear = 0;
    let mut n_radial = 0;
    let mut n_focal = 0;
    let mut n_bitmap = 0;
    let mut n_clipping = 0;
    let mut n_blending = 0;
    let mut n_fill0 = 0;
    let mut n_fill1 = 0;

    let limit = 9999;
    for tag in swf.tags {
        match tag {
            swf::Tag::ExportAssets(exported_assets) => soft_todo!(),
            swf::Tag::ScriptLimits { max_recursion_depth, timeout_in_seconds } => soft_todo!(),
            swf::Tag::ShowFrame => { /*trace!("frame {}\n", frame_i);*/ display_lists.push(display_list.clone()); frame_i += 1; if frame_i >= limit { break;} },
            swf::Tag::Protect(None) => /*trace!("no protect\n")*/(),
            swf::Tag::Protect(Some(swf_str)) => /*trace!("protect is {}\n", swf_str.to_string_lossy(encoding))*/(),
            swf::Tag::CsmTextSettings(csm_text_settings) => soft_todo!(),
            swf::Tag::DebugId(_) => soft_todo!(),
            swf::Tag::DefineBinaryData(define_binary_data) => soft_todo!(),
            swf::Tag::DefineBits { id, jpeg_data } => soft_todo!(),
            swf::Tag::DefineBitsJpeg2 { id, jpeg_data } => soft_todo!(),
            swf::Tag::DefineBitsJpeg3(define_bits_jpeg3) => soft_todo!(),
            swf::Tag::DefineBitsLossless(define_bits_lossless) => soft_todo!(),
            swf::Tag::DefineButton(button) => soft_todo!(),
            swf::Tag::DefineButton2(button) => ignored.push(button.id),
            swf::Tag::DefineButtonColorTransform(button_color_transform) => soft_todo!(),
            swf::Tag::DefineButtonSound(button_sounds) => soft_todo!(),
            swf::Tag::DefineEditText(edit_text) => soft_todo!(),
            swf::Tag::DefineFont(font_v1) => soft_todo!(),
            swf::Tag::DefineFont2(font) => soft_todo!(),
            swf::Tag::DefineFont4(font4) => soft_todo!(),
            swf::Tag::DefineFontAlignZones { id, thickness, zones } => soft_todo!(),
            swf::Tag::DefineFontInfo(font_info) => {
                /*trace!("define font info. no {}\n", font_info.code_table.len());*/()
            },
            swf::Tag::DefineFontName { id, name, copyright_info } => soft_todo!(),
            swf::Tag::DefineMorphShape(define_morph_shape) => soft_todo!(),
            swf::Tag::DefineScalingGrid { id, splitter_rect } => soft_todo!(),
            swf::Tag::DefineShape(shape) => { /*trace!("defshape {}\n", shape.id);*/ shapes.push(shape)},
            swf::Tag::DefineSound(sound) => soft_todo!(),
            swf::Tag::DefineSprite(sprite) => {
                /*ignored.push(sprite.id);*/
                /*trace!("sprite {} with {} tags and {} frames\n", sprite.id, sprite.tags.len(), sprite.num_frames);*/
                sprite_ids.push(sprite.id);
                sprites.push(sprite);
            },
            swf::Tag::DefineText(text) => ignored.push(text.id),
            swf::Tag::DefineText2(text) => soft_todo!(),
            swf::Tag::DefineVideoStream(define_video_stream) => soft_todo!(),
            swf::Tag::DoAbc(items) => soft_todo!(),
            swf::Tag::DoAbc2(do_abc2) => soft_todo!(),
            swf::Tag::DoAction(items) => {
                /*trace!("some {} actions\n", items.len());*/
            },
            swf::Tag::DoInitAction { id, action_data } => soft_todo!(),
            swf::Tag::EnableDebugger(swf_str) => soft_todo!(),
            swf::Tag::EnableTelemetry { password_hash } => soft_todo!(),
            swf::Tag::End => soft_todo!(),
            swf::Tag::Metadata(swf_str) => soft_todo!(),
            swf::Tag::ImportAssets { url, imports } => soft_todo!(),
            swf::Tag::JpegTables(items) => soft_todo!(),
            swf::Tag::NameCharacter(name_character) => soft_todo!(),
            swf::Tag::SetBackgroundColor(color) => { soft_todo!(); fuckprint!("//bg is ({} {} {} {})\n", color.r, color.g, color.b, color.a) },
            swf::Tag::SetTabIndex { depth, tab_index } => soft_todo!(),
            swf::Tag::SoundStreamBlock(items) => soft_todo!(),
            swf::Tag::SoundStreamHead(sound_stream_head) => soft_todo!(),
            swf::Tag::SoundStreamHead2(sound_stream_head) => soft_todo!(),
            swf::Tag::StartSound(start_sound) => soft_todo!(),
            swf::Tag::StartSound2 { class_name, sound_info } => soft_todo!(),
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
                        display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape},  place_object.matrix));
                    },
                    swf::PlaceObjectAction::Modify => {
                        /*trace!("modifyobj at depth {}\n", place_object.depth);*/
                        display_list[place_object.depth as usize]
                        = Some((display_list[place_object.depth as usize].clone().unwrap().0, display_list[place_object.depth as usize].clone().unwrap().1, place_object.matrix))
                    },
                    swf::PlaceObjectAction::Replace(id) => {
                        /*trace!("replace id {}\n", id);*/
                        display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape}, place_object.matrix));
                    },
                };
                if let Some(name) = place_object.name { /*trace!("Got name {} when placing\n", name.to_string_lossy(encoding))*/() };
                if let Some(ratio) = place_object.ratio { /*trace!("Got ratio {}\n", ratio)*/() }
                if let Some(actions) = place_object.clip_actions { for ca in actions.iter() {
                    // soft_todo The action data is in swf::OpCode
                    /*trace!("clip action {} with {} data\n", ca.events.bits(), ca.action_data.len())*/
                } }
            },
            swf::Tag::RemoveObject(remove_object) => {
                /*trace!("remove\n");*/
                display_list[remove_object.depth as usize] = None;
            },
            swf::Tag::VideoFrame(video_frame) => soft_todo!(),
            swf::Tag::FileAttributes(file_attributes) => soft_todo!(),
            swf::Tag::FrameLabel(frame_label) => {
                /*trace!("frame label: {} (is anchor: {})\n", frame_label.label.to_str_lossy(encoding), frame_label.is_anchor);*/
                // TODO What is the current frame? the previous rendered or the to-be-rendered?
                ()
            },
            swf::Tag::DefineSceneAndFrameLabelData(define_scene_and_frame_label_data) => soft_todo!(),
            swf::Tag::ProductInfo(product_info) => soft_todo!(),
            swf::Tag::Unknown { tag_code, data } => soft_todo!(),
        }
    }

    let mut i_want_to_kill_myself = 0;
    fuckprint!("#define SHAPE_XS \\\n"); // should be SHAPE_XS (or should it?)
    for s in shapes.iter() {
        fuckprint!(" X({},{}, \\\n", s.id, s.shape.len());
        for sr in s.shape.iter() {
            match sr {
                swf::ShapeRecord::StyleChange(style_change_data) => {
                    n_fill0 += style_change_data.fill_style_0.is_some() as i32;
                    n_fill1 += style_change_data.fill_style_1.is_some() as i32;
                    match style_change_data.move_to {
                        Some(to) => fuckprint!("  M({},{}) \\\n", to.x.to_pixels(), to.y.to_pixels()),
                        None => (),
                    }
                    if let Some(ns) = style_change_data.new_styles.clone() {
                        for fs in ns.fill_styles.iter() {
                            match fs {
                                swf::FillStyle::Color(color) => n_solid += 1,
                                swf::FillStyle::LinearGradient(gradient) => n_linear += 1,
                                swf::FillStyle::RadialGradient(gradient) => n_radial += 1,
                                swf::FillStyle::FocalGradient { gradient, focal_point } => n_focal += 1,
                                swf::FillStyle::Bitmap { id, matrix, is_smoothed, is_repeating } => n_bitmap += 1,
                            }
                        }
                    }
                },
                swf::ShapeRecord::StraightEdge { delta } => {
                    fuckprint!("  L({},{}) \\\n", delta.dx.to_pixels(), delta.dy.to_pixels());
                },
                swf::ShapeRecord::CurvedEdge { control_delta, anchor_delta } => {
                    fuckprint!(
                        "  B({},{},{},{}) \\\n",
                        anchor_delta.dx.to_pixels(),
                        anchor_delta.dy.to_pixels(),
                        control_delta.dx.to_pixels(),
                        control_delta.dy.to_pixels(),
                    );
                },
            }
        }
        fuckprint!(" ){}", if i_want_to_kill_myself == shapes.len() - 1 {"\n"} else {" \\\n"});
        i_want_to_kill_myself += 1;
    }
    fuckprint!("\n");

    // TODO Review those cursed macro names

    sprite_ids.sort();

    fuckprint!("#define SPRITE_XS \\\n");
    let mut here_we_go_couting = 0;
    for s in sprites.iter() {
        fuckprint!(" SDEF({}, {}, \\\n", s.id, s.num_frames);
        display_list.clear();
        let mut frame_i = 0;
        for t in s.tags.iter() {
            match t {
                swf::Tag::ShowFrame => {
                    if display_list.iter().filter(|&n| if let Some((id,_,_)) = n {ignored.binary_search(id).is_ok()} else {false}).count() > 0 {
                        fuckprint!("  NOSFRAME({}) \\\n", frame_i);
                        frame_i += 1;
                    } else {
                        fuckprint!("  SFRAME({}, \\\n", frame_i);
                        frame_i += 1;
                        for i in display_list.iter() {
                            if let Some((id, t, m)) = i {
                                if !ignored.binary_search(id).is_ok() {
                                    let m = m.unwrap_or(swf::Matrix::IDENTITY);
                                    fuckprint!(
                                        "   SP{}({}, {}, {}, {}, {}, {}, {}) \\\n",
                                        match t { Object::Shape => "SH", Object::Sprite => "SP" },
                                        id,
                                        m.a.to_f64(),
                                        m.b.to_f64(),
                                        m.c.to_f64(),
                                        m.d.to_f64(),
                                        m.tx.to_pixels(),
                                        m.ty.to_pixels(),
                                    );
                                }
                                else {
                                    fuckprint!("   /* would be id {}*/\n", id);
                                }
                            }
                        }
                        fuckprint!("  ) \\\n");
                    }
                },
                swf::Tag::DoAbc(items) => soft_todo!(),
                swf::Tag::DoAbc2(do_abc2) => soft_todo!(),
                swf::Tag::DoAction(items) => soft_todo!(),
                swf::Tag::DoInitAction { id, action_data } => soft_todo!(),
                swf::Tag::End => soft_todo!(),
                swf::Tag::SoundStreamBlock(items) => soft_todo!(),
                swf::Tag::SoundStreamHead(sound_stream_head) => soft_todo!(),
                swf::Tag::SoundStreamHead2(sound_stream_head) => soft_todo!(),
                swf::Tag::StartSound(start_sound) => soft_todo!(),
                swf::Tag::StartSound2 { class_name, sound_info } => soft_todo!(),
                swf::Tag::PlaceObject(place_object) => {
                    // soft_todo This code has been copy&pasted!! Refactor!
                    match place_object.action {
                        swf::PlaceObjectAction::Place(id) => {
                            /*trace!("placeobj {}\n", id);*/
                            if display_list.len() <= place_object.depth as usize {
                                display_list.resize(place_object.depth as usize + 1, None);
                            }
                            display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape},  place_object.matrix));
                        },
                        swf::PlaceObjectAction::Modify => {
                            /*trace!("modifyobj at depth {}\n", place_object.depth);*/
                            display_list[place_object.depth as usize]
                            = Some((display_list[place_object.depth as usize].clone().unwrap().0, display_list[place_object.depth as usize].clone().unwrap().1, place_object.matrix))
                        },
                        swf::PlaceObjectAction::Replace(id) => {
                            /*trace!("replace id {}\n", id);*/
                            display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape}, place_object.matrix));
                        },
                    };
                },
                swf::Tag::RemoveObject(remove_object) => {
                    /*trace!("remove obj in sprite\n");*/
                    display_list[remove_object.depth as usize] = None;
                },
                _ => soft_todo!("Unexpected tag {:?}", t),
            }
        }
        here_we_go_couting += 1;
        fuckprint!(" ){}\n", if here_we_go_couting == sprites.len() {""} else {" \\"});
    }

    fuckprint!("\n#define FRAME_XS \\\n");
    ignored.sort();
    for (i, dl) in display_lists.iter().enumerate() {
        fuckprint!(" F({}, \\\n", i);
        for d in dl.iter() {
            match d {
                Some((id, t, m)) => {
                    if !ignored.binary_search(id).is_ok() {
                        // TODO Optimize out: if previous matrix identical to
                        // current, don't change matrix. but needs to check
                        // recursively for every sprite
                        // TODO Deduplicate code from sprite and main loop
                        let m = m.unwrap_or(swf::Matrix::IDENTITY);
                        fuckprint!(
                            "  P{}({}, {}, {}, {}, {}, {}, {}) \\\n",
                            match t { Object::Shape => "SH", Object::Sprite => "SP" },
                            id,
                            m.a.to_f64(),
                            m.b.to_f64(),
                            m.c.to_f64(),
                            m.d.to_f64(),
                            m.tx.to_pixels(),
                            m.ty.to_pixels(),
                        );
                    }
                },
                None => (),
            }
        }
        fuckprint!(" ){}\n", if i == display_lists.len() - 1 {""} else {" \\"});
    }

    //assert!(swf.header.num_frames() as usize == display_lists.len());
/*
    fuckprint!("\n");
    fuckprint!("n\n");
*/

    fuckprint!(r#"
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
#define B(ADX, ADY, CDX, CDY) curve(p, &x, &y, (ADX), (ADY), (CDX), (CDY));
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

#define SDEF(ID, COUNT, CTOR) static int s##ID##d = COUNT;
SPRITE_XS
#undef SDEF

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
 plutovg_matrix_multiply(&matrix, base, &PLUTOVG_MAKE_MATRIX(__VA_ARGS__)); \
 plutovg_canvas_set_matrix(c, &matrix); \
 plutovg_canvas_stroke_path(c, o##ID);
#define SPSP(ID, ...) \
 plutovg_matrix_multiply(&matrix, base, &PLUTOVG_MAKE_MATRIX(__VA_ARGS__)); \
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
#define B(ADX, ADY, CDX, CDY) \
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

// https://lists.freedesktop.org/archives/cairo/2010-April/019691.html
static void
helper_quadratic_to (cairo_t *cr,
                     double x1, double y1,
                     double x2, double y2)
{{
  double x0, y0;
  cairo_get_current_point (cr, &x0, &y0);
  x1 += x0;
  y1 += y0;
  x2 += x1;
  y2 += y1;
  cairo_curve_to (cr,
                  2.0 / 3.0 * x1 + 1.0 / 3.0 * x0,
                  2.0 / 3.0 * y1 + 1.0 / 3.0 * y0,
                  2.0 / 3.0 * x1 + 1.0 / 3.0 * x2,
                  2.0 / 3.0 * y1 + 1.0 / 3.0 * y2,
                  x2, y2);
}}

#define M(X, Y) cairo_move_to(cr, (X), (Y));
#define L(DX, DY) cairo_rel_line_to(cr, (DX), (DY));
#define B(ADX, ADY, CDX, CDY) helper_quadratic_to(cr, (CDX), (CDY), (ADX), (ADY));
#define X(ID, COUNT, CTOR) static void car##ID(cairo_t *cr) {{ CTOR }}
SHAPE_XS
#undef X
#undef B
#undef L
#undef M

// TODO Implement sprites for Cairo

static void cairo_trans(cairo_t *cr, void (*r)(cairo_t *cr), double a, double b, double c, double d, double tx, double ty)
{{
 cairo_matrix_t matrix = {{a, b, c, d, tx, ty}};
 cairo_set_matrix(cr, &matrix);
 r(cr);
 cairo_stroke(cr);
}}

void {0}_render_cairo(cairo_t *cr, int frame)
{{
 cairo_set_line_width(cr, 1);
 cairo_set_source_rgb(cr, 0, 0, 0);
 switch (frame)
 {{
  #define F(ID, CTOR) case ID: CTOR break;
  #define PSH(ID, ...) cairo_trans(cr, car##ID, __VA_ARGS__);
  #define PSP(...)
  FRAME_XS
  #undef PSP
  #undef PSH
  #undef F
  default: assert(!"No such frame"); break;
 }}
}}

#endif

#ifndef FEAT_JAVASCRIPT
#ifndef FEAT_NO_DATA
void {0}_data(float *framerate, int *n_frame, int *width, int *height)
{{
    if (framerate) *framerate = {3};
    if (n_frame)   *n_frame   = {4};
    if (width)     *width     = {1};
    if (height)    *height    = {2};
}}
#endif
#endif
"#, lowername, pixel_width, pixel_height, le_framerate, le_n_frame);

    /*trace!("max x = {}\n", swf.header.stage_size().x_max.to_pixels());*/
    /*trace!("max y = {}\n", swf.header.stage_size().y_max.to_pixels());*/
    /*trace!("min x = {}\n", swf.header.stage_size().x_min.to_pixels());*/
    /*trace!("min y = {}\n", swf.header.stage_size().y_min.to_pixels());*/
    /*trace!("decompressed tags: {} bytes\n", swf_buf.data.len());*/
    efuckprint!("n_solid = {} n_linear = {} n_radial = {}  n_focal = {} n_bitmap = {}\n", n_solid, n_linear, n_radial, n_focal, n_bitmap);
    efuckprint!("n_clipping = {} n_blending = {} n_fill0 = {} n_fill1 = {}\n", n_clipping, n_blending, n_fill0, n_fill1);
}
