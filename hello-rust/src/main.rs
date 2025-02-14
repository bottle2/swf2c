use std::fs::File;
use std::io::BufReader;
use std::io::Write;

fn main() {
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

    //fuckprint!("hi baby\n");
    let file = File::open(r"C:\Users\Nero\Downloads\flash_There_She_Is.swf").unwrap();
    let reader = BufReader::new(file);
    let swf_buf = swf::decompress_swf(reader).unwrap();
    let swf = swf::parse_swf(&swf_buf).unwrap();
    efuckprint!("Hello, world!\n");
    efuckprint!("The SWF is version {}.\n", swf.header.version());
    efuckprint!("The SWF has {} tags.\n", swf.tags.len());
    fuckprint!("#include <assert.h>\n#include <string.h>\n#include <plutovg.h>\n\n");
    fuckprint!("int const there_she_is_framerate = {};\n", swf.header.frame_rate());
    fuckprint!("int const there_she_is_width = {};\n", (swf.header.stage_size().x_max - swf.header.stage_size().x_min).to_pixels());
    fuckprint!("int const there_she_is_height = {};\n\n", (swf.header.stage_size().y_max - swf.header.stage_size().y_min).to_pixels());
    let mut shapes: Vec<swf::Shape> = Vec::new();
    let mut ignored: Vec<u16> = Vec::new();
    let encoding = swf::SwfStr::encoding_for_version(swf.header.version());
    let mut display_list: Vec<Option<(u16, Option<swf::Matrix>)>> = Vec::new();
    let mut display_lists: Vec<Vec<Option<(u16, Option<swf::Matrix>)>>> = Vec::new();
    let mut frame_i = 0;
    let limit = 99999;
    for tag in swf.tags {
        match tag {
            swf::Tag::ExportAssets(exported_assets) => todo!(),
            swf::Tag::ScriptLimits { max_recursion_depth, timeout_in_seconds } => todo!(),
            swf::Tag::ShowFrame => { display_lists.push(display_list.clone()); frame_i += 1; if frame_i >= limit { break;} },
            swf::Tag::Protect(None) => efuckprint!("no protect\n"),
            swf::Tag::Protect(Some(swf_str)) => efuckprint!("protect is {}\n", swf_str.to_string_lossy(encoding)),
            swf::Tag::CsmTextSettings(csm_text_settings) => todo!(),
            swf::Tag::DebugId(_) => todo!(),
            swf::Tag::DefineBinaryData(define_binary_data) => todo!(),
            swf::Tag::DefineBits { id, jpeg_data } => todo!(),
            swf::Tag::DefineBitsJpeg2 { id, jpeg_data } => {
                efuckprint!("bits jpeg 2 is interesting\n");
            },
            swf::Tag::DefineBitsJpeg3(define_bits_jpeg3) => {
                efuckprint!("interesting bits jpeg, but now 3\n");
            },
            swf::Tag::DefineBitsLossless(define_bits_lossless) => todo!(),
            swf::Tag::DefineButton(button) => todo!(),
            swf::Tag::DefineButton2(button) => ignored.push(button.id),
            swf::Tag::DefineButtonColorTransform(button_color_transform) => todo!(),
            swf::Tag::DefineButtonSound(button_sounds) => todo!(),
            swf::Tag::DefineEditText(edit_text) => todo!(),
            swf::Tag::DefineFont(font_v1) => {
                efuckprint!("define font. no\n");
            },
            swf::Tag::DefineFont2(font) => todo!(),
            swf::Tag::DefineFont4(font4) => todo!(),
            swf::Tag::DefineFontAlignZones { id, thickness, zones } => todo!(),
            swf::Tag::DefineFontInfo(font_info) => {
                efuckprint!("define font info. no {}\n", font_info.code_table.len());
            },
            swf::Tag::DefineFontName { id, name, copyright_info } => todo!(),
            swf::Tag::DefineMorphShape(define_morph_shape) => todo!(),
            swf::Tag::DefineScalingGrid { id, splitter_rect } => todo!(),
            swf::Tag::DefineShape(shape) => shapes.push(shape),
            swf::Tag::DefineSound(sound) => {
                efuckprint!("no sound for now\n");
            },
            swf::Tag::DefineSprite(sprite) => ignored.push(sprite.id),
            swf::Tag::DefineText(text) => ignored.push(text.id),
            swf::Tag::DefineText2(text) => todo!(),
            swf::Tag::DefineVideoStream(define_video_stream) => todo!(),
            swf::Tag::DoAbc(items) => todo!(),
            swf::Tag::DoAbc2(do_abc2) => todo!(),
            swf::Tag::DoAction(items) => {
                efuckprint!("some {} actions\n", items.len());
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
                efuckprint!("I don't want sound stream block for now\n");
            },
            swf::Tag::SoundStreamHead(_sound_stream_head) => efuckprint!("ignoring sound at line {}\n", line!()),
            swf::Tag::SoundStreamHead2(sound_stream_head) => todo!(),
            swf::Tag::StartSound(start_sound) => {
                efuckprint!("no sound starting for now\n");
            },
            swf::Tag::StartSound2 { class_name, sound_info } => todo!(),
            swf::Tag::SymbolClass(symbol_class_links) => todo!(),
            swf::Tag::PlaceObject(place_object) => {
                match place_object.action {
                    swf::PlaceObjectAction::Place(id) => {
                        if display_list.len() <= place_object.depth as usize {
                            display_list.resize(place_object.depth as usize + 1, None);
                        }
                        display_list[place_object.depth as usize] = Some((id, place_object.matrix));
                    },
                    swf::PlaceObjectAction::Modify => {
                        display_list[place_object.depth as usize]
                        = Some((display_list[place_object.depth as usize].unwrap().0, place_object.matrix))
                    },
                    swf::PlaceObjectAction::Replace(id) => {
                        display_list[place_object.depth as usize] = Some((id, place_object.matrix));
                    },
                }
            },
            swf::Tag::RemoveObject(remove_object) => {
                display_list[remove_object.depth as usize] = None;
            },
            swf::Tag::VideoFrame(video_frame) => todo!(),
            swf::Tag::FileAttributes(file_attributes) => todo!(),
            swf::Tag::FrameLabel(frame_label) => {
                efuckprint!("frame label (IDK)\n");
            },
            swf::Tag::DefineSceneAndFrameLabelData(define_scene_and_frame_label_data) => todo!(),
            swf::Tag::ProductInfo(product_info) => todo!(),
            swf::Tag::Unknown { tag_code, data } => todo!(),
        }
    }

    //assert!(swf.header.num_frames() as usize == display_lists.len());

    fuckprint!("static plutovg_surface_t *s;\n");
    fuckprint!("static plutovg_canvas_t *c;\n\n");

    let mut i_want_to_kill_myself = 0;
    fuckprint!("#define OBJECT_XS \\\n");
    for s in shapes.iter() {
        fuckprint!(" X({},{}, \\\n", s.id, s.shape.len());
        for sr in s.shape.iter() {
            match sr {
                swf::ShapeRecord::StyleChange(style_change_data) => {
                    match style_change_data.move_to {
                        Some(to) => fuckprint!("  M({},{}) \\\n", to.x.to_pixels(), to.y.to_pixels()),
                        None => (),
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

    fuckprint!(r"
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
OBJECT_XS
#undef X

void there_she_is_init(void) {{
 s = plutovg_surface_create(there_she_is_width, there_she_is_height);
 c = plutovg_canvas_create(s);
 plutovg_canvas_set_line_width(c, 1);
 plutovg_canvas_set_rgb(c, 0,0,0);

 #define X(ID, COUNT, CTOR) \
  o##ID = plutovg_path_create(); \
  plutovg_path_reserve(o##ID, (COUNT));
 OBJECT_XS
 #undef X

 #define X(ID, COUNT, CTOR) o##ID##i,
 static void(*inits[])(void) = {{
  OBJECT_XS
 }};
 #undef X

 #pragma omp parallel for schedule(dynamic)
 for (int i = 0; i < (int)(sizeof inits / sizeof *inits); i++)
  inits[i]();
}}

void there_she_is_free(void) {{
 #define X(ID, COUNT, CTOR) plutovg_path_destroy(o##ID);
 OBJECT_XS
 #undef X
 plutovg_canvas_destroy(c);
 plutovg_surface_destroy(s);
}}");

    fuckprint!(r"

#define P(ID, A, B, C, D, TX, TY) \
 assert(o##ID != NULL); \
 plutovg_canvas_set_matrix(c, &PLUTOVG_MAKE_MATRIX(A, B, C, D, TX, TY)); \
 plutovg_canvas_stroke_path(c, o##ID)

void there_she_is_render(void *pixels, int pitch, int frame) {{
 assert(c != NULL);
 switch (frame) {{
");
    ignored.sort();
    for (i, dl) in display_lists.iter().enumerate() {
        fuckprint!("  case {}:\n", i);
        for d in dl.iter() {
            match d {
                Some((id, m)) => {
                    if !ignored.binary_search(id).is_ok() {
                        let m = m.unwrap_or(swf::Matrix::IDENTITY);
                        fuckprint!(
                            "   P({}, {}, {}, {}, {}, {}, {});\n",
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
        fuckprint!("  break;\n");
    }
    fuckprint!(r#"  default: assert(!"No such frame"); break;
 }}

 int stride = plutovg_surface_get_stride(s);
 unsigned char *data = plutovg_surface_get_data(s);
 if (stride == pitch)
  memcpy(pixels, data, 4 * there_she_is_width * there_she_is_height);
 else for (int i = 0; i < there_she_is_height; i++)
  memcpy(((unsigned char *)pixels) + pitch * i, data + stride * i, 4 * there_she_is_width);
 plutovg_surface_clear(s, &PLUTOVG_WHITE_COLOR);
}}
"#);

    efuckprint!("max x = {}\n", swf.header.stage_size().x_max.to_pixels());
    efuckprint!("max y = {}\n", swf.header.stage_size().y_max.to_pixels());
    efuckprint!("min x = {}\n", swf.header.stage_size().x_min.to_pixels());
    efuckprint!("min y = {}\n", swf.header.stage_size().y_min.to_pixels());
    efuckprint!("decompressed tags: {} bytes\n", swf_buf.data.len());
    fuckprint!("\nint const there_she_is_n_frame = {};\n", display_lists.len());
}
