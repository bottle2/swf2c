use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

#[derive(Clone)]
enum Object { Shape, Sprite }

fn main() {
    let mut is_trace = false;
    let mut is_header = false;
    let mut is_source = false;
    let mut out = std::io::stdout().lock();
    let mut err = std::io::stderr().lock();

    macro_rules! fuckprint {
        ($($x:tt)*) => {
            write!(out, $($x)*).unwrap_or_else(|_| { std::process::exit(0)})
        }
    }
    macro_rules! efuckprint {
        ($($x:tt)*) => {
            write!(err, $($x)*).unwrap_or_else(|_| { std::process::exit(0)})
        }
    }
    macro_rules! trace {
        ($($x:tt)*) => { if is_trace { efuckprint!($($x)*) }}
    }

    let mut die = || efuckprint!("Usage: swf2c (c|h)t? <file>\n");

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
    }

    if is_header == is_source {
        die()
    }

    if is_header {
        fuckprint!(r"#ifndef THERE_SHE_IS_H
#define THERE_SHE_IS_H

extern int const there_she_is_framerate;
extern int const there_she_is_n_frame;
extern int const there_she_is_width;
extern int const there_she_is_height;

#ifdef FEAT_PLUTOVG
void there_she_is_init_plutovg(void);
void there_she_is_free_plutovg(void);
void there_she_is_render_sdl_plutovg(void *pixels, int pitch, int frame);
#endif

#ifdef FEAT_HTML5
void there_she_is_render_html5(__externref_t CanvasRenderingContext2D, int frame);
void there_she_is_render_sdl_html5(__externref_t CanvasRenderingContext2D, int frame, void *pixels, int pitch);
#endif

// TODO Discuss rendering cutscenes, pre-rendering sprites, ImageBitmap etc.
// TODO See Web Worker + OffscreenCanvasRenderingContext2D.
// TODO Document pixel format expected.
// TODO Explain how to copy HTML canvas data to e.g. texture.
// TODO Figure out parameter order
// TODO Figure out some naming convention <swf>_{{init,free,render}}[_<framework>]_<engine>[_<variant>]
// TODO Discuss thread safety and reentrancy

#endif
");
        std::process::exit(0);
    }

    //fuckprint!("hi baby\n");
    let file = File::open(r"C:\Users\Nero\Downloads\flash_There_She_Is.swf").unwrap();
    let reader = BufReader::new(file);
    let swf_buf = swf::decompress_swf(reader).unwrap();
    let swf = swf::parse_swf(&swf_buf).unwrap();
    trace!("The SWF is version {}.\n", swf.header.version());
    trace!("The SWF has {} tags.\n", swf.tags.len());
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

    let limit = 9999;
    for tag in swf.tags {
        match tag {
            swf::Tag::ExportAssets(exported_assets) => todo!(),
            swf::Tag::ScriptLimits { max_recursion_depth, timeout_in_seconds } => todo!(),
            swf::Tag::ShowFrame => { trace!("frame {}\n", frame_i); display_lists.push(display_list.clone()); frame_i += 1; if frame_i >= limit { break;} },
            swf::Tag::Protect(None) => trace!("no protect\n"),
            swf::Tag::Protect(Some(swf_str)) => trace!("protect is {}\n", swf_str.to_string_lossy(encoding)),
            swf::Tag::CsmTextSettings(csm_text_settings) => todo!(),
            swf::Tag::DebugId(_) => todo!(),
            swf::Tag::DefineBinaryData(define_binary_data) => todo!(),
            swf::Tag::DefineBits { id, jpeg_data } => todo!(),
            swf::Tag::DefineBitsJpeg2 { id, jpeg_data } => {
                trace!("bits jpeg 2 is interesting\n");
            },
            swf::Tag::DefineBitsJpeg3(define_bits_jpeg3) => {
                trace!("interesting bits jpeg, but now 3\n");
            },
            swf::Tag::DefineBitsLossless(define_bits_lossless) => todo!(),
            swf::Tag::DefineButton(button) => todo!(),
            swf::Tag::DefineButton2(button) => ignored.push(button.id),
            swf::Tag::DefineButtonColorTransform(button_color_transform) => todo!(),
            swf::Tag::DefineButtonSound(button_sounds) => todo!(),
            swf::Tag::DefineEditText(edit_text) => todo!(),
            swf::Tag::DefineFont(font_v1) => {
                trace!("define font. no\n");
            },
            swf::Tag::DefineFont2(font) => todo!(),
            swf::Tag::DefineFont4(font4) => todo!(),
            swf::Tag::DefineFontAlignZones { id, thickness, zones } => todo!(),
            swf::Tag::DefineFontInfo(font_info) => {
                trace!("define font info. no {}\n", font_info.code_table.len());
            },
            swf::Tag::DefineFontName { id, name, copyright_info } => todo!(),
            swf::Tag::DefineMorphShape(define_morph_shape) => todo!(),
            swf::Tag::DefineScalingGrid { id, splitter_rect } => todo!(),
            swf::Tag::DefineShape(shape) => { trace!("defshape {}\n", shape.id); shapes.push(shape)},
            swf::Tag::DefineSound(sound) => {
                trace!("no sound for now\n");
            },
            swf::Tag::DefineSprite(sprite) => {
                /*ignored.push(sprite.id);*/
                trace!("sprite {} with {} tags and {} frames\n", sprite.id, sprite.tags.len(), sprite.num_frames);
                sprite_ids.push(sprite.id);
                sprites.push(sprite);
            },
            swf::Tag::DefineText(text) => ignored.push(text.id),
            swf::Tag::DefineText2(text) => todo!(),
            swf::Tag::DefineVideoStream(define_video_stream) => todo!(),
            swf::Tag::DoAbc(items) => todo!(),
            swf::Tag::DoAbc2(do_abc2) => todo!(),
            swf::Tag::DoAction(items) => {
                trace!("some {} actions\n", items.len());
            },
            swf::Tag::DoInitAction { id, action_data } => todo!(),
            swf::Tag::EnableDebugger(swf_str) => todo!(),
            swf::Tag::EnableTelemetry { password_hash } => todo!(),
            swf::Tag::End => todo!(),
            swf::Tag::Metadata(swf_str) => todo!(),
            swf::Tag::ImportAssets { url, imports } => todo!(),
            swf::Tag::JpegTables(items) => todo!(),
            swf::Tag::NameCharacter(name_character) => todo!(),
            swf::Tag::SetBackgroundColor(color) => fuckprint!("//bg is ({} {} {} {})\n", color.r, color.g, color.b, color.a),
            swf::Tag::SetTabIndex { depth, tab_index } => todo!(),
            swf::Tag::SoundStreamBlock(items) => {
                trace!("sb");
            },
            swf::Tag::SoundStreamHead(_sound_stream_head) => trace!("ignoring sound at line {}\n", line!()),
            swf::Tag::SoundStreamHead2(sound_stream_head) => todo!(),
            swf::Tag::StartSound(start_sound) => {
                trace!("no sound starting for now\n");
            },
            swf::Tag::StartSound2 { class_name, sound_info } => todo!(),
            swf::Tag::SymbolClass(symbol_class_links) => todo!(),
            swf::Tag::PlaceObject(place_object) => {
                sprite_ids.sort();
                match place_object.action {
                    swf::PlaceObjectAction::Place(id) => {
                        trace!("placeobj {}\n", id);
                        if display_list.len() <= place_object.depth as usize {
                            display_list.resize(place_object.depth as usize + 1, None);
                        }
                        display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape},  place_object.matrix));
                    },
                    swf::PlaceObjectAction::Modify => {
                        trace!("modifyobj at depth {}\n", place_object.depth);
                        display_list[place_object.depth as usize]
                        = Some((display_list[place_object.depth as usize].clone().unwrap().0, display_list[place_object.depth as usize].clone().unwrap().1, place_object.matrix))
                    },
                    swf::PlaceObjectAction::Replace(id) => {
                        trace!("replace id {}\n", id);
                        display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape}, place_object.matrix));
                    },
                };
                match place_object.name {
                    Some(name) => trace!("Got name {} when placing\n", name.to_string_lossy(encoding)),
                    None => (),
                };
                match place_object.ratio {
                    Some(ratio) => trace!("Got ratio {}\n", ratio),
                    None => (),
                }
                match place_object.clip_actions {
                    Some(actions) => for ca in actions.iter() {
                        // TODO The action data is in swf::OpCode
                        trace!("clip action {} with {} data\n", ca.events.bits(), ca.action_data.len())
                    }
                    None => (),
                }
            },
            swf::Tag::RemoveObject(remove_object) => {
                trace!("remove\n");
                display_list[remove_object.depth as usize] = None;
            },
            swf::Tag::VideoFrame(video_frame) => todo!(),
            swf::Tag::FileAttributes(file_attributes) => todo!(),
            swf::Tag::FrameLabel(frame_label) => {
                trace!("frame label: {} (is anchor: {})\n", frame_label.label.to_str_lossy(encoding), frame_label.is_anchor);
                // TODO What is the current frame? the previous rendered or the to-be-rendered?
            },
            swf::Tag::DefineSceneAndFrameLabelData(define_scene_and_frame_label_data) => todo!(),
            swf::Tag::ProductInfo(product_info) => todo!(),
            swf::Tag::Unknown { tag_code, data } => todo!(),
        }
    }

    let mut i_want_to_kill_myself = 0;
    fuckprint!("#define SHAPE_XS \\\n"); // should be SHAPE_XS (or should it?)
    for s in shapes.iter() {
        fuckprint!(" X({},{}, \\\n", s.id, s.shape.len());
        for sr in s.shape.iter() {
            match sr {
                swf::ShapeRecord::StyleChange(style_change_data) => {
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
                swf::Tag::DoAbc(items) => todo!(),
                swf::Tag::DoAbc2(do_abc2) => todo!(),
                swf::Tag::DoAction(items) => trace!("sprite action\n"),
                swf::Tag::DoInitAction { id, action_data } => todo!(),
                swf::Tag::End => todo!(),
                swf::Tag::SoundStreamBlock(items) => todo!(),
                swf::Tag::SoundStreamHead(sound_stream_head) => todo!(),
                swf::Tag::SoundStreamHead2(sound_stream_head) => trace!("sprite sshead\n"),
                swf::Tag::StartSound(start_sound) => todo!(),
                swf::Tag::StartSound2 { class_name, sound_info } => todo!(),
                swf::Tag::PlaceObject(place_object) => {
                    // TODO This code has been copy&pasted!! Refactor!
                    match place_object.action {
                        swf::PlaceObjectAction::Place(id) => {
                            trace!("placeobj {}\n", id);
                            if display_list.len() <= place_object.depth as usize {
                                display_list.resize(place_object.depth as usize + 1, None);
                            }
                            display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape},  place_object.matrix));
                        },
                        swf::PlaceObjectAction::Modify => {
                            trace!("modifyobj at depth {}\n", place_object.depth);
                            display_list[place_object.depth as usize]
                            = Some((display_list[place_object.depth as usize].clone().unwrap().0, display_list[place_object.depth as usize].clone().unwrap().1, place_object.matrix))
                        },
                        swf::PlaceObjectAction::Replace(id) => {
                            trace!("replace id {}\n", id);
                            display_list[place_object.depth as usize] = Some((id, if sprite_ids.binary_search(&id).is_ok() {Object::Sprite} else {Object::Shape}, place_object.matrix));
                        },
                    };
                },
                swf::Tag::RemoveObject(remove_object) => {
                    trace!("remove obj in sprite\n");
                    display_list[remove_object.depth as usize] = None;
                },
                _ => todo!("Unexpected tag {:?}", t),
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
    fuckprint!("int const there_she_is_framerate = {};\n", swf.header.frame_rate());
    fuckprint!("int const there_she_is_width = {};\n", (swf.header.stage_size().x_max - swf.header.stage_size().x_min).to_pixels());
    fuckprint!("int const there_she_is_height = {};\n", (swf.header.stage_size().y_max - swf.header.stage_size().y_min).to_pixels());

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

void there_she_is_init_plutovg(void) {{
 s = plutovg_surface_create(there_she_is_width, there_she_is_height);
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

void there_she_is_free_plutovg(void) {{
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

void there_she_is_render_sdl_plutovg(void *pixels, int pitch, int frame) {{
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
  memcpy(pixels, data, 4 * there_she_is_width * there_she_is_height);
 else for (int i = 0; i < there_she_is_height; i++)
  memcpy(((unsigned char *)pixels) + pitch * i, data + stride * i, 4 * there_she_is_width);
 plutovg_surface_clear(s, &PLUTOVG_WHITE_COLOR);
}}
#endif

#ifdef FEAT_HTML5
#include <emscripten.h>

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
#define EM_JS2(...) EM_JS(__VA_ARGS__)
EM_JS2(void, there_she_is_render_html5, (__externref_t CanvasRenderingContext2D, int frame), {{
 SHAPE_XS
 const ctx = CanvasRenderingContext2D;
 ctx.resetTransform();
 ctx.fillStyle = 'white';
 ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
 ctx.strokeStyle = 'black';
 switch (frame) {{
  FRAME_XS
  default: alert('No such frame'); break;
 }}
}});
#undef EM_JS2
#undef P
#undef F
#undef B
#undef L
#undef M
#undef X
#endif
"#);

    trace!("max x = {}\n", swf.header.stage_size().x_max.to_pixels());
    trace!("max y = {}\n", swf.header.stage_size().y_max.to_pixels());
    trace!("min x = {}\n", swf.header.stage_size().x_min.to_pixels());
    trace!("min y = {}\n", swf.header.stage_size().y_min.to_pixels());
    trace!("decompressed tags: {} bytes\n", swf_buf.data.len());
    fuckprint!("\nint const there_she_is_n_frame = {};\n", display_lists.len());
    efuckprint!("n_solid = {} n_linear = {} n_radial = {}  n_focal = {} n_bitmap = {}\n", n_solid, n_linear, n_radial, n_focal, n_bitmap);
}
