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
    fuckprint!(r"
#include <assert.h>
#include <SDL2_gfxPrimitives.h>

#define O(ID) static void o##ID(SDL_Renderer *r, float a, float b, float c, float d, float tx, float ty)
#define F(ID) static void f##ID(SDL_Renderer *r)
#define V float x, y, new_x, new_y
#define E (void)r; (void)a; (void)b; (void)c; (void)d; (void)tx; (void)ty
#define C(ID) case ID: f##ID(r); break
#define L(DX, DY) \
    new_x = x + DX; new_y = y + DY; \
    aalineRGBA(r, NX(x,y), NY(x,y), NX(new_x,new_y), NY(new_x,new_y), 0, 0, 0, 1); \
    x = new_x; y = new_y
#define B(ADX, ADY, CDX, CDY) \
    new_x = x + ADX + CDX; new_y = y + ADY + CDY; \
    bezierRGBA( \
        r, \
        (Sint16[]){{NX(x,y), NX(x+CDX,y+CDY), NX(new_x,new_y)}}, \
        (Sint16[]){{NY(x,y), NY(x+CDX,y+CDY), NY(new_x,new_y)}}, \
        3, 300, 0, 0, 0, 1 \
    ); \
    x = new_x; y = new_y
#define NX(X, Y) ((X)*a + (Y)*c + tx) 
#define NY(X, Y) ((X)*b + (Y)*d + ty)

");
    let encoding = swf::SwfStr::encoding_for_version(swf.header.version());
    let mut display_list: Vec<Option<(u16, Option<swf::Matrix>)>> = Vec::new();
    let mut frame_i = 0;
    for tag in swf.tags {
        match tag {
            swf::Tag::ExportAssets(exported_assets) => todo!(),
            swf::Tag::ScriptLimits { max_recursion_depth, timeout_in_seconds } => todo!(),
            swf::Tag::ShowFrame => {
                fuckprint!("F({}) {{\n", frame_i);
                frame_i += 1;
                for d in display_list.iter() {
                    match d {
                        Some((id, m)) => {
                            let m = m.unwrap_or(swf::Matrix::IDENTITY);
                            fuckprint!(
                                "  o{}(r, {}, {}, {}, {}, {}, {});\n",
                                id,
                                m.a.to_f64(),
                                m.b.to_f64(),
                                m.c.to_f64(),
                                m.d.to_f64(),
                                m.tx.to_pixels(),
                                m.ty.to_pixels(),
                            );
                        },
                        None => (),
                    }
                }
                fuckprint!("}}\n\n");
            }
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
            swf::Tag::DefineButton2(button) => fuckprint!("O({}){{E;/*button*/}}\n", button.id),
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
            swf::Tag::DefineShape(shape) => {
                fuckprint!("O({}){{E;V;\n", shape.id);
                for sr in shape.shape {
                    match sr {
                        swf::ShapeRecord::StyleChange(style_change_data) => {
                            match style_change_data.move_to {
                                Some(to) => fuckprint!(" x = {}; y = {};\n", to.x.to_pixels(), to.y.to_pixels()),
                                None => (),
                            }
                        },
                        swf::ShapeRecord::StraightEdge { delta } => {
                            fuckprint!(" L({},{});\n", delta.dx.to_pixels(), delta.dy.to_pixels());
                        },
                        swf::ShapeRecord::CurvedEdge { control_delta, anchor_delta } => {
                            fuckprint!(
                                " B({},{},{},{});\n",
                                anchor_delta.dx.to_pixels(),
                                anchor_delta.dy.to_pixels(),
                                control_delta.dx.to_pixels(),
                                control_delta.dy.to_pixels()
                            );
                        },
                    }
                }
                fuckprint!("}}\n\n");
            },
            swf::Tag::DefineSound(sound) => {
                efuckprint!("no sound for now\n");
            },
            swf::Tag::DefineSprite(sprite) => {
                fuckprint!("O({}){{E;/*sprite with {} frames*/}}\n", sprite.id, sprite.num_frames)
            },
            swf::Tag::DefineText(text) => {
                fuckprint!("O({}){{E;/*text with {} records*/}}\n", text.id, text.records.len());
            },
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
                    swf::PlaceObjectAction::Modify => display_list[place_object.depth as usize]
                    = Some((display_list[place_object.depth as usize].unwrap().0, place_object.matrix)),
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
    fuckprint!("void there_she_is_render(SDL_Renderer *r, int frame) {{\n");
    fuckprint!(" switch (frame) {{");
    for i in 0..swf.header.num_frames() {
        fuckprint!("  C({});\n", i);
    }
    fuckprint!("  default: assert(!\"No such frame\"); break;\n");
    fuckprint!(" }}\n}}\n");

    fuckprint!("\nint const there_she_is_framerate = {};\n", swf.header.frame_rate());
    fuckprint!("\nint const there_she_is_n_frame = {};\n", swf.header.num_frames());
    fuckprint!("\nint const there_she_is_width = {};\n", (swf.header.stage_size().x_max - swf.header.stage_size().x_min).to_pixels());
    fuckprint!("\nint const there_she_is_height = {};\n", (swf.header.stage_size().y_max - swf.header.stage_size().y_min).to_pixels());
    efuckprint!("max x = {}\n", swf.header.stage_size().x_max.to_pixels());
    efuckprint!("max y = {}\n", swf.header.stage_size().y_max.to_pixels());
    efuckprint!("min x = {}\n", swf.header.stage_size().x_min.to_pixels());
    efuckprint!("min y = {}\n", swf.header.stage_size().y_min.to_pixels());
    efuckprint!("decompressed tags: {} bytes\n", swf_buf.data.len());
}
